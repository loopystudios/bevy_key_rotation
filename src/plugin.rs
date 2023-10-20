use crate::{
    data_types::{AuthProvider, KeyRotationSettings, Keygen, KeystoreState},
    systems,
};
use bevy::prelude::*;
use bevy_async_task::AsyncTask;
use std::sync::Arc;

pub struct KeyRotationPlugin {
    pub username: String,
    pub password: String,
    pub rotation_settings: KeyRotationSettings,
    pub auth_provider: Arc<dyn AuthProvider + Send + Sync + 'static>,
}

impl Plugin for KeyRotationPlugin {
    fn build(&self, app: &mut App) {
        info!("checking settings...");
        assert!(
            self.rotation_settings.rotation_check_interval < self.rotation_settings.rotate_before,
            "Invalid key rotation settings: rotation interval must be smaller than than time to rotate before expiration"
        );

        info!("authenticating credentials...");
        let credentials = AsyncTask::new({
            let username = self.username.clone();
            let password = self.password.clone();
            let auth_provider = self.auth_provider.clone();
            async move { auth_provider.authenticate(username, password).await }
        })
        .blocking_recv()
        .unwrap();
        info!("credentials authenticated!");
        app.insert_resource(credentials)
            .insert_resource(self.rotation_settings.clone())
            .insert_resource(Keygen(self.auth_provider.clone()))
            .add_state::<KeystoreState>()
            .add_systems(
                Update,
                systems::rotate_tokens, // .run_if(state_exists_and_equals(KeystoreState::Conformant)),
            );
    }
}
