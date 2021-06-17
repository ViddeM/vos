#include "print.h"

void kernel_main() {
    print_clear();
    print_set_color(PRINT_COLOR_WHITE, PRINT_COLOR_GREEN);
    print_str("Welcome to the 64-bit VOS kernel!");
}
