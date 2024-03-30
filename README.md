![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![crates-io](https://img.shields.io/crates/v/ao-fec.svg)](https://crates.io/crates/ao-fec)
[![api-docs](https://docs.rs/ao-fec/badge.svg)](https://docs.rs/ao-fec)
[![dependency-status](https://deps.rs/repo/github/BartMassey/ao-fec/status.svg)](https://deps.rs/repo/github/BartMassey/ao-fec)

# ao-fec: AltOS FEC using convolutional codes
Copyright © 2024 Bart Massey (Version 0.1.0)

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

# Acknowledgments

The hard part of this is obviously Keith Packard's
work. Thanks much! Thanks also to Keith for helping me debug
and test this.

# License

This code is licensed under the "GNU General Public License
version 2". See the file `csrc/LICENSE.txt` in this codebase for
licensing terms.
