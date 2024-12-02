# Bevy Key Rotation

![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
[![crates.io](https://img.shields.io/crates/v/bevy_key_rotation.svg)](https://crates.io/crates/bevy_key_rotation)
[![docs.rs](https://img.shields.io/docsrs/bevy_key_rotation)](https://docs.rs/bevy_key_rotation)

A minimum crate for non-blocking, continuous use of an access token, by ensuring it is constantly rotated ahead-of-time via refresh token. When a refresh token needs to be rotated, it is rotated with username/password credentials.

There is full API support for **wasm** and **native**. Android and iOS are untested (Help needed).

## Bevy version support

|bevy|bevy_key_rotation|
|---|---|
|0.15|0.3, main|
|0.14|0.2|
|0.13|0.1|
|< 0.13|Unsupported|

## Usage

Please see [examples](examples/) for more.

## License

This project is dual-licensed under both [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT) licenses.
