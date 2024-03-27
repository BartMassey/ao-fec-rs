/*
 * Copyright © 2012 Keith Packard <keithp@keithp.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
 */

#ifndef _AO_FEC_H_
#define _AO_FEC_H_

#include <stdint.h>
#include <stddef.h>

#define AO_FEC_CRC_INIT			0xffff
#define AO_FEC_TRELLIS_TERMINATOR	0x0b

extern const uint8_t ao_fec_whiten_table[];

static inline uint16_t
ao_fec_crc_byte(uint8_t byte, uint16_t crc)
{
	uint8_t	bit;

	for (bit = 0; bit < 8; bit++) {
		if (((crc & 0x8000) >> 8) ^ (byte & 0x80))
			crc = (uint16_t) ((crc << 1) ^ 0x8005);
		else
			crc = (crc << 1);
		byte = (uint8_t) (byte << 1);
	}
	return crc;
}

extern uint16_t
ao_fec_crc(const uint8_t *bytes, size_t len);

/*
 * 'len' is the length of the original data; 'bytes'
 * must  be four bytes longer than that, and the first
 * two after 'len' must be the received crc
 */
extern int
ao_fec_check_crc(const uint8_t *bytes, size_t len);

/*
 * Number of bytes needed for encoder output array for input
 * length LEN.
 */
#define AOC_FEC_ENCODE_LEN(LEN) (2 * (((LEN) + 4) & ~1))

/*
 * Compute CRC, whiten, convolve and interleave data. 'out' must be
 * AOC_FEC_ENCODE_LEN(len) bytes long.
 */
extern size_t
ao_fec_encode(const uint8_t *in, size_t len, uint8_t *out);

/*
 * Number of bytes needed for decoder output array for soft
 * input length LEN.
 */
#define AOC_FEC_DECODE_LEN(LEN) ((LEN) / 16)

/*
 * Decode data. 'in' is one byte per bit, soft decision
 * 'out' must be AOC_FEC_DECODE_LEN(LEN) bytes long
 */

#define AO_FEC_DECODE_BLOCK	(32)	/* callback must return multiples of this many bits */

#define AO_FEC_DECODE_CRC_OK	0x80	/* stored in out[out_len-1] */

extern void
ao_fec_decode(const uint8_t *in, size_t in_len, uint8_t *out);

#endif /* _AO_FEC_H_ */
