Learning Rust and writing a small C compiler with the book:
[Writing a C Compiler
Build a Real Programming Language from Scratch
by Nora Sandler](https://nostarch.com/writing-c-compiler)

<details open>
<summary>Chapter 2: unary operators</summary>

```c
int main(void) {
    return -(-4);
}
``` 
```asm
        .globl main
main:
        pushq %rbp
        movq %rsp, %rbp
        subq $8, %rsp
        movl $4, -4(%rbp)
        negl -4(%rbp)
        movl -4(%rbp), %r10d
        movl %r10d, -8(%rbp)
        negl -8(%rbp)
        movl -8(%rbp), %eax
        movq %rbp, %rsp
        popq %rbp
        ret
        .section .note.GNU-stack,"",@progbits```
```

```bash
~/rcc$ ./parens_3 ; echo $?
4
```

```bash
~/writing-a-c-compiler-tests$  ./test_compiler ../rcc/target/debug/rcc --chapter 2
----------------------------------------------------------------------
Ran 43 tests in 3.758s

OK
```
</details>


<details>
<summary>Chapter 1: a minimal compiler</summary>

```c
int main(void) {
    return 2;
}
```

```asm
        .globl main
main:
        movl $2, %eax
        ret
        .section .note.GNU-stack,"",@progbits
```        

```bash
~/rcc$ ./return_2 ; echo $?
2
```
```bash
~/writing-a-c-compiler-tests$ ./test_compiler ../rcc/target/debug/rcc --chapter 1
----------------------------------------------------------------------
Ran 24 tests in 1.158s

OK
```
</details>

