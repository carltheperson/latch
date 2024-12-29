#include "include.h"

static int self_n = 18;

static int retrieve_self_n(int offset) { return self_n + offset; }

void announce_self_18(){
    int offset = 59;
    int n = retrieve_self_n(offset);
    n -= offset;
    print("This is my number: ", 19);
    print_number((long)n);
    print("\n", 1);
} 

void announce_others_18(){
  print("-- I will announce others: ", 27);
  print_number((long)self_n);
  print("\n", 1);
  // Below is a random segment of other files
  announce_self_2();
  announce_self_3();
  announce_self_4();
}