#!/usr/bin/env bash

set -euxo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# rebuild provwasm-std
cd "$SCRIPT_DIR/../packages/proto-build/" && cargo run -- --update-deps
