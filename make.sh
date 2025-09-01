#!/usr/bin/env bash

set -e

# Export environment variables and functions.
export RUST_TOOLCHAIN_BUILD="1.84.1"
export RUST_TOOLCHAIN_FORMAT="nightly-2025-02-16"
export RUST_TOOLCHAIN_LINT="nightly-2025-02-16"
export RUST_TOOLCHAIN_TEST="1.84.1"
export SOLANA_CLI_VERSION="2.3.4"
export WORKING_DIR=$(cd $(dirname $0) && pwd)

MANIFEST_FILE="$WORKING_DIR/Cargo.toml"

# Prints a red message.
function RED() { echo $'\e[1;31m'$1$'\e[0m'; }

# Prints a grey dimmed message.
function GREY() { echo $'\e[2;37m'$1$'\e[0m'; }

export -f RED
export -f GREY

# Print the help message.
function help() {
  cli_name=${0##*/}
  echo "
Usage: $cli_name [command]

Commands:
  build      Run cargo build
  build-sbf  Build using cargo-build-sbf
  clippy     Run cargo clippy linter
  doc        Run cargo doc linter
  fmt        Format code using rustfmt
  hack       Run cargo hack
  miri       Run cargo miri
  test       Run tests using cargo test
  *          Help
"
}

COMMAND="$1"

if [ ! -z "$COMMAND" ]; then
  shift
fi

case "$COMMAND" in
  build|b)
    ".scripts/build.sh" $MANIFEST_FILE $@
    ;;
  build-sbf|s)
    ".scripts/build-sbf.sh" $MANIFEST_FILE $@
    ;;
  clippy|c)
    ".scripts/clippy.sh" $MANIFEST_FILE $@
    ;;
  doc|d)
    ".scripts/doc.sh" $MANIFEST_FILE $@
    ;;
  fmt|f)
    ".scripts/fmt.sh" $MANIFEST_FILE $@
    ;;
  hack|h)
    ".scripts/hack.sh" $MANIFEST_FILE $@
    ;;
  miri|m)
    ".scripts/miri.sh" $@
    ;;
  test|t)
    ".scripts/test.sh" $MANIFEST_FILE $@
    ;;
  *)
    help
    ;;
esac
