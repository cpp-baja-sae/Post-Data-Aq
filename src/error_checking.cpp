#include "error_checking.h"
#include <cstdlib>
#include <iostream>
#include <math.h>

using namespace std;
int hammingcode(string input) {
  int data_bits[16];
  int data_size;
  int redundant_bits = 0;
  int parity; 

  cout<<"Enter the size of data bits: ";
  cin>>data_size;
  

  // finding no. of redundant bits
 #define SQUARE_LAW 2
  while (pow(SQUARE_LAW, redundant_bits) < data_size + redundant_bits + 1) {
    redundant_bits++;
  }

   cout<<"Enter the data bit: ";
  for (int i = 1; i <= data_size; i++){
     cin>>data_bits[i];
    // data_bits[i] = 1;
    }

  int hamming[data_size + redundant_bits];
  int redun_exponent = 0;
  int index_input = 1;

  // finding positions of redundant bits.
  for (int i = 1; i <= data_size + redundant_bits; i++) {

    if (i == pow(SQUARE_LAW, redun_exponent)) {
      hamming[i] = -1;
      redun_exponent++;
    } else {
      hamming[i] = data_bits[index_input];
      index_input++;
    }
  }

  int redundant_bit_position = 0;
  int x = 0;
  int xor_min = 0;
  int xor_max = 0;
  int j = 0;
  //int parity = 0
  // finding parity bit
  for (int i = 1; i <= data_size + redundant_bits; i = pow(SQUARE_LAW, redundant_bit_position)) {
    redundant_bit_position++;
    parity = 0;
    j = i;
    x = i;
    int xor_min = 1;
    int xor_max = i;
    while (j <= data_size + redundant_bits) {
      for (x = j; xor_max >= xor_min && x <= data_size + redundant_bits; xor_min++, x++) {
        if (hamming[x] == 1)
          parity = parity + 1;
         //cout << x << "\n";
      }
      //cout<<"\n" << j << "\n";
      j = x + i;
     xor_min = 1;
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
   }
   // NEWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW


// storing redundant bits of hamming[i] into redundant_bits_array[i]
  int redundant_bits_array[16];
  int redun_exponent_2 = 0;
  int e = 1;

  for (int i = 1; i <= data_size + redundant_bits; i++) {

   if (i == pow(SQUARE_LAW, redun_exponent_2)) {
      redundant_bits_array[e] = hamming[i];
      e = e + 1;
      redun_exponent_2++;
  }
  }
  cout<<"\nredundant bits is: ";
  for(int i = 1; i <= redun_exponent_2; i++){
   cout<<redundant_bits_array[i]<<" ";
  }


//storing data bits of hamming[i] into data_bit_array[i]
  int data_bits_array[16];
  int redun_exponent_3 = 0;
  int w = 1;

for (int i = 1; i <= data_size + redundant_bits; i++) {

   if (i != pow(SQUARE_LAW, redun_exponent_3)) {
      data_bits_array[w] = hamming[i];
      w = w + 1;
      redun_exponent_3++;
  }
  }
  cout<<"\ndata bits is: ";
  for(int i = 2; i <= redun_exponent_3; i++){
   cout<<data_bits_array[i]<<" ";
  }

  return 0;
}