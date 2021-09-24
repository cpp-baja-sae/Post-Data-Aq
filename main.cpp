#include <iostream>

int main(int argc, char **argv) {
  for (int ten_times = 10; ten_times >= 0; ten_times--) {
    std::cout << argv[1] << "\n";
  }

  return 0;
}
