#!/usr/bin/env bash

MANIFEST_FILE="$1"
shift

if [ ! -f "$MANIFEST_FILE" ]; then
    echo -n "$(RED "error: ")"
    echo "$(GREY "missing manifest file (use 'fmt <manifest file>')")"
    exit 1
fi

if [ ! -z "$RUST_TOOLCHAIN_FORMAT" ]; then
    TOOLCHAIN="+$RUST_TOOLCHAIN_FORMAT"
fi

cargo $TOOLCHAIN fmt --manifest-path $MANIFEST_FILE $@
