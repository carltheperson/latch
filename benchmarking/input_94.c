#include "include.h"

static int self_n = 94;

static int retrieve_self_n(int offset) { return self_n + offset; }

void announce_self_94(){
    int offset = 51;
    int n = retrieve_self_n(offset);
    n -= offset;
    print("This is my number: ", 19);
    print_number((long)n);
    print("\n", 1);
} 

void announce_others_94(){
  print("-- I will announce others: ", 27);
  print_number((long)self_n);
  print("\n", 1);
  // Below is a random segment of other files
  announce_self_54();
  announce_self_55();
  announce_self_56();
  announce_self_57();
  announce_self_58();
  announce_self_59();
  announce_self_60();
  announce_self_61();
  announce_self_62();
  announce_self_63();
  announce_self_64();
  announce_self_65();
  announce_self_66();
  announce_self_67();
  announce_self_68();
  announce_self_69();
}