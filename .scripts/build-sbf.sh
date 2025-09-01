#!/usr/bin/env bash

MANIFEST_FILE="$1"
shift

if [ ! -f "$MANIFEST_FILE" ]; then
    echo -n "$(RED "error: ")"
    echo "$(GREY "missing manifest file (use 'fmt <manifest file>')")"
    exit 1
fi

if [ "$(command -v cargo-build-sbf)" = "" ]; then
    echo -n "$(RED "ERROR: ")"
    echo "$(GREY "Missing \`cargo-build-sbf\` command.")"
    exit 1
fi

cargo-build-sbf --manifest-path $MANIFEST_FILE --tools-version v1.51 $@
