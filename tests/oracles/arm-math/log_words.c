#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

static void compare_word(uint32_t input) {
    float x;
    memcpy(&x, &input, sizeof(x));
    float y = logf(x);
    uint32_t output;
    memcpy(&output, &y, sizeof(y));
    printf("%08x %08x\n", input, output);
}

int main(void) {
    for (uint32_t exponent = 0; exponent < 256; ++exponent) {
        for (uint32_t sample = 0; sample < 64; ++sample) {
            compare_word((exponent << 23) | ((sample * 2654435761u) & 0x7fffffu));
        }
    }
    for (uint32_t i = 0; i <= 16; ++i) {
        uint32_t boundary = 0x3f330000u + (i << 19);
        for (int offset = -2; offset <= 2; ++offset)
            compare_word(boundary + offset);
    }
    const uint32_t special[] = {1, 0x7fffff, 0x800000, 0x3f800000, 0x80000000,
                                0xbf800000, 0xff800000, 0xffc00002};
    for (size_t i = 0; i < sizeof(special) / sizeof(special[0]); ++i)
        compare_word(special[i]);
    return ferror(stdout) != 0;
}
