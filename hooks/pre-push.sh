#!/bin/bash

set -eu
set -o pipefail

pushd "$(git rev-parse --show-toplevel)" >/dev/null
cargo check
cargo check && \
cargo format && \
cargo clippy --all --all-targets -- -Dwarnings -Drust-2018-idioms && \
cargo test

find . -name '*.sh' -print0 | xargs -0 shellcheck

if [ -x "$(command -v circleci)" ]; then
  echo "Validating CircleCI config"
  circleci config validate .circleci/config.yml 
fi
popd >/dev/null
