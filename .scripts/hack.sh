#!/usr/bin/env bash

MANIFEST_FILE="$1"
shift

if [ ! -f "$MANIFEST_FILE" ]; then
    echo -n "$(RED "error: ")"
    echo "$(GREY "missing manifest file (use 'fmt <manifest file>')")"
    exit 1
fi

CHECK_ARGS="--all-targets \
  --feature-powerset"

cargo hack check --manifest-path $MANIFEST_FILE $CHECK_ARGS $@
