use std::sync::Arc;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_key_rotation::AuthProvider;
use bevy_key_rotation::KeyRotationPlugin;
use bevy_key_rotation::KeyRotationSettings;
use bevy_key_rotation::Keystore;
use bevy_key_rotation::KeystoreState;
use bevy_key_rotation::StartKeyRotationExt;
use bevy_key_rotation::TokenRotationError;

pub struct MyAuthProvider;

#[cfg_attr(not(target_arch = "wasm32"), bevy_key_rotation::async_trait)]
#[cfg_attr(target_arch = "wasm32", bevy_key_rotation::async_trait(?Send))]
impl AuthProvider for MyAuthProvider {
    async fn authenticate(
        &self,
        username: String,
        password: String,
    ) -> Result<Keystore, TokenRotationError> {
        Ok(Keystore {
            username,
            password,
            access_token: "123".to_string(),
            refresh_token: "456".to_string(),
            access_expires: bevy_key_rotation::Instant::now()
                + bevy_key_rotation::Duration::from_secs(20),
            refresh_expires: bevy_key_rotation::Instant::now()
                + bevy_key_rotation::Duration::from_secs(60),
        })
    }
    async fn refresh(&self, _keystore: Keystore) -> Result<Keystore, TokenRotationError> {
        #[derive(thiserror::Error, Default, Debug)]
        #[error("This fails on purpose!")]
        struct MyError;
        Err(TokenRotationError::new(MyError))
    }
}

fn status_check(time: Res<Time>, mut update_every: Local<Option<Timer>>, keystore: Res<Keystore>) {
    // Print status every few seconds...
    const PRINT_EVERY_SECONDS: f32 = 1.0;
    let update_every = update_every.get_or_insert(Timer::from_seconds(
        PRINT_EVERY_SECONDS,
        TimerMode::Repeating,
    ));
    update_every.tick(time.delta());
    if !update_every.is_finished() {
        return;
    }

    if keystore.access_token_valid_for() < bevy_key_rotation::Duration::from_secs(5) {
        warn!("The keystore is about to be non-conformant!");
        // You could attempt to re-authenticate from scratch:
        // commands.start_key_rotation(username, password);
        // Or panic, or safe your system and prepare to exit, etc.
    }

    // Log current access token
    info!(
        token = keystore.access_token,
        refresh_token = keystore.refresh_token,
        "token valid for: {:.0?}, refresh token valid for: {:.0?}",
        keystore.access_token_valid_for(),
        keystore.refresh_token_valid_for(),
    );
}

pub fn main() {
    App::new()
        .add_plugins((MinimalPlugins, LogPlugin::default()))
        .add_plugins(KeyRotationPlugin {
            rotation_settings: KeyRotationSettings {
                rotation_timeout: bevy_async_task::MAX_TIMEOUT, // no timeout
                rotation_check_interval: bevy_key_rotation::Duration::from_secs(5),
                rotate_before: bevy_key_rotation::Duration::from_secs(15),
            },
            auth_provider: Arc::new(MyAuthProvider),
        })
        .add_systems(Startup, |mut commands: Commands| {
            commands.start_key_rotation("username".to_string(), "password".to_string());
        })
        .add_systems(
            Update,
            status_check.run_if(in_state(KeystoreState::Conformant)),
        )
        .add_systems(
            OnTransition {
                exited: KeystoreState::Conformant,
                entered: KeystoreState::NonConformant,
            },
            || {
                error!("Keystore is now non-conformant! Keys cannot be rotated.");
            },
        )
        .run();
}
