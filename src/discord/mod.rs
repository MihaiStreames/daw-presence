mod presence;
mod worker;

use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use discord_rich_presence::DiscordIpc as _;
use discord_rich_presence::DiscordIpcClient;
use discord_rich_presence::activity;
use presence::DiscordPresence;
pub use presence::PresenceRequest;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::trace;
use tracing::warn;
pub use worker::DiscordSender;
pub use worker::discord_subscription;

use crate::error::DiscordError;

struct Connection {
    client: DiscordIpcClient,
    client_id: String,
    started_at: i64,
}

impl Connection {
    fn open(client_id: &str) -> Result<Self, DiscordError> {
        let mut client = DiscordIpcClient::new(client_id);
        client.connect().map_err(|e| DiscordError::Connect(e.to_string()))?;

        info!("Connected to Discord RPC");

        Ok(Self {
            client,
            client_id: client_id.to_owned(),
            started_at: current_timestamp(),
        })
    }

    fn set_presence(mut self, presence: &DiscordPresence) -> (Option<Self>, Result<(), DiscordError>) {
        let build_activity = || {
            activity::Activity::new()
                .details(&presence.details)
                .state(&presence.state)
                .assets(
                    activity::Assets::new()
                        .large_image(&presence.large_image)
                        .large_text(&presence.large_text),
                )
                .timestamps(activity::Timestamps::new().start(self.started_at))
        };

        let Err(activity_error) = self.client.set_activity(build_activity()) else {
            return (Some(self), Ok(()));
        };

        // retry once on connection failure
        warn!("Couldn't set activity: {activity_error}, trying to reconnect...");

        if let Err(reconnect_error) = self.client.reconnect() {
            error!("Couldn't reconnect: {reconnect_error}");

            let _ = self.client.close();

            return (
                None,
                Err(DiscordError::Reconnect {
                    activity: activity_error.to_string(),
                    reconnect: reconnect_error.to_string(),
                }),
            );
        }

        let result = self
            .client
            .set_activity(build_activity())
            .map(|()| info!("Reconnected to Discord RPC"))
            .map_err(|e| DiscordError::Activity(e.to_string()));

        (Some(self), result)
    }

    fn close(mut self) {
        let _ = self.client.clear_activity();
        let _ = self.client.close();

        debug!("Disconnected from Discord RPC");
    }
}

struct DiscordManager {
    connection: Option<Connection>,
}

impl DiscordManager {
    const fn new() -> Self {
        Self { connection: None }
    }

    const fn is_connected(&self) -> bool {
        self.connection.is_some()
    }

    fn disconnect(&mut self) {
        if let Some(connection) = self.connection.take() {
            connection.close();
        }
    }

    fn apply_request(&mut self, request: Option<&PresenceRequest>) -> Result<(), DiscordError> {
        let Some(request) = request else {
            self.disconnect();
            return Ok(());
        };

        let connection = match self.connection.take() {
            Some(connection) if connection.client_id == request.client_id => connection,
            stale => {
                if let Some(connection) = stale {
                    debug!("Client ID changed, reconnecting...");
                    connection.close();
                }

                Connection::open(&request.client_id)?
            }
        };

        let (connection, result) = connection.set_presence(&request.presence);
        self.connection = connection;
        result?;

        trace!("Presence updated: {}", request.presence.details);

        Ok(())
    }
}

fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs()
        .cast_signed()
}
