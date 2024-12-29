#include "include.h"

static int self_n = 12;

static int retrieve_self_n(int offset) { return self_n + offset; }

void announce_self_12(){
    int offset = 52;
    int n = retrieve_self_n(offset);
    n -= offset;
    print("This is my number: ", 19);
    print_number((long)n);
    print("\n", 1);
} 

void announce_others_12(){
  print("-- I will announce others: ", 27);
  print_number((long)self_n);
  print("\n", 1);
  // Below is a random segment of other files
  announce_self_9();
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
}