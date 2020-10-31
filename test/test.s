.intel_syntax noprefix
.globl main
main:
push 5
push 6
  pop rdi
  pop rax
  add rax, rdi
push rax
push 3
push 2
push 3
  pop rdi
  pop rax
  sub rax, rdi
push rax
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
