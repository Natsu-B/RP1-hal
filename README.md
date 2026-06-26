# RP1 HAL

This repository is the workspace for future RP1 firmware HAL work. It does not
yet contain a Cargo workspace or HAL implementation.

## Development Shell

Enter the Nix shell before building RP1 firmware code:

```sh
nix develop
```

The shell provides:

- nightly Rust
- `rust-src`
- `llvm-tools-preview`
- `thumbv7m-none-eabi`
- `cargo-binutils`
- `llvm-objcopy` / `rust-objcopy`
- `readelf`
- `xxd`
- `dtc`

Once a Cargo workspace exists, the standard build command is:

```sh
cargo build --target thumbv7m-none-eabi
```

The shell also provides a convenience wrapper:

```sh
build-rp1-hal
```

Until `Cargo.toml` is added, `build-rp1-hal` intentionally exits with an
explanation instead of pretending a firmware build happened.
