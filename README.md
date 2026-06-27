# RP1 HAL

This repository is the workspace for RP1 firmware HAL work. It contains the
shared ABI definitions, a minimal runtime, a small HAL facade, entry-point
macro support, build-time RP1 note generation, and a minimal firmware example.

## `rp1-abi`

`crates/rp1-abi` defines the initial fixed `.note.rp1` ABI shared by RP1
firmware images and the CM5 RP1 bootloader PoC. It is `#![no_std]` and currently
contains only the boot note layout and owner bitmap device constants.

## Minimal Firmware

`examples/minimal` builds a `thumbv7m-none-eabi` no-std firmware ELF and uses
`rp1-build` to generate an RP1 boot note from `examples/minimal/rp1.toml`.

Build it:

```sh
nix develop -c cargo build -p rp1-example-minimal --release --target thumbv7m-none-eabi
```

Attach the generated note to the ELF:

```sh
NOTE_BIN=$(find target/thumbv7m-none-eabi/release/build/rp1-example-minimal-* -name rp1_note.bin | head -n1)
tools/attach-rp1-note.sh \
  target/thumbv7m-none-eabi/release/rp1-example-minimal \
  "$NOTE_BIN" \
  target/thumbv7m-none-eabi/release/rp1-example-minimal-note.elf
```

The minimal owner bitmap generated from `examples/minimal/rp1.toml` is:

```text
owner_rp1      = 0x343
owner_linux    = 0x3c
owner_disabled = 0x80
```

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
cargo build --workspace
```

Build the firmware target explicitly when producing RP1 images:

```sh
cargo build -p rp1-example-minimal --release --target thumbv7m-none-eabi
```

The shell also provides a convenience wrapper:

```sh
build-rp1-hal
```

`build-rp1-hal` runs the same target build through the Nix-provided toolchain.
