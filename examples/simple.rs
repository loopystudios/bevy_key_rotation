use bevy::{log::LogPlugin, prelude::*};
use bevy_key_rotation::{
    AuthProvider, KeyRotationPlugin, KeyRotationSettings, Keystore, StartKeyRotationExt,
    TokenRotationError,
};
use std::sync::Arc;

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
            access_token: random_token(),
            refresh_token: random_token(),
            access_expires: bevy_key_rotation::Instant::now()
                + bevy_key_rotation::Duration::from_secs(10),
            refresh_expires: bevy_key_rotation::Instant::now()
                + bevy_key_rotation::Duration::from_secs(20),
        })
    }
    async fn refresh(&self, keystore: Keystore) -> Result<Keystore, TokenRotationError> {
        Ok(Keystore {
            username: keystore.username,
            password: keystore.password,
            access_token: random_token(),
            refresh_token: keystore.refresh_token,
            access_expires: keystore.access_expires + bevy_key_rotation::Duration::from_secs(5),
            refresh_expires: keystore.refresh_expires,
        })
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
    if !update_every.finished() {
        return;
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
                rotation_timeout: bevy_key_rotation::Duration::MAX, // no timeout
                rotation_check_interval: bevy_key_rotation::Duration::from_secs(1),
                rotate_before: bevy_key_rotation::Duration::from_secs(5),
            },
            auth_provider: Arc::new(MyAuthProvider),
        })
        .add_systems(Startup, |mut commands: Commands| {
            commands.start_key_rotation("username".to_string(), "password".to_string());
        })
        .add_systems(Update, status_check)
        .run();
}

fn random_token() -> String {
    let mut token = vec![0; 6];
    getrandom::fill(&mut token).unwrap();
    for byte in token.as_mut_slice() {
        *byte = (*byte % 26) + b'A'; // Convert to A-Z character
    }
    String::from_utf8(token).unwrap()
}
