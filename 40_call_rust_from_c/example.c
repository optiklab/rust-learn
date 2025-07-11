#include <stdio.h>

// Declare the Rust function
void call_from_c();

int main() {
    printf("Calling Rust from C...\n");
    call_from_c();
    return 0;
}