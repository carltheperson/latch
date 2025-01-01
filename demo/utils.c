void print(const char *message, long length) {
  asm volatile(
      "mov $1, %%rax\n"
      "mov $1, %%rdi\n"
      "mov %0, %%rsi\n"
      "mov %1, %%rdx\n"
      "syscall\n"
      :
      : "r"(message), "r"(length)
      : "rax", "rdi", "rsi", "rdx");
}

void print_number(long num) {
  char buf[32];
  long i = 31;
  buf[i] = '\n';
  i--;
  if (num == 0) {
    buf[i--] = '0';
  } else {
    while (num > 0) {
      buf[i--] = '0' + (num % 10);
      num /= 10;
    }
  }
  i++;
  long length = 32 - i;

  asm volatile(
      "mov $1, %%rax\n"
      "mov $1, %%rdi\n"
      "mov %0, %%rsi\n"
      "mov %1, %%rdx\n"
      "syscall\n"
      :
      : "r"(&buf[i]), "r"(length)
      : "rax", "rdi", "rsi", "rdx");
}
