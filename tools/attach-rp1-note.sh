#!/usr/bin/env sh
set -eu

IN_ELF="$1"
NOTE_BIN="$2"
OUT_ELF="$3"

llvm-objcopy \
  --add-section .note.rp1="$NOTE_BIN" \
  --set-section-flags .note.rp1=alloc,readonly \
  "$IN_ELF" "$OUT_ELF"
