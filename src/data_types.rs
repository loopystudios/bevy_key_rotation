use crate::{Duration, Instant, error::TokenRotationError};
use async_trait::async_trait;
use bevy::prelude::*;
use std::sync::Arc;

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait AuthProvider {
    async fn authenticate(
        &self,
        username: String,
        password: String,
    ) -> Result<Keystore, TokenRotationError>;
    async fn refresh(&self, keystore: Keystore) -> Result<Keystore, TokenRotationError>;
}

/// A resource around the auth provider used (mostly internally) to perform
/// auth.
#[derive(Resource)]
pub struct Keygen(pub Arc<dyn AuthProvider + Send + Sync + 'static>);

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, States)]
pub enum KeystoreState {
    /// The keystore is valid.
    Conformant,
    #[default]
    // The keystore is expired and cannot update.
    NonConformant,
}

#[derive(Resource, Debug, Clone)]
pub struct Keystore {
    /// The latest username
    pub username: String,

    /// The latest password
    pub password: String,

    /// The latest access token
    pub access_token: String,

    /// The latest refresh token
    pub refresh_token: String,

    /// The instant when the access token expires
    pub access_expires: Instant,

    /// The instant when the refresh token expires
    pub refresh_expires: Instant,
}

impl Keystore {
    /// The amount of time the access token is valid for. A duration of zero
    /// means expired.
    pub fn access_token_valid_for(&self) -> Duration {
        self.access_expires
            .saturating_duration_since(web_time::Instant::now())
    }

    /// The amount of time the refresh token is valid for. A duration of zero
    /// means expired.
    pub fn refresh_token_valid_for(&self) -> Duration {
        self.refresh_expires
            .saturating_duration_since(web_time::Instant::now())
    }
}

#[derive(Resource, Debug, Clone)]
pub struct KeyRotationSettings {
    /// The amount of time before the rotation attempt times out
    pub rotation_timeout: Duration,

    /// The interval to check for rotation
    pub rotation_check_interval: Duration,

    /// The amount of time to begin rotation before expiration
    pub rotate_before: Duration,
}

impl Default for KeyRotationSettings {
    fn default() -> Self {
        // Default settings:
        // - Rotation attempt timeout is 10 seconds
        // - Re-attempt rotation, if necessary, every 60 seconds
        // - Begin to attempt key rotation 5 minutes before expiration
        Self {
            rotation_timeout: Duration::from_secs(10),
            rotation_check_interval: Duration::from_secs(60),
            rotate_before: Duration::from_secs(60 * 5), // 5 min
        }
    }
}

/// An event triggered for important key rotation events.
#[derive(Event)]
pub enum KeyRotationEvent {
    /// Key rotation has started
    Started(Keystore),
    /// Keys were rotated successfully
    Rotated(Keystore),
    /// Tokens failed to rotate
    FailedRotation(TokenRotationError),
    /// Key rotation became non-conformant
    Stopped,
}
