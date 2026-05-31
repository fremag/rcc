Learning Rust and writing a small C compiler with the book:
[Writing a C Compiler
Build a Real Programming Language from Scratch
by Nora Sandler](https://nostarch.com/writing-c-compiler)


# Chapter 1

```bash
~/rcc$ cat return_2.c
int main(void) {
    return 2;
}

~/rcc$ ./target/debug/rcc return_2.c
return_2.c => return_2.i

        .globl main
main:
        movl $2, %eax
        ret
        .section .note.GNU-stack,"",@progbits
Done.

~/rcc$ ./return_2 ; echo $?
2
```
```bash
~/writing-a-c-compiler-tests$ ./test_compiler ../rcc/target/debug/rcc --chapter 1
----------------------------------------------------------------------
Ran 24 tests in 1.158s

OK
```