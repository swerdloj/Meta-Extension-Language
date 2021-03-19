#include "stdio.h"


struct Example {
    int field1;
    float field2;
};

int main(void) {
    float f = 1.23;

    auto example = Example {
    0,
    f + 2 * 3.5
};

    printf("Re-ordered:\nfield1 = %i, field2 = %0.2f", example.field1, example.field2);

    return 0;
}