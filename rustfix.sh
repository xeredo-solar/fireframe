#!/bin/bash

set -euxo pipefail

TOOLCHAIN="x86_64-unknown-linux-gnu"
VERSION=$(rustc --version | grep "[0-9.]*-nightly" -o)

BASE="$HOME/.rustup/toolchains/nightly-$TOOLCHAIN"
SRC="$BASE/lib/libLLVM-10-$VERSION.so"
DST="$BASE/lib/rustlib/$TOOLCHAIN/lib"

ln -s "$SRC" "$DST"
