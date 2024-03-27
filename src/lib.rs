pub mod ao_fec_ffi;
use ao_fec_ffi::*;

pub const fn encode_len(len: usize) -> usize {
    // Matches `AOC_FEC_ENCODE_LEN(LEN)` in `aoc_fec.h`.
    2 * ((len + 4) & !1)
}

pub fn encode(input: &[u8], output: &mut [u8]) {
    assert_eq!(output.len(), encode_len(input.len()));
    unsafe {
        let len = ao_fec_encode(
            input.as_ptr(),
            input.len(),
            output.as_mut_ptr(),
        );
        assert!(len == output.len());
    }
}
