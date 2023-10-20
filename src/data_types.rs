use std::sync::Arc;

use crate::error::TokenRotationError;
use async_trait::async_trait;
use bevy::prelude::*;
use instant::{Duration, Instant};

#[async_trait]
pub trait AuthProvider {
    async fn authenticate(
        &self,
        username: String,
        password: String,
    ) -> Result<Keystore, TokenRotationError>;
    async fn refresh(
        &self,
        keystore: Keystore,
    ) -> Result<Keystore, TokenRotationError>;
}

/// A wrapper around the auth provider used internally to perform auth.
#[derive(Resource)]
pub(crate) struct Keygen(pub Arc<dyn AuthProvider + Send + Sync + 'static>);

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, States)]
pub enum KeystoreState {
    #[default]
    /// The keystore is valid.
    Conformant,
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

#[derive(Resource, Debug, Clone)]
pub struct KeyRotationSettings {
    /// The amount of time an access token is valid for
    pub access_valid_time: Duration,

    /// The amount of time a refresh token is valid for
    pub refresh_valid_time: Duration,

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
        // - Access token valid for 1 hour
        // - Refresh token valid for 30 days
        // - Rotation attempt timeout is 10 seconds
        // - Re-attempt rotation, if necessary, every 60 seconds
        // - Begin to attempt key rotation 5 minutes before expiration
        Self {
            access_valid_time: Duration::from_secs(60 * 60), // 1 hour
            refresh_valid_time: Duration::from_secs(60 * 60 * 24 * 30), /* 30 days */
            rotation_timeout: Duration::from_secs(10),
            rotation_check_interval: Duration::from_secs(60),
            rotate_before: Duration::from_secs(60 * 5), // 5 min
        }
    }
}
