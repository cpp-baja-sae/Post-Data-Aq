#include "main.h"
#include "error_checking.h"
#include <iostream>
#include <chrono>

int main(int argc, char **argv) {
    std::string var_dummy={"this is my WORLD >:)"};
    auto start = std::chrono::high_resolution_clock::now();
hammingcode(var_dummy);
auto stop = std::chrono::high_resolution_clock::now();
auto duration = std::chrono::duration_cast<std::chrono::microseconds>(stop - start);
 std::cout << "Time taken by function: "
         << duration.count() << " microseconds" << std::endl;
    /*
    for (int five = 5; five >= 0; five--) {
        for (int i = 0; i < argc; i++) {
            std::cout << argv[i] << "\n";
        }
    }
    */
    return 0; 
}