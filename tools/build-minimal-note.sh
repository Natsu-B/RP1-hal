#!/usr/bin/env sh
set -eu

cargo build -p rp1-example-minimal --release --target thumbv7m-none-eabi

NOTE_BIN=$(find target/thumbv7m-none-eabi/release/build/rp1-example-minimal-* -name rp1_note.bin | sort | tail -n1)

tools/attach-rp1-note.sh \
  target/thumbv7m-none-eabi/release/rp1-example-minimal \
  "$NOTE_BIN" \
  target/thumbv7m-none-eabi/release/rp1-example-minimal-note.elf

printf '%s\n' target/thumbv7m-none-eabi/release/rp1-example-minimal-note.elf
