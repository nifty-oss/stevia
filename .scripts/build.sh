#!/usr/bin/env bash

MANIFEST_FILE="$1"
shift

if [ ! -f "$MANIFEST_FILE" ]; then
    echo -n "$(RED "error: ")"
    echo "$(GREY "missing manifest file (use 'fmt <manifest file>')")"
    exit 1
fi

if [ ! -z "$RUST_TOOLCHAIN_BUILD" ]; then
    TOOLCHAIN="+$RUST_TOOLCHAIN_BUILD"
fi

cargo $TOOLCHAIN build --manifest-path $MANIFEST_FILE $@
