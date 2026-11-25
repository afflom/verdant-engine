#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

static void compare_word(uint32_t input) {
    float x;
    memcpy(&x, &input, sizeof(x));
    float y = sinf(x);
    uint32_t output;
    memcpy(&output, &y, sizeof(y));
    printf("%08x %08x\n", input, output);
}

int main(void) {
    const uint32_t boundaries[] = {0x39800000u, 0x3f400000u, 0x3fc90fdbu, 0x42f00000u};
    for (unsigned sign = 0; sign < 2; ++sign) {
        for (uint32_t exponent = 0; exponent < 256; ++exponent)
            for (uint32_t sample = 0; sample < 32; ++sample)
                compare_word((sign << 31) | (exponent << 23) | ((sample * 2654435761u) & 0x7fffffu));
        for (size_t i = 0; i < sizeof(boundaries) / sizeof(boundaries[0]); ++i)
            for (int offset = -3; offset <= 3; ++offset)
                compare_word((sign << 31) | (boundaries[i] + offset));
    }
    const uint32_t special[] = {1, 0x7fffff, 0x800000, 0x3f800000, 0x80000000,
                                0xbf800000, 0xff800000, 0xffc00002};
    for (size_t i = 0; i < sizeof(special) / sizeof(special[0]); ++i)
        compare_word(special[i]);
    return ferror(stdout) != 0;
}
