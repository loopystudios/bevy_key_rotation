use async_trait::async_trait;
use bevy::{
    log::{self, LogPlugin},
    prelude::*,
};
use bevy_key_rotation::{
    AuthProvider, KeyRotationPlugin, KeyRotationSettings, Keystore,
    TokenRotationError,
};
use std::{sync::Arc, time::Duration};

pub struct MyAuthProvider;

#[async_trait]
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
                + Duration::from_secs(30),
            refresh_expires: bevy_key_rotation::Instant::now()
                + Duration::from_secs(60),
        })
    }
    async fn refresh(
        &self,
        keystore: Keystore,
    ) -> Result<Keystore, TokenRotationError> {
        Ok(Keystore {
            username: keystore.username,
            password: keystore.password,
            access_token: "789".to_string(),
            refresh_token: keystore.refresh_token,
            access_expires: keystore.access_expires + Duration::from_secs(30),
            refresh_expires: keystore.refresh_expires,
        })
    }
}

fn status_check(
    time: Res<Time>,
    mut update_every: Local<Option<Timer>>,
    keystore: Res<Keystore>,
) {
    // Print status every 2s
    let update_every = update_every
        .get_or_insert(Timer::from_seconds(2.0, TimerMode::Repeating));
    update_every.tick(time.delta());
    if !update_every.just_finished() {
        return;
    }

    // Log current access token
    log::info!("{:?}", *keystore);
}

pub fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin::default())
        .add_plugins(KeyRotationPlugin {
            username: "username".to_string(),
            password: "password".to_string(),
            rotation_settings: KeyRotationSettings {
                access_valid_time: bevy_key_rotation::Duration::from_secs(5),
                refresh_valid_time: bevy_key_rotation::Duration::from_secs(10),
                rotation_timeout: bevy_key_rotation::Duration::from_secs(1),
                rotation_check_interval: bevy_key_rotation::Duration::from_secs(
                    1,
                ),
                rotate_before: bevy_key_rotation::Duration::from_secs(3),
            },
            auth_provider: Arc::new(MyAuthProvider),
        })
        .add_systems(Update, status_check)
        .run();
}
