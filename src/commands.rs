use crate::{data_types::Keygen, Keystore, KeystoreState};
use bevy::{ecs::system::Command, prelude::*};
use bevy_async_task::AsyncTask;

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
        if keystore.access_token_valid_for() > crate::Duration::ZERO {
            let mut state = world.resource_mut::<NextState<KeystoreState>>();
            state.set(KeystoreState::Conformant);
        } else {
            warn!("auth provider authenticated, but returned an expired access token, remaining nonconformant");
        }
        world.insert_resource(keystore);
    }
}

struct StartKeyRotationWithKeystore {
    keystore: Keystore,
}

impl Command for StartKeyRotationWithKeystore {
    fn apply(self, world: &mut bevy::prelude::World) {
        info!("starting key rotation...");
        let keystore = self.keystore.clone();

        if keystore.access_token_valid_for() > crate::Duration::ZERO {
            let mut state = world.resource_mut::<NextState<KeystoreState>>();
            state.set(KeystoreState::Conformant);
        } else {
            warn!("started key rotation with an expired keystore, remaining nonconformant");
        }
        world.insert_resource(keystore);
    }
}

/// A [`Commands`] extension used to start key rotation.
pub trait StartKeyRotationExt {
    /// Start (and block) the key rotation by authenticating with the auth
    /// provider.
    fn start_key_rotation(&mut self, username: String, password: String);
    /// Start the key rotation immediately with a keystore.
    fn start_key_rotation_with_keystore(&mut self, keystore: Keystore);
}

impl<'w, 's> StartKeyRotationExt for Commands<'w, 's> {
    fn start_key_rotation(&mut self, username: String, password: String) {
        self.add(StartKeyRotation { username, password })
    }
    fn start_key_rotation_with_keystore(&mut self, keystore: Keystore) {
        self.add(StartKeyRotationWithKeystore { keystore })
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
