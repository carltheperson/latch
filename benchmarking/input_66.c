#include "include.h"

static int self_n = 66;

static int retrieve_self_n(int offset) { return self_n + offset; }

void announce_self_66(){
    int offset = 29;
    int n = retrieve_self_n(offset);
    n -= offset;
    print("This is my number: ", 19);
    print_number((long)n);
    print("\n", 1);
} 

void announce_others_66(){
  print("-- I will announce others: ", 27);
  print_number((long)self_n);
  print("\n", 1);
  // Below is a random segment of other files
  announce_self_2();
  announce_self_3();
  announce_self_4();
  announce_self_5();
  announce_self_6();
  announce_self_7();
  announce_self_8();
  announce_self_9();
  announce_self_10();
  announce_self_11();
  announce_self_12();
  announce_self_13();
  announce_self_14();
  announce_self_15();
}