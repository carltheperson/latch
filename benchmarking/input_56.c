#include "include.h"

static int self_n = 56;

static int retrieve_self_n(int offset) { return self_n + offset; }

void announce_self_56(){
    int offset = 38;
    int n = retrieve_self_n(offset);
    n -= offset;
    print("This is my number: ", 19);
    print_number((long)n);
    print("\n", 1);
} 

void announce_others_56(){
  print("-- I will announce others: ", 27);
  print_number((long)self_n);
  print("\n", 1);
  // Below is a random segment of other files
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
  announce_self_44();
  announce_self_45();
  announce_self_46();
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
}