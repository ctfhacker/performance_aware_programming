# 8086 Emulator

Currently only decoding instructions from the 8086 in preparation of potentially simulating the 
8086 later in the series.

## Tests

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
