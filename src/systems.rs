use crate::{
    data_types::{KeyRotationSettings, Keygen, Keystore},
    error::TokenRotationError,
    KeystoreState,
};
use bevy::prelude::*;
use bevy_async_task::{
    AsyncTask, AsyncTaskRunner, AsyncTaskStatus, TimeoutError,
};
use instant::Duration;

pub(crate) fn rotate_tokens(
    keygen: Res<Keygen>,
    settings: Res<KeyRotationSettings>,
    mut keystore: ResMut<Keystore>,
    mut tr_rotate: AsyncTaskRunner<
        Result<Result<Keystore, TokenRotationError>, TimeoutError>,
    >,
    mut rotation_timer: Local<Option<Timer>>,
    time: Res<Time>,
) {
    if let AsyncTaskStatus::Finished(resp) = tr_rotate.poll() {
        match resp {
            Ok(Ok(keys)) => {
                *keystore = keys;
                // todo send event
                info!("token rotation successful");
            }
            err @ (Err(_) | Ok(Err(_))) => {
                if let Err(_timeout) = err {
                    warn!(
                        "key rotation timed out after {:?}",
                        settings.rotation_check_interval
                    );
                } else if let Ok(Err(e)) = err {
                    warn!("key rotation failed: {e}");
                }
                // Todo send event for warnings
            }
        }
    }

    let rotation_timer = rotation_timer.get_or_insert(Timer::new(
        settings.rotation_check_interval,
        TimerMode::Once,
    ));
    rotation_timer.tick(time.delta());
    if !rotation_timer.finished() {
        return;
    }
    rotation_timer.reset();

    // Check if rotation is necessary
    let rtoken_expiring =
        keystore.refresh_token_valid_for() < settings.rotate_before;
    let atoken_expiring =
        keystore.access_token_valid_for() < settings.rotate_before;

    if rtoken_expiring {
        info!("rotating refresh token...");
        let task = AsyncTask::new({
            let username = keystore.username.clone();
            let password = keystore.password.clone();
            let auth_provider = keygen.0.clone();
            async move { auth_provider.authenticate(username, password).await }
        })
        .with_timeout(settings.rotation_timeout);
        tr_rotate.start(task);
    } else if atoken_expiring {
        info!("rotating access token...");
        let task = AsyncTask::new({
            let keystore = (*keystore).clone();
            let auth_provider = keygen.0.clone();
            async move { auth_provider.refresh(keystore).await }
        })
        .with_timeout(settings.rotation_timeout);
        tr_rotate.start(task);
    }
}

pub fn state_transfer(
    mut token_state: ResMut<NextState<KeystoreState>>,
    keystore: Res<Keystore>,
) {
    if keystore.access_token_valid_for() == Duration::ZERO {
        token_state.set(KeystoreState::NonConformant);
    }
}
