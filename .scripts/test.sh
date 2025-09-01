#!/usr/bin/env bash

MANIFEST_FILE="$1"
shift

if [ ! -f "$MANIFEST_FILE" ]; then
    echo -n "$(RED "error: ")"
    echo "$(GREY "missing manifest file (use 'fmt <manifest file>')")"
    exit 1
fi

if [ ! -z "$RUST_TOOLCHAIN_TEST" ]; then
    TOOLCHAIN="+$RUST_TOOLCHAIN_TEST"
fi

TEST_ARGS="--all-features"

cargo $TOOLCHAIN test --manifest-path $MANIFEST_FILE $TEST_ARGS $@
