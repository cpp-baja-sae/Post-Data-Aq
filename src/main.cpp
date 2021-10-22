#include "main.h"
#include "error_checking.h"
#include <iostream>

int main(int argc, char **argv) {

    for (int five = 5; five >= 0; five--) {
        for (int i = 0; i < argc; i++) {
            std::cout << argv[i] << "\n";
        }
    }
    return 0;
}