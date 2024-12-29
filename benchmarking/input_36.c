#include "include.h"

static int self_n = 36;

static int retrieve_self_n(int offset) { return self_n + offset; }

void announce_self_36(){
    int offset = 77;
    int n = retrieve_self_n(offset);
    n -= offset;
    print("This is my number: ", 19);
    print_number((long)n);
    print("\n", 1);
} 

void announce_others_36(){
  print("-- I will announce others: ", 27);
  print_number((long)self_n);
  print("\n", 1);
  // Below is a random segment of other files
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
  announce_self_38();
  announce_self_39();
  announce_self_40();
  announce_self_41();
  announce_self_42();
  announce_self_43();
}