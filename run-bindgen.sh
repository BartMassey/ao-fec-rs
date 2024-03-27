#!/bin/sh
bindgen \
    --allowlist-function 'ao_fec_check_crc|ao_fec_encode|ao_fec_decode' \
    csrc/ao_fec.h >src/ao_fec_ffi.rs
