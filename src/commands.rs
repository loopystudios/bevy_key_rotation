use bevy::{ecs::system::Command, prelude::*};
use bevy_async_task::AsyncTask;

use crate::{data_types::Keygen, KeystoreState};

struct StartKeyRotation {
    username: String,
    password: String,
}

impl Command for StartKeyRotation {
    fn apply(self, world: &mut bevy::prelude::World) {
        info!("starting key rotation, authenticating credentials...");
        let keygen = world.resource::<Keygen>();
        let keystore = AsyncTask::new({
            let username = self.username.clone();
            let password = self.password.clone();
            let auth_provider = keygen.0.clone();
            async move { auth_provider.authenticate(username, password).await }
        })
        .blocking_recv()
        .unwrap();
        info!("credentials authenticated!");
        world.insert_resource(keystore);
        let mut state = world.resource_mut::<NextState<KeystoreState>>();
        state.set(KeystoreState::Conformant);
    }
}

/// A [`Commands`] extension used to start key rotation.
pub trait StartKeyRotationExt {
    fn start_key_rotation(&mut self, username: String, password: String);
}

impl<'w, 's> StartKeyRotationExt for Commands<'w, 's> {
    fn start_key_rotation(&mut self, username: String, password: String) {
        self.add(StartKeyRotation { username, password })
    }
}

struct StopKeyRotation;

impl Command for StopKeyRotation {
    fn apply(self, world: &mut bevy::prelude::World) {
        let mut state = world.resource_mut::<NextState<KeystoreState>>();
        state.set(KeystoreState::NonConformant);
        info!("stopping key rotation");
    }
}

/// A [`Commands`] extension used to stop key rotation.
pub trait StopKeyRotationExt {
    fn stop_key_rotation(&mut self);
}

impl<'w, 's> StopKeyRotationExt for Commands<'w, 's> {
    fn stop_key_rotation(&mut self) {
        self.add(StopKeyRotation);
    }
}
