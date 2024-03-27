#include <assert.h>
#include <stdio.h>
#include <inttypes.h>
#include <ao_fec.h>

size_t encode(uint8_t *input, size_t len, uint8_t *output) {
    size_t output_len = AOC_FEC_ENCODE_LEN(len);
    size_t result_len = ao_fec_encode(input, len, output);
    assert(result_len == output_len);
    return result_len;
}

ptrdiff_t decode(uint8_t *input, size_t input_len, uint8_t **output) {
    size_t soft_len = 8 * input_len;
    uint8_t soft[soft_len];
    for (size_t i = 0; i < input_len; i++) {
        for (size_t j = 0; j < 8; j++) {
            soft[8 * i + j] = 0xff * ((input[i] >> j) & 1);
        }
    }

    size_t decode_len = AOC_FEC_DECODE_LEN(soft_len);
    printf("input_len = %ld\n", input_len);
    printf("soft_len = %ld\n", soft_len);
    printf("decode_len = %ld\n", decode_len);
    assert(decode_len <= 256);
    static uint8_t decoded[256];
    int decode_result = ao_fec_decode(soft, soft_len, decoded, decode_len);
    if (!decode_result) {
        return -1;
    }
    if (!decoded[decode_len - 1]) {
        for (size_t i = 0; i < decode_len; i++) {
            uint8_t byte = decoded[i];
            printf("d[%ld] = %d (%c)\n", i, byte, byte);
        }
        return -1;
    }

    *output = decoded;
    return (ptrdiff_t) decode_len - 1;
}

int main() {
    uint8_t input[] = "hello";
    size_t nbytes = sizeof(input) - 1;
    uint8_t packet[AOC_FEC_ENCODE_LEN(nbytes)];
    size_t len = encode(input, nbytes, packet);
    for (size_t i = 0; i < len; i++) {
        printf("%02x\n", packet[i] & 0xff);
    }

    uint8_t *decoded;
    ptrdiff_t result = decode(packet, len, &decoded);
    if (result == -1) {
        printf("decode failed\n");
        return 1;
    }
    printf("decode succeeded\n");
    for (size_t i = 0; i < nbytes; i++) {
        assert(input[i] == decoded[i]);
    }
    return 0;
}
