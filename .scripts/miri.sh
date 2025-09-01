#!/usr/bin/env bash

if [ ! -z "$RUST_TOOLCHAIN_LINT" ]; then
    TOOLCHAIN="+$RUST_TOOLCHAIN_LINT"
fi

cargo $TOOLCHAIN miri test $@
