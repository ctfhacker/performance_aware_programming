# Performance Aware Programming

This repository contains material from Casey Muratori's  
[Performance Aware Programming](https://www.computerenhance.com/) series.

Most of the exercises will be completed in Rust, C, and Python.


## Problems

* [Interlude - Haversine Distance](./01_haversine)
* [Part 1 - Vectorized 8086 Emulator](./emu8086)

## Vectorized Emulator status

This section will be updated as the course continues.

```
CPU Speed: 3.9 GHz
Number of iterations: 0x1

0x000 add bx, 0x7530       | mov esi, 0x7530
                           | vpbroadcastw zmm28, esi
                           | vpaddw zmm2, zmm2, zmm28
------------------------------------------------------------
0x004 add bx, 0x2710       | mov esi, 0x2710
                           | vpbroadcastw zmm29, esi
                           | vpaddw zmm2, zmm2, zmm29
------------------------------------------------------------
0x008 sub bx, 0xff88       | mov esi, 0xffffff88
                           | vpbroadcastw zmm30, esi
                           | vpsubw zmm2, zmm2, zmm30
------------------------------------------------------------
0x00c sub bx, 0xff88       | mov esi, 0xffffff88
                           | vpbroadcastw zmm31, esi
                           | vpsubw zmm2, zmm2, zmm31
------------------------------------------------------------
0x010 mov bx, 0x1          | mov esi, 0x1
                           | vpbroadcastw zmm2, esi
------------------------------------------------------------
0x013 mov cx, 0x64         | mov esi, 0x64
                           | vpbroadcastw zmm3, esi
------------------------------------------------------------
0x016 add bx, cx           | vpaddw zmm2, zmm2, zmm3
------------------------------------------------------------
0x018 mov dx, 0xa          | mov esi, 0xa
                           | vpbroadcastw zmm4, esi
------------------------------------------------------------
0x01b sub cx, dx           | vpsubw zmm3, zmm3, zmm4
------------------------------------------------------------
0x01d add bx, 0x9c40       | mov esi, 0xffff9c40
                           | vpbroadcastw zmm28, esi
                           | vpaddw zmm2, zmm2, zmm28
------------------------------------------------------------
0x021 add cx, 0xffa6       | mov esi, 0xffffffa6
                           | vpbroadcastw zmm29, esi
                           | vpaddw zmm3, zmm3, zmm29
------------------------------------------------------------
0x024 mov sp, 0x63         | mov esi, 0x63
                           | vpbroadcastw zmm7, esi
------------------------------------------------------------
0x027 mov bp, 0x62         | mov esi, 0x62
                           | vpbroadcastw zmm8, esi
------------------------------------------------------------
0x02a cmp bp, sp           | vpcmpeqw k2, zmm8, zmm7

+--------------------------------------------- CPU Before ---------------------------------------------+
Core 07
    IP: 0000
    AX: 0000 BX: 0000 CX: 0000 DX: 0000
    SP: 0000 BP: 0000 SI: 0000 DI: 0000
+--------------------------------------------- CPU After ----------------------------------------------+
Core 07
    IP: 0000
    AX: 0000 BX: 9ca5 CX: 0000 DX: 000a
    SP: 0063 BP: 0062 SI: 0000 DI: 0000
```

