/*!
This Rust library crate wraps Keith Packard's Forward Error
Correction C code from AltOS <git:git.gag.com/fw/altos>.
See <https://github.com/BartMassey/ao-fec> for the
underlying code and details.

The C code underlying this library was originally written
for [AltOS]() embedded OS, used by [Altus Metrum]() rocket
telemetry hardware. For this reason, the error correction
here is compatible (tested) with the hardware FEC in the
Texas Instruments
[CC1111](https://www.ti.com/product/CC1110-CC1111) radio
chip.

CC1111 compatibility makes things a bit strange. The CC1111
uses a rate 1/2 limit 4 convolutional code. A CRC-16
checksum and one or two 0x00 pad bytes are appended to the
payload before encoding. This ensures that the payload is
even-length and that correct decoding can be checked.

The soft decoding code accepts one byte per bit and uses a
Viterbi-style decoder.

The CC1111 limits block sizes to 128 bytes including header
and padding. The underlying C code and this code have been
generalized to allow arbitrary block sizes. That said, long
blocks are probably not a good idea. If the decoder loses
sync, it is likely to corrupt the entire remainder of a
block. Further, the small CRC means that the possibility of
a false positive on a damaged block is high.
*/

/// Module containing bindgen-generated FFI for functions
/// used here.
pub mod ao_fec_ffi;
use ao_fec_ffi::*;

/// Error type for decode failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    /// The output slice provided did not have the correct
    /// length for the input.
    OutputLengthError,
    /// Decode CRC did not match block contents.
    ChecksumMismatch,
}

/// Expected length in bytes of the encoding of an input
/// block of length `len`.
pub const fn encode_len(len: usize) -> usize {
    // Matches `AOC_FEC_ENCODE_LEN(LEN)` in `aoc_fec.h`.
    2 * ((len + 4) & !1)
}

/// Given an `input` block, place in `output` the encoding
/// of the input block including CRC and padding.
///
/// # Panics
///
/// Panics if `output.len()` is not `encode_len(input.len())`.
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

/// Given an `input` block of soft-decision bytes, place in
/// `output` the decoding of the input block including CRC.
///
/// The input is one byte per received bit, MSB…LSB
/// (big-endian), with `0xff…0x00` mapping to `0…1`
/// (inverted) as received value in soft-decision style.
///
/// The size of the `output` block must be at least 2 and exactly
/// satisfy
/// ```ignore
/// input.len() == 8 * aoc_fec_encode_len(output.len() - 2)
/// ```
pub fn decode<'a>(
    input: &[u8],
    output: &'a mut [u8],
) -> Result<&'a [u8], DecodeError> {
    let ninput = input.len();
    let noutput = output.len();
    unsafe {
        let ok = ao_fec_decode(
            input.as_ptr(),
            ninput,
            output.as_mut_ptr(),
            noutput,
        );

        if ok == -1 {
            if output[noutput - 1] == 0 {
                return Err(DecodeError::ChecksumMismatch);
            }
            return Err(DecodeError::OutputLengthError);
        }
    }
    Ok(&output[..noutput - 2])
}
