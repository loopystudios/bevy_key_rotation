use crate::{data_types::Keygen, KeyRotationEvent, Keystore, KeystoreState};
use bevy::prelude::*;

struct StartKeyRotation {
    username: String,
    password: String,
}

impl Command for StartKeyRotation {
    fn apply(self, world: &mut bevy::prelude::World) {
        info!("starting key rotation, authenticating credentials...");
        let keygen = world.resource::<Keygen>();

        let keystore = bevy::tasks::block_on({
            let username = self.username.clone();
            let password = self.password.clone();
            let auth_provider = keygen.0.clone();
            async move { auth_provider.authenticate(username, password).await }
        })
        .unwrap();
        info!("credentials authenticated!");
        if keystore.access_token_valid_for() > crate::Duration::ZERO {
            let mut state = world.resource_mut::<NextState<KeystoreState>>();
            state.set(KeystoreState::Conformant);
            world.send_event(KeyRotationEvent::Started(keystore.clone()));
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
            world.send_event(KeyRotationEvent::Started(self.keystore.clone()));
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

impl StartKeyRotationExt for Commands<'_, '_> {
    fn start_key_rotation(&mut self, username: String, password: String) {
        self.queue(StartKeyRotation { username, password })
    }
    fn start_key_rotation_with_keystore(&mut self, keystore: Keystore) {
        self.queue(StartKeyRotationWithKeystore { keystore })
    }
}

struct StopKeyRotation;

impl Command for StopKeyRotation {
    fn apply(self, world: &mut bevy::prelude::World) {
        let mut state = world.resource_mut::<NextState<KeystoreState>>();
        state.set(KeystoreState::NonConformant);
        world.send_event(KeyRotationEvent::Stopped);
        info!("stopping key rotation");
    }
}

/// A [`Commands`] extension used to stop key rotation.
pub trait StopKeyRotationExt {
    fn stop_key_rotation(&mut self);
}

impl StopKeyRotationExt for Commands<'_, '_> {
    fn stop_key_rotation(&mut self) {
        self.queue(StopKeyRotation);
    }
}
