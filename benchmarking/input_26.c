#include "include.h"

static int self_n = 26;

static int retrieve_self_n(int offset) { return self_n + offset; }

void announce_self_26(){
    int offset = 50;
    int n = retrieve_self_n(offset);
    n -= offset;
    print("This is my number: ", 19);
    print_number((long)n);
    print("\n", 1);
} 

void announce_others_26(){
  print("-- I will announce others: ", 27);
  print_number((long)self_n);
  print("\n", 1);
  // Below is a random segment of other files
  announce_self_64();
  announce_self_65();
  announce_self_66();
  announce_self_67();
  announce_self_68();
  announce_self_69();
  announce_self_70();
  announce_self_71();
  announce_self_72();
  announce_self_73();
  announce_self_74();
  announce_self_75();
  announce_self_76();
  announce_self_77();
  announce_self_78();
  announce_self_79();
  announce_self_80();
  announce_self_81();
  announce_self_82();
  announce_self_83();
  announce_self_84();
  announce_self_85();
  announce_self_86();
  announce_self_87();
  announce_self_88();
  announce_self_89();
  announce_self_90();
  announce_self_91();
  announce_self_92();
  announce_self_93();
  announce_self_94();
  announce_self_95();
  announce_self_96();
  announce_self_97();
  announce_self_98();
}