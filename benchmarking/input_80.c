#include "include.h"

static int self_n = 80;

static int retrieve_self_n(int offset) { return self_n + offset; }

void announce_self_80(){
    int offset = 22;
    int n = retrieve_self_n(offset);
    n -= offset;
    print("This is my number: ", 19);
    print_number((long)n);
    print("\n", 1);
} 

void announce_others_80(){
  print("-- I will announce others: ", 27);
  print_number((long)self_n);
  print("\n", 1);
  // Below is a random segment of other files
  announce_self_47();
  announce_self_48();
  announce_self_49();
  announce_self_50();
  announce_self_51();
  announce_self_52();
  announce_self_53();
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
}