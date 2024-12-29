#include "include.h"

static int self_n = 17;

static int retrieve_self_n(int offset) { return self_n + offset; }

void announce_self_17(){
    int offset = 90;
    int n = retrieve_self_n(offset);
    n -= offset;
    print("This is my number: ", 19);
    print_number((long)n);
    print("\n", 1);
} 

void announce_others_17(){
  print("-- I will announce others: ", 27);
  print_number((long)self_n);
  print("\n", 1);
  // Below is a random segment of other files
  announce_self_90();
  announce_self_91();
  announce_self_92();
}