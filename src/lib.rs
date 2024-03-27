pub mod ao_fec_ffi;
use ao_fec_ffi::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    ChecksumMismatch,
}

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

pub const fn decode_len(len: usize) -> usize {
    // Matches `AOC_FEC_DECODE_LEN(LEN)` in `aoc_fec.h`.
    len / 16
}

pub fn decode<'a>(
    input: &[u8],
    output: &'a mut [u8],
) -> Result<&'a [u8], DecodeError> {
    let ninput = input.len();
    let noutput = output.len();
    assert_eq!(noutput, decode_len(ninput));
    unsafe {
        ao_fec_decode(
            input.as_ptr(),
            ninput,
            output.as_mut_ptr(),
        );

        if output[noutput - 1] == 0 {
            return Err(DecodeError::ChecksumMismatch);
        }
    }
    Ok(&output[..noutput - 2])
}
