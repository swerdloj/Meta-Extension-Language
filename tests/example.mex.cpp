#include "stdio.h"

@register
struct Example {
    int field1;
    float field2;
};

int main(void) {
    float f = 1.23;

    auto example = @Example {
        .field2 = f + 2 * 3.5,
        .field1 = 0,
    };

    printf("Re-ordered:\nfield1 = %i, field2 = %0.2f", example.field1, example.field2);

    return 0;
}