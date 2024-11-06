#!/bin/bash

set -eu
set -o pipefail

pushd "$(git rev-parse --show-toplevel)" >/dev/null
tmpdir="$(mktemp -d)"

cleanup() {
  # shellcheck disable=SC2317
  rm -rf "$tmpdir"
}

trap cleanup EXIT

rust=()
shell=()
total=0
while read -r fname; do
  total=$((total + 1))
  fullpath="$tmpdir/$fname"
  if [[ "$fname" == *.rs ]]; then
    rust+=("$fullpath")
  elif [[ "$fname" == *.sh ]]; then
    shell+=("$fullpath")
  fi
done < <(git diff --cached --name-only --diff-filter=ACM)

if [ $total -eq 0 ]; then exit 0 ; fi

git checkout-index --prefix="$tmpdir"/ -af
set +e
ec=0

if [ "${#rust[@]}" -gt 0 ]; then
  echo "Running cargo check"
  cargo check && \
  echo "Running cargo fmt" && \
  cargo fmt -- --check && \
  echo "Running cargo clippy" && \
  cargo clippy --all-features --all --all-targets -- -Dwarnings -Drust-2018-idioms && \
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
