#!/usr/bin/env bash

set -euo pipefail

rustup default nightly
rustup component add --toolchain nightly rustc-dev
