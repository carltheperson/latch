#include "include.h"

static int self_n = 70;

static int retrieve_self_n(int offset) { return self_n + offset; }

void announce_self_70(){
    int offset = 12;
    int n = retrieve_self_n(offset);
    n -= offset;
    print("This is my number: ", 19);
    print_number((long)n);
    print("\n", 1);
} 

void announce_others_70(){
  print("-- I will announce others: ", 27);
  print_number((long)self_n);
  print("\n", 1);
  // Below is a random segment of other files
  announce_self_10();
  announce_self_11();
  announce_self_12();
  announce_self_13();
  announce_self_14();
  announce_self_15();
  announce_self_16();
  announce_self_17();
  announce_self_18();
  announce_self_19();
  announce_self_20();
  announce_self_21();
  announce_self_22();
  announce_self_23();
  announce_self_24();
  announce_self_25();
  announce_self_26();
  announce_self_27();
  announce_self_28();
  announce_self_29();
  announce_self_30();
  announce_self_31();
  announce_self_32();
  announce_self_33();
  announce_self_34();
  announce_self_35();
  announce_self_36();
  announce_self_37();
}