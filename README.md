# RP1 HAL

This repository is the workspace for RP1 firmware HAL work. It currently
contains shared ABI definitions and does not yet contain a full HAL
implementation.

## `rp1-abi`

`crates/rp1-abi` defines the initial fixed `.note.rp1` ABI shared by RP1
firmware images and the CM5 RP1 bootloader PoC. It is `#![no_std]` and currently
contains only the boot note layout and owner bitmap device constants.

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

The standard build command is:

```sh
cargo build --target thumbv7m-none-eabi
```

The shell also provides a convenience wrapper:

```sh
build-rp1-hal
```

`build-rp1-hal` runs the same target build through the Nix-provided toolchain.
