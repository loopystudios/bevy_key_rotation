use std::sync::Arc;

use bevy::prelude::*;
use bevy::state::app::StatesPlugin;

use crate::KeyRotationEvent;
use crate::data_types::AuthProvider;
use crate::data_types::KeyRotationSettings;
use crate::data_types::Keygen;
use crate::data_types::KeystoreState;
use crate::systems;

pub struct KeyRotationPlugin {
    pub rotation_settings: KeyRotationSettings,
    pub auth_provider: Arc<dyn AuthProvider + Send + Sync + 'static>,
}

impl Plugin for KeyRotationPlugin {
    fn build(&self, app: &mut App) {
        assert!(
            self.rotation_settings.rotation_check_interval < self.rotation_settings.rotate_before,
            "Invalid key rotation settings: rotation interval must be smaller than than time to rotate before expiration"
        );

        if !app.is_plugin_added::<StatesPlugin>() {
            app.add_plugins(StatesPlugin);
        }
        app.insert_resource(self.rotation_settings.clone())
            .insert_resource(Keygen(self.auth_provider.clone()))
            .init_state::<KeystoreState>()
            .add_event::<KeyRotationEvent>()
            .add_systems(
                Update,
                (systems::rotate_tokens, systems::state_transfer)
                    .chain()
                    .run_if(in_state(KeystoreState::Conformant)),
            );
    }
}
