use iced::Task;
use iced::window;
use tracing::debug;

use super::AppState;
use super::IntervalDraft;
use super::Message;
use super::Page;
use super::save_or_warn;
use crate::discord::DiscordSender;
use crate::discord::PresenceRequest;
use crate::settings::AppSettings;
use crate::ui::tray::TrayUpdate;
use crate::ui::tray::send_tray_update;
use crate::win32::autostart;

const INTERVAL_PARSE_ERROR: &str = "Interval must be a number";

fn remember_hidden(state: &mut AppState, is_hidden: bool) {
    if state.settings.was_hidden == is_hidden {
        return;
    }

    state.settings.was_hidden = is_hidden;
    save_or_warn(&state.settings);
}

pub(super) fn close_requested(state: &mut AppState, window_id: window::Id) -> Task<Message> {
    if !state.settings.close_to_tray {
        return window::close(window_id);
    }

    remember_hidden(state, true);
    window::set_mode(window_id, window::Mode::Hidden)
}

pub(super) fn window_opened(state: &AppState, window_id: window::Id) -> Task<Message> {
    if state.start_hidden {
        return window::set_mode(window_id, window::Mode::Hidden);
    }

    Task::none()
}

pub(super) fn drag_window() -> Task<Message> {
    window::latest().and_then(window::drag)
}

pub(super) fn minimize_window() -> Task<Message> {
    window::latest().and_then(|id| window::minimize(id, true))
}

pub(super) fn close_window() -> Task<Message> {
    window::latest().and_then(|id| Task::done(Message::CloseRequested(id)))
}

pub(super) fn tray_show(state: &mut AppState) -> Task<Message> {
    remember_hidden(state, false);
    window::latest().and_then(|id| {
        Task::batch(vec![
            window::set_mode(id, window::Mode::Windowed),
            window::gain_focus(id),
        ])
    })
}

pub(super) fn tray_quit() -> Task<Message> {
    window::latest().and_then(window::close)
}

pub(super) fn navigate_to(state: &mut AppState, page: Page) -> Task<Message> {
    state.active_page = page;
    debug!("Navigated to {page:?}");

    if page == Page::Settings {
        state.interval_input = state.settings.update_interval.to_string();
        state.interval_draft = IntervalDraft::Unchanged;
    }

    Task::none()
}

pub(super) fn toggle_auto_start(state: &mut AppState, value: bool) -> Task<Message> {
    autostart::set_enabled(value);
    // re-read registry to confirm write succeeded
    state.auto_start_enabled = autostart::is_enabled();
    debug!("Auto-start toggled: {}", state.auto_start_enabled);
    Task::none()
}

pub(super) fn toggle_close_to_tray(state: &mut AppState, value: bool) -> Task<Message> {
    state.settings.close_to_tray = value;
    debug!("Close to tray toggled: {value}");
    save_or_warn(&state.settings);
    Task::none()
}

pub(super) fn toggle_hide_project_name(state: &mut AppState, value: bool) -> Task<Message> {
    state.settings.hide_project_name = value;
    debug!("Hide project name toggled: {value}");
    save_or_warn(&state.settings);
    send_tray_update(TrayUpdate::HideProjectName(value));
    Task::none()
}

pub(super) fn toggle_hide_system_usage(state: &mut AppState, value: bool) -> Task<Message> {
    state.settings.hide_system_usage = value;
    debug!("Hide system usage toggled: {value}");
    save_or_warn(&state.settings);
    send_tray_update(TrayUpdate::HideSystemUsage(value));
    Task::none()
}

pub(super) fn update_interval_input(state: &mut AppState, value: &str) -> Task<Message> {
    value.clone_into(&mut state.interval_input);

    state.interval_draft = match value.parse::<u64>() {
        Ok(interval) if interval == state.settings.update_interval => IntervalDraft::Unchanged,
        Ok(interval) => match AppSettings::validate_update_interval(interval) {
            Ok(()) => IntervalDraft::Valid(interval),
            Err(error) => IntervalDraft::Invalid(error.to_string()),
        },
        Err(_) => IntervalDraft::Invalid(INTERVAL_PARSE_ERROR.to_owned()),
    };

    Task::none()
}

pub(super) fn apply_interval(state: &mut AppState) -> Task<Message> {
    let IntervalDraft::Valid(interval) = state.interval_draft else {
        return Task::none();
    };

    state.settings.update_interval = interval;
    state.interval_draft = IntervalDraft::Applied;
    debug!("Update interval applied: {interval}ms");
    save_or_warn(&state.settings);
    Task::none()
}

pub(super) fn tick(state: &mut AppState) -> Task<Message> {
    if state.interval_draft == IntervalDraft::Applied {
        state.interval_draft = IntervalDraft::Unchanged;
    }

    state.daw_status = state.daw_scanner.poll();

    let Some(discord) = &state.discord else {
        return Task::none();
    };

    let request = state
        .daw_status
        .as_ref()
        .map(|status| PresenceRequest::from_daw_status(status, &state.settings));

    if !discord.send(request) {
        state.discord = None;
    }

    Task::none()
}

pub(super) fn discord_ready(state: &mut AppState, sender: DiscordSender) -> Task<Message> {
    state.discord = Some(sender);
    debug!("Discord worker ready");
    Task::none()
}

pub(super) fn discord_connected(state: &mut AppState, connected: bool) -> Task<Message> {
    state.discord_connected = connected;
    debug!("Discord connection state: {connected}");
    send_tray_update(TrayUpdate::DiscordConnected(connected));
    Task::none()
}
