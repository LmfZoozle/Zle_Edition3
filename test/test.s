.intel_syntax noprefix
.globl main
main:
push 10
push 5
  pop rdi
  pop rax
  sub rax, rdi
push rax
push 3
  pop rdi
  pop rax
  add rax, rdi
push rax
  pop rax
  ret
