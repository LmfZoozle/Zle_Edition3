.intel_syntax noprefix
.globl main
main:
push 3
push 2
  pop rdi
  pop rax
  add rax, rdi
push rax
push 1
push 4
  pop rdi
  pop rax
  imul rax, rdi
push rax
  pop rdi
  pop rax
  cmp rax, rdi
  setle al
  movzb rax, al
push rax
  pop rax
  ret
