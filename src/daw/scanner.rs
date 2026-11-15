use std::rc::Rc;
use std::thread;

use tracing::debug;
use tracing::trace;

use super::config::DawConfig;
use super::config::NormalizedConfig;
use super::config::normalize_process_name;
use super::status::DawStatus;
use crate::win32::handle::OwnedHandle;
use crate::win32::process;
use crate::win32::version;
use crate::win32::watcher;
use crate::win32::watcher::ExitChannel;
use crate::win32::window;

type Handle = windows_sys::Win32::Foundation::HANDLE;

struct TrackedProcess {
    pid: u32,
    handle: OwnedHandle,
    wait_handle: Handle,
    prev_kernel: u64,
    prev_user: u64,
    prev_wall: u64,
}

impl Drop for TrackedProcess {
    fn drop(&mut self) {
        watcher::unregister(self.wait_handle);
    }
}

struct AttachedDaw {
    config: Rc<NormalizedConfig>,
    processes: Vec<TrackedProcess>,
    version: Option<String>,
}

/// Two-phase DAW monitor:
/// 1. Discovery via process snapshot
/// 2. Per-PID metric polling
pub struct DawScanner {
    configs: Vec<Rc<NormalizedConfig>>,
    attached: Option<AttachedDaw>,
    exits: ExitChannel,
    cpu_count: usize,
}

impl DawScanner {
    pub fn new(configs: Vec<DawConfig>) -> Self {
        let normalized = NormalizedConfig::from_configs(configs);
        debug!("Loaded {} DAW configs", normalized.len());

        Self {
            configs: normalized,
            attached: None,
            exits: ExitChannel::new(),
            cpu_count: thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get),
        }
    }

    /// Returns current DAW status, or `None` if no DAW is running
    /// (transitions between discovery and monitoring automatically)
    pub fn poll(&mut self) -> Option<DawStatus> {
        self.handle_exits();

        if self.attached.is_none() {
            self.attached = self.discover();
        }

        let daw = self.attached.as_mut()?;
        Some(read_metrics(daw, self.cpu_count))
    }

    fn handle_exits(&mut self) {
        let dead = self.exits.drain();

        let Some(daw) = &mut self.attached else {
            return;
        };

        for pid in &dead {
            debug!("Process {pid} exited");
            daw.processes.retain(|process| process.pid != *pid);
        }

        if daw.processes.is_empty() {
            debug!("All DAW processes exited, returning to discovery");
            self.attached = None;
        }
    }

    fn discover(&self) -> Option<AttachedDaw> {
        if self.configs.is_empty() {
            return None;
        }

        // normalize once per entry, not once per (entry * config)
        let entries: Vec<(u32, String)> = process::snapshot()
            .iter()
            .map(|entry| (entry.pid, normalize_process_name(&entry.name)))
            .collect();

        self.configs.iter().find_map(|config| {
            let pids = entries
                .iter()
                .filter(|(_, name)| config.matches(name))
                .map(|(pid, _)| *pid);

            self.attach(config, pids)
        })
    }

    fn attach(&self, config: &Rc<NormalizedConfig>, pids: impl Iterator<Item = u32>) -> Option<AttachedDaw> {
        let mut processes = Vec::new();
        let mut version = None;

        for pid in pids {
            let Some(handle) = process::open(pid) else {
                trace!("Couldn't open PID {pid}, skipping");
                continue;
            };

            // try each process until we get a real version
            // some DAW processes don't embed version info

            if version.is_none() && !config.hide_version() {
                version = process::exe_path(handle.raw()).and_then(|path| version::exe_version(&path));
            }

            let wait_handle = self.exits.watch(pid, handle.raw());
            let (prev_kernel, prev_user) = process::cpu_times(handle.raw()).unwrap_or((0, 0));

            processes.push(TrackedProcess {
                pid,
                handle,
                wait_handle,
                prev_kernel,
                prev_user,
                prev_wall: process::wall_ticks(),
            });
        }

        if processes.is_empty() {
            return None;
        }

        debug!("Found {} ({} processes)", config.display_text(), processes.len());

        Some(AttachedDaw {
            config: Rc::clone(config),
            processes,
            version,
        })
    }
}

fn read_metrics(daw: &mut AttachedDaw, cpu_count: usize) -> DawStatus {
    let config = &daw.config;

    let mut total_cpu: f32 = 0.0;
    let mut total_memory: u64 = 0;
    let mut best_title = String::new();

    for process in &mut daw.processes {
        let handle = process.handle.raw();

        if let Some(bytes) = process::memory_bytes(handle) {
            total_memory += bytes / (1024 * 1024);
        }

        total_cpu += calculate_cpu_percent(process, cpu_count);

        let titles = window::window_titles(process.pid);

        // prefer title that matches the DAW's title regex
        // else fall back to longest (#46)

        let title = titles
            .iter()
            .find(|title| config.title().matches(title))
            .or_else(|| titles.iter().max_by_key(|title| title.len()))
            .cloned()
            .unwrap_or_default();

        if title.len() > best_title.len() {
            best_title = title;
        }
    }

    DawStatus {
        display_name: config.display_text().to_owned(),
        project: config.title().project(&best_title),
        cpu_usage: total_cpu,
        memory_mb: total_memory,
        version: daw.version.clone(),
        client_id: config.client_id().to_owned(),
    }
}

fn calculate_cpu_percent(process: &mut TrackedProcess, cpu_count: usize) -> f32 {
    let Some((kernel, user)) = process::cpu_times(process.handle.raw()) else {
        return 0.0;
    };

    let now = process::wall_ticks();
    let cpu_delta = kernel.saturating_sub(process.prev_kernel) + user.saturating_sub(process.prev_user);
    let wall_delta = now.saturating_sub(process.prev_wall);

    process.prev_kernel = kernel;
    process.prev_user = user;
    process.prev_wall = now;

    if wall_delta > 0 {
        (cpu_delta as f64 / wall_delta as f64 / cpu_count as f64 * 100.0) as f32
    } else {
        0.0
    }
}
