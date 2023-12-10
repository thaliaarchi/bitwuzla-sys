#!/bin/sh

# cargo install bindgen-cli
bindgen \
    --output bindings.rs \
    --allowlist-function '^bitwuzla_(.*)$' \
    --allowlist-type '^Bitwuzla(.*)$' \
    --no-recursive-allowlist \
    --no-doc-comments \
    --raw-line 'use libc::FILE;' \
    --no-prepend-enum-name \
    ../bitwuzla/include/bitwuzla/c/bitwuzla.h \
    -- \
    -I../bitwuzla/include
