#include <assert.h>
#include <stdio.h>
#include <inttypes.h>
#include <ao_fec.h>

uint8_t encode(uint8_t *input, size_t len, uint8_t *output) {
    size_t output_len = 2 * (len + AO_FEC_PREPARE_EXTRA);
    assert(output_len <= 255);
    uint8_t result_len = ao_fec_encode(input, (uint8_t) len, output);
    assert(result_len <= output_len);
    printf("%ld %ld %d\n", len, output_len, result_len);
    return result_len;
}

int main() {
    uint8_t input[] = "hello";
    uint8_t packet[2 * (5 + AO_FEC_PREPARE_EXTRA)];
    uint8_t len = encode(input, 5, packet);
    for (int i = 0; i < len; i++) {
        printf("%02x\n", packet[i] & 0xff);
    }
    return 0;
}
