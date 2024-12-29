#include "include.h"

void _start() {
  announce_others_0();
  announce_others_1();
  announce_others_2();
  announce_others_3();
  announce_others_4();
  announce_others_5();
  announce_others_6();
  announce_others_7();
  announce_others_8();
  announce_others_9();
  announce_others_10();
  announce_others_11();
  announce_others_12();
  announce_others_13();
  announce_others_14();
  announce_others_15();
  announce_others_16();
  announce_others_17();
  announce_others_18();
  announce_others_19();
  asm volatile(
      "mov $60, %rax\n"
      "xor %rdi, %rdi\n"
      "syscall\n");
}
