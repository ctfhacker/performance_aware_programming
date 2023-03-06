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
