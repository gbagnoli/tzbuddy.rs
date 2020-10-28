#!/bin/bash

set -eu
set -o pipefail

pushd "$(git rev-parse --show-toplevel)" >/dev/null
tmpdir="$(mktemp -d)"

cleanup() {
  rm -rf "$tmpdir"
}

trap cleanup EXIT

rust=()
shell=()
circleci=false
total=0
while read -r fname; do
  total=$((total + 1))
  fullpath="$tmpdir/$fname"
  if [[ "$fname" == *.rs ]]; then
    rust+=("$fullpath")
  elif [[ "$fname" == *.sh ]]; then
    shell+=("$fullpath")
  fi
  if [[ "$fname" == ".circleci/config.yml" ]]; then
    circleci=true
  fi
done < <(git diff --cached --name-only --diff-filter=ACM)

if [ $total -eq 0 ]; then exit 0 ; fi

git checkout-index --prefix="$tmpdir"/ -af
set +e
ec=0

if $circleci; then
  if [ -x "$(command -v circleci)" ]; then
    echo "Validating CircleCI config"
    circleci config validate .circleci/config.yml ; ec=$?
  else
    echo "Cannot validate CircleCI config, missing CLI"
  fi
fi
if [ "${#rust[@]}" -gt 0 ]; then
  echo "Running cargo check"
  cargo check && \
  echo "Running cargo fmt" && \
  cargo fmt -- --check && \
  echo "Running cargo clippy" && \
  cargo clippy --all --all-targets -- -Dwarnings -Drust-2018-idioms && \
  echo "Running cargo test" && \
  cargo test
  ec=$?
fi
if [ "${#shell[@]}" -gt 0 ]; then
  if [ -x "$(command -v shellcheck)" ]; then
    echo "running shellcheck"
    shellcheck -x "${shell[@]}"; e=$?; [ $e -ne 0 ] && ec=$e
  else
    echo >&2 "Please install shellcheck for your platform"; ec=1
  fi
fi

exit $ec
