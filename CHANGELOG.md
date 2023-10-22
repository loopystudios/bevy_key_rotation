# Changelog

This changelog follows the patterns described here: <https://keepachangelog.com/en/1.0.0/>.

Subheadings to categorize changes are `added, changed, deprecated, removed, fixed, security`.

## v1.3.0

### changed

- `Kegen` resource is now public

## v1.2.0

### added

- `commands.start_key_rotation_with_keystore(keystore)` begins key rotation without blocking

### fixed

- On initial authentication, expired tokens no longer grant a `KeystoreState::Conformant` state

## v1.1.0

### added

- `commands.start_key_rotation(username, password)` begins key rotation
- `commands.stop_key_rotation()` stops key rotation

### changed

- `KeystoreState` now starts as `KeystoreState::NonConformant` by default

## v1.0.0

Initial release
