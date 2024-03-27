pub mod ao_fec_ffi;
use ao_fec_ffi::*;

pub fn encode(input: &[u8], output: &mut Vec<u8>) {
    unsafe {
        // This length is from examination of the underlying
        // C code. It allows for a 2-byte CRC and up to 2
        // bytes of trellis run-out. See
        // `AO_FEC_PREPARE_EXTRA`.
        let extra_len = 2 + 2;
        // This length is from examination of the underlying
        // C code. It allows for a rate 0.5 code.
        let output_len = (input.len() + extra_len) * 2;
        assert!(output_len <= 255);
        output.resize(output_len, 0);
        let len = ao_fec_encode(
            input.as_ptr(),
            input.len() as u8,
            output.as_mut_ptr(),
        );
        assert!(len as usize <= output_len);
        output.resize(len as usize, 0);
    }
}
