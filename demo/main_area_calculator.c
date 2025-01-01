#include "include.h"

void _start() {
  int width = 5, height = 10;
  int area = calculate_rectangle_area(width, height);

  print("Rectangle 5x10, Calculated Area: ", 34);
  print_number((long)area);

  // Exit
  asm volatile(
      "mov $60, %rax\n"
      "xor %rdi, %rdi\n"
      "syscall\n");
}
