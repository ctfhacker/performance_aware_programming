RAX: ??? RBX: ??? RCX: ??? RDX: ??? RSP: S0

top:                                                                     dst              | src
  pop rcx       | RAX:  S0 RBX:  S1 RCX:  S2 RDX:  S3 RSP:  S4 ZF: S5  | rsp->S6  rcx->S7 | S4
  sub rsp, rdx  | RAX:  S0 RBX:  S1 RCX:  S2 RDX:  S1 RSP:  S0 ZF: S5  | rsp->S8  zf->S9  | S1
  mov rbx, rax  | RAX:  S0 RBX:  S1 RCX:  S2 RDX:  S1 RSP:  S8 ZF: S9  | rbx->S10         | S0
  shl rbx, 0    | RAX:  S0 RBX: S10 RCX:  S2 RDX:  S1 RSP:  S8 ZF: S9  | rbx->S11         | S10
  not rbx       | RAX:  S0 RBX: S11 RCX:  S2 RDX:  S1 RSP:  S8 ZF: S9  | rbx->S12         | S11
  loopne top    | RAX:  S0 RBX: S12 RCX:  S2 RDX:  S1 RSP:  S8 ZF: S9  | rip->S13         | S9 S2
