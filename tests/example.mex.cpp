#include "stdio.h"

@register
struct Example {
    int field1;
    float field2;
};

@register
struct Another {
    int one;
    unsigned char two;
    float three;
};

int main(void) {
    float f = 1.23;

    auto example = @Example {
        .field2 = f + 2.0f * 3.5f,
        .field1 = 0,
    };

    auto another = @Another {
        .three = 0.47f,
        .one = -123,
        .two = 8,
    };

    printf("example:\n\tfield1 = %i\n\tfield2 = %0.2f\n", example.field1, example.field2);
    printf("another:\n\tone = %i\n\ttwo = %i\n\tthree = %0.2f", another.one, another.two, another.three);

    return 0;
}