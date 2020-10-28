#!/bin/bash

set -eu
set -o pipefail

pushd "$(git rev-parse --show-toplevel)" >/dev/null
echo "* Running cargo check"
cargo check
cargo check && \
echo "* Running cargo fmt"
cargo fmt -- --check && \
echo "* Running cargo clippy"
cargo clippy --all --all-targets -- -Dwarnings -Drust-2018-idioms --all-features && \
echo "* Running cargo test"
cargo test

echo "* Running shellcheck"
find . -name '*.sh' -print0 | xargs -0 shellcheck

popd >/dev/null
