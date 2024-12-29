#include "include.h"

static int self_n = 51;

static int retrieve_self_n(int offset) { return self_n + offset; }

void announce_self_51(){
    int offset = 78;
    int n = retrieve_self_n(offset);
    n -= offset;
    print("This is my number: ", 19);
    print_number((long)n);
    print("\n", 1);
} 

void announce_others_51(){
  print("-- I will announce others: ", 27);
  print_number((long)self_n);
  print("\n", 1);
  // Below is a random segment of other files
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
}