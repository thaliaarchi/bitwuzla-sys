#!/bin/sh -e

# cargo install bindgen-cli
bindgen \
    --output bindings.rs \
    --allowlist-function '^bitwuzla_(.*)$' \
    --allowlist-type '^Bitwuzla(.*)$' \
    --no-recursive-allowlist \
    --raw-line 'use libc::FILE;' \
    --no-prepend-enum-name \
    ../bitwuzla/include/bitwuzla/c/bitwuzla.h \
    -- \
    -I../bitwuzla/include

# Clean up the bindings:
# - Name nested structs within a union
# - Ignore warnings for bindgen layout tests, that mix snake case and pascal
#   case
sed -i.bak \
    -e 's/__bindgen_anon_1: BitwuzlaOptionInfo__bindgen_ty_1,/value: BitwuzlaOptionValue,/' \
    -e 's/BitwuzlaOptionInfo__bindgen_ty_1/BitwuzlaOptionValue/' \
    -e 's/BitwuzlaOptionInfo_NumericValue/BitwuzlaOptionNumericValue/' \
    -e 's/BitwuzlaOptionInfo_ModeValue/BitwuzlaOptionModeValue/' \
    -e '/#\[test\]/i \
#[allow(non_snake_case)]' \
    bindings.rs
rm bindings.rs.bak
