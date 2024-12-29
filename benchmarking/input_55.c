#include "include.h"

static int self_n = 55;

static int retrieve_self_n(int offset) { return self_n + offset; }

void announce_self_55(){
    int offset = 35;
    int n = retrieve_self_n(offset);
    n -= offset;
    print("This is my number: ", 19);
    print_number((long)n);
    print("\n", 1);
} 

void announce_others_55(){
  print("-- I will announce others: ", 27);
  print_number((long)self_n);
  print("\n", 1);
  // Below is a random segment of other files
  announce_self_93();
  announce_self_94();
  announce_self_95();
  announce_self_96();
  announce_self_97();
  announce_self_98();
  announce_self_99();
}