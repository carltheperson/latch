import argparse
import random

def write_file(name, src):
    with open(f"benchmarking/{name}", "w") as f:
        f.write(src)

def create_input_files(amount):
    for n in range(amount):
        start = random.randint(0, amount)
        end = random.randint(start, amount)
        calls = "\n".join([f"  announce_self_{n}();" for n in range(start, end)])
        offset = random.randrange(0, 100);
        src = f"""#include "include.h"

static int self_n = {n};

static int retrieve_self_n(int offset) {{ return self_n + offset; }}

void announce_self_{n}(){{
    int offset = {offset};
    int n = retrieve_self_n(offset);
    n -= offset;
    print("This is my number: ", 19);
    print_number((long)n);
    print("\\n", 1);
}} 

void announce_others_{n}(){{
  print("-- I will announce others: ", 27);
  print_number((long)self_n);
  print("\\n", 1);
  // Below is a random segment of other files
{calls}
}}"""
        write_file(f"input_{n}.c", src)

def create_include_file(amount):
    externs = "\n".join([f"extern void announce_others_{n}();\nextern void announce_self_{n}();" for n in range(amount)])
    src = externs + """
void print(const char *message, long length);
void print_number(long num);
"""
    write_file("include.h", src)

    

def create_main_file(amount):
    calls = "\n".join([f"  announce_others_{n}();" for n in range(amount)])
    src = f"""#include "include.h"

void _start() {{
{calls}
  asm volatile(
      "mov $60, %rax\\n"
      "xor %rdi, %rdi\\n"
      "syscall\\n");
}}
"""
    write_file("main.c", src)

def create_object_files_script(amount):
    c_sources = ["main", "global"] + [f"input_{n}" for n in range(amount)]
    commands = "\n".join([f"gcc -c benchmarking/{s}.c -o benchmarking/object_files/{s}" for s in c_sources])
    src = f"""#!/bin/sh
{commands}
"""
    write_file("create_object_files.sh", src)

def create_run_ld_script(amount):
    prefix = "benchmarking/object_files/"
    elf_sources = " ".join([f"{prefix}main", f"{prefix}global"] + [f"{prefix}input_{n}" for n in range(amount)])
    src = f"""#!/bin/sh
ld -o ld_out {elf_sources}
"""
    write_file("../run_ld.sh", src)

def create_run_latch_script(amount):
    prefix = "benchmarking/object_files/"
    elf_sources = " ".join([f"{prefix}main", f"{prefix}global"] + [f"{prefix}input_{n}" for n in range(amount)])
    src = f"""#!/bin/sh
target/release/latch {elf_sources}
"""
    write_file("../run_latch.sh", src)

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--amount", required=True, type=int, help="Specify the amount.")
    args = parser.parse_args()
    create_input_files(args.amount)
    create_main_file(args.amount)
    create_include_file(args.amount)
    create_object_files_script(args.amount)
    create_run_ld_script(args.amount)
    create_run_latch_script(args.amount)

if __name__ == "__main__":
    main()