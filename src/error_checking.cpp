#include "error_checking.h"
#include <cstdlib>
#include <iostream>
#include <math.h>

using namespace std;
int hammingcode(string input) {
  int data_bits[128000];
  int data_size;
  int redundant_bits = 0;
  int parity; // m = no. of data bits, r = no. of redundant bits

  cout<<"Enter the size of data bits: ";
  cin>>data_size;
  data_size = 128000;

  // finding no. of redundant bits
  // hamming code equation to find redundant bits??? 
  while (pow(2, redundant_bits) < data_size + redundant_bits + 1) {
    redundant_bits++;
  }

   cout<<"Enter the data bit: ";
  for (int i = 1; i <= data_size; i++){
     cin>>data_bits[i];
    data_bits[i] = 1;}

  int hamming[data_size + redundant_bits];
  int redun_exponent = 0;
  int index_input = 1;

  // finding positions of redundant bits.
  for (int i = 1; i <= data_size + redundant_bits; i++) {

    if (i == pow(2, redun_exponent)) {
      hamming[i] = 0;
      redun_exponent++;
    } else {
      hamming[i] = data_bits[index_input];
      index_input++;
    }
  }

  int redundant_bit_position = 0;
  int x = 0;
  int min = 0;
  int max = 0;
  int j ;
  //int parity;
  // finding parity bit
  for (int i = 1; i <= data_size + redundant_bits; i = pow(2, redundant_bit_position)) {
    redundant_bit_position++;
    parity = 0;
    j = i;
    x = i;
    min = 1;
    max = i;
    while (j <= data_size + redundant_bits) {
      for (x = i; max >= min && x <= data_size + redundant_bits; min++, x++) {
        if (hamming[x] == 1)
          parity = parity + 1;
        ;
         cout << x << "\n";
      }
      // cout << x << "\n";
      j = x + i;
      min = 1;
    }

    // checking for even parity
    if (parity % 2 == 0) {
      hamming[i] = 0;
    } else {
      hamming[i] = 1;
    }
  }

  cout<<"\nHamming code is: ";
  for(int i = 1; i <= data_size + redundant_bits ; i++){
   cout<<hamming[i]<<" ";
   cout <<"\n"<<input<< "\n";}

  return 8383;
}