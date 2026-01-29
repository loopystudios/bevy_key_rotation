# Bevy Key Rotation

![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
[![crates.io](https://img.shields.io/crates/v/bevy_key_rotation.svg)](https://crates.io/crates/bevy_key_rotation)
[![docs.rs](https://img.shields.io/docsrs/bevy_key_rotation)](https://docs.rs/bevy_key_rotation)
[![Following released Bevy versions](https://img.shields.io/badge/bevy%20tracking-released%20version-lightblue)](https://bevyengine.org/learn/quick-start/plugin-development/#main-branch-tracking)

A minimum crate for non-blocking, continuous use of an access token, by ensuring it is constantly rotated ahead-of-time via refresh token. When a refresh token needs to be rotated, it is rotated with username/password credentials.

There is full API support for **wasm** and **native**. Android and iOS are untested (Help needed).

## Bevy version support

|bevy|bevy_key_rotation|
|---|---|
|0.18|0.7, main|
|0.17|0.6|
|0.16|0.5|
|0.15|0.3-0.4|
|0.14|0.2|
|0.13|0.1|
|< 0.13|Unsupported|

## Usage

There are several [examples](examples/) for reference.

You can also run examples on web:

```shell
# Make sure the Rust toolchain supports the wasm32 target
rustup target add wasm32-unknown-unknown

cargo run_wasm --example simple
```

## Community

All Loopy projects and development happens in the [Loopy Discord](https://discord.gg/KSfKceUKde). The discord is open to the public.

Contributions are welcome by pull request. The [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct) applies.

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
