Learning Rust and writing a small C compiler with the book:
[Writing a C Compiler
Build a Real Programming Language from Scratch
by Nora Sandler](https://nostarch.com/writing-c-compiler)



<details open>
<summary>Chapter 4: Logical and relational operators</summary>

```bash
~/writing-a-c-compiler-tests$ ./test_compiler ../rcc/target/debug/rcc --chapter 4
----------------------------------------------------------------------
Ran 105 tests in 9.990s
OK
```

----------------------------------------------------------------------
```bash
~/rcc$ cargo run -- --full /home/fred/writing-a-c-compiler-tests/tests/chapter_4/valid/and_short_circuit.c
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
Running `target/debug/rcc --full /home/fred/writing-a-c-compiler-tests/tests/chapter_4/valid/and_short_circuit.c`
/home/fred/writing-a-c-compiler-tests/tests/chapter_4/valid/and_short_circuit.c => /home/fred/writing-a-c-compiler-tests/tests/chapter_4/valid/and_short_circuit.i
```

```c
int main(void) {
  return 0 && (1 / 0);
}
```
```
TackyProgram { 
  function_def: TackyFunction { 
    identifier: "main", 
    body: [
      JumpIfZero { condition: Constant(0), target: "label_and_false_0" }, 
      Binary(Divide, Constant(1), Constant(0), Var("tmp.0")), 
      JumpIfZero { condition: Var("tmp.0"), target: "label_and_false_0" }, 
      Copy { src: Constant(1), dst: Var("tmp.1") }, 
      Jump { target: "label_end_0" }, 
      Label { identifier: "label_and_false_0" }, 
      Copy { src: Constant(0), dst: Var("tmp.1") }, 
      Label { identifier: "label_end_0" }, 
      Return(Var("tmp.1"))
    ]
  } 
}
```
```asm
        .globl main
main:
    pushq %rbp
    movq %rsp, %rbp
    subq $8, %rsp
    movl $0, %r11d
    cmpl $0, %r11d
    jE .L_label_and_false_0
    movl $1, %eax
    cdq
    movl $0, %r10d
    idivl %r10d
    movl %eax, -4(%rbp)
    cmpl $0, -4(%rbp)
    jE .L_label_and_false_0
    movl $1, -8(%rbp)
    jmp  .L_label_end_0
.L_label_and_false_0:
    movl $0, -8(%rbp)
.L_label_end_0:
    movl -8(%rbp), %eax
    movq %rbp, %rsp
    popq %rbp
    ret
.section .note.GNU-stack,"",@progbits
```

Done.
</details>
 
<details>
<summary>Chapter 3: binary operators</summary>

```c
int main(void) {
    return 5 * 4 / 2 -
        3 % (2 + 1);
}
``` 
```asm

        .globl main
main:
        pushq %rbp
        movq %rsp, %rbp
        subq $20, %rsp
        movl $5, -4(%rbp)
        movl -4(%rbp), %r11d
        imull $4, %r11d 
        movl %r11d, -4(%rbp)
        movl -4(%rbp), %eax
        cdq
        movl $2, %r10d
        idivl %r10d
        movl %eax, -8(%rbp)
        movl $2, -12(%rbp)
        addl $1, -12(%rbp) 
        movl $3, %eax
        cdq
        idivl -12(%rbp)
        movl %edx, -16(%rbp)
        movl -8(%rbp), %r10d
        movl %r10d, -20(%rbp)
        movl -16(%rbp), %r10d
        subl %r10d, -20(%rbp) 
        movl -20(%rbp), %eax
        movq %rbp, %rsp
        popq %rbp
        ret
        .section .note.GNU-stack,"",@progbits
```

```bash
~/rcc$ ./associativity_and_precedence ; echo $?
10
```

```bash
~/writing-a-c-compiler-tests$ ./test_compiler ../rcc/target/debug/rcc --chapter 3 
----------------------------------------------------------------------
Ran 66 tests in 5.937s

OK
```
</details>


<details>
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

