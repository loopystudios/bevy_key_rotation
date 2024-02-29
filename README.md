# Bevy Key Rotation

![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
[![crates.io](https://img.shields.io/crates/v/bevy-key-rotation.svg)](https://crates.io/crates/bevy-key-rotation)
[![docs.rs](https://img.shields.io/docsrs/bevy-key-rotation)](https://docs.rs/bevy-key-rotation)

A minimum crate for non-blocking, continuous use of an access token, by ensuring it is constantly rotated ahead-of-time via refresh token. When a refresh token needs to be rotated, it is rotated with username/password credentials.

There is full API support for **wasm** and **native**. Android and iOS are untested (Help needed).

## Bevy version support

|bevy|bevy-key-rotation|
|---|---|
|0.13|1.5, main|
|0.12|1.4|
|0.11|1.3|
|<= 0.10|Unsupported|

## Usage

Please see [examples](examples/) for more.

## License

This project is dual-licensed under both [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT) licenses.
