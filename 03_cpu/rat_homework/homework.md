                                                    dst      | src
mov rax, 1   | RAX: ??  RBX: ??  RCX: ??  RDX: ?? | rax->S0  |
mov rbx, 2   | RAX: S0  RBX: ??  RCX: ??  RDX: ?? | rbx->S1  |
mov rcx, 3   | RAX: S0  RBX: S1  RCX: ??  RDX: ?? | rcx->S2  | 
mov rdx, 4   | RAX: S0  RBX: S1  RCX: S2  RDX: ?? | rdx->S3  |
add rax, rbx | RAX: S0  RBX: S1  RCX: S3  RDX: S3 | rax->S5  | S1
add rcx, rdx | RAX: S5  RBX: S1  RCX: S3  RDX: S3 | rcx->S6  | S3
mov rcx, rbx | RAX: S5  RBX: S1  RCX: S6  RDX: S3 | rcx->S7  | S1
inc rax      | RAX: S5  RBX: S1  RCX: S7  RDX: S3 | rax->S8  | S5
dec rcx      | RAX: S8  RBX: S1  RCX: S6  RDX: S3 | rcx->S9  | S6
sub rax, rbx | RAX: S5  RBX: S1  RCX: S9  RDX: S3 | rax->S10 | S1
sub rcx, rdx | RAX: S10 RBX: S1  RCX: S6  RDX: S3 | rcx->S11 | S3
sub rax, rcx | RAX: S5  RBX: S1  RCX: S11 RDX: S3 | rax->S12 | S11
