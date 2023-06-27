# emp-rust
Efficient Multi-Party computation toolkit in Rust.

## Platforms
emp-rust supports `x86/x86_64` and `aarch64` architectures. To enable AES intrinsics on `aarch64` CPUs, it is recommended to use the nightly version of Rust.
## Documentation
run

> `cargo doc --open --no-deps`

## Examples
run in one terminal:

> `cargo run --release --example netio -- --party 1`

run in another terminal:

> `cargo run --release --example netio -- --party 2`