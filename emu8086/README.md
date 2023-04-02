# 8086 Emulator

## AVX512 Emulation

This emulator attempts to create an 8086 vectorized emulator using AVX512. This means we will be running
32 concurrent emulators using the AVX512 instruction set. This is an on going effort as the Performance
Aware Programming series continues.

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

## Decoding Tests

Testing "infrastructure":

  * Build the ./tests/*asm files
  * Decode the built assembly files using `emu8086`
  * Re-build the decoded assembly as a sanity check
  * Compare the original assembly with the rebuilt assembly using radiff2 from [radare2](https://github.com/radareorg/radare2)

```
$ ./test.sh
```

## Performance

Basic performance metrics are included with cycles and clock time:

```
CPU Speed: 3.9 GHz
Number of iterations: 0x1fff
ReadInput
  Best 2.50µs
  Avg  2.77µs/iter
  Best 9692 cycles/iter
  Avg  10724.00 cycles/iter
  % of total time:  1.37%
Decode
  Best 4.63µs
  Avg  5.08µs/iter
  Best 17979 cycles/iter
  Avg  19766.54 cycles/iter
  % of total time:  2.53%
WriteDecode
  Best 142.52µs
  Avg  191.96µs/iter
  Best 557249 cycles/iter
  Avg  750755.01 cycles/iter
  % of total time: 95.99%
```

