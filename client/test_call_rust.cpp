#include "vitium_client_rust.h"
#include <iostream>
int main() {
    std::cout << test_fn(1, 2).a._0.a << std::endl;
    return 0;
}