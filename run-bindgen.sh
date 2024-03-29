#!/bin/sh
bindgen \
    --allowlist-function 'ao_fec_encode|ao_fec_decode' \
    ao-fec/src/ao_fec.h >src/ao_fec_ffi.rs
