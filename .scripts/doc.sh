#!/usr/bin/env bash

MANIFEST_FILE="$1"
shift

if [ ! -f "$MANIFEST_FILE" ]; then
    echo -n "$(RED "error: ")"
    echo "$(GREY "missing manifest file (use 'fmt <manifest file>')")"
    exit 1
fi

if [ ! -z "$RUST_TOOLCHAIN_LINT" ]; then
    TOOLCHAIN="+$RUST_TOOLCHAIN_LINT"
fi

DOC_ARGS="--all-features \
 --no-deps"

RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo $TOOLCHAIN doc --manifest-path $MANIFEST_FILE $DOC_ARGS $@
