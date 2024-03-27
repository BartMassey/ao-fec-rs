#include <assert.h>
#include <stdio.h>
#include <inttypes.h>
#include <ao_fec.h>

uint8_t encode(uint8_t *input, size_t len, uint8_t *output) {
    size_t output_len = AOC_FEC_ENCODE_LEN(len);
    assert(output_len <= 255);
    uint8_t result_len = ao_fec_encode(input, (uint8_t) len, output);
    assert(result_len <= output_len);
    return result_len;
}

int main() {
    uint8_t input[] = "hello";
    size_t nbytes = sizeof(input) - 1;
    uint8_t packet[AOC_FEC_ENCODE_LEN(nbytes)];
    uint8_t len = encode(input, nbytes, packet);
    for (int i = 0; i < len; i++) {
        printf("%02x\n", packet[i] & 0xff);
    }
    return 0;
}
