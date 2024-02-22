# MOV read width bandwidths

This repository benchmarks the bandwidth of reading memory via `mov` or `vmovdqu` for 
a variety of register widths.

* `src/lib.rs` contains all of the benchmark functions
* `src/main.rs` contains the benchmark runner

## Results

These results were ran on the following machine:

`Intel(R) Xeon(R) W-2245 CPU @ 3.90GHz`

![svg](./data.png)

## Example benchmark function

This example tests `4` reads of with width `64 bytes` (using AVX512) using 4 different read registers.

```rust
pub fn read_64_diffregx4(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 zmm0, [{buffer}]
            vmovdqu32 zmm1, [{buffer} + 64]
            vmovdqu32 zmm2, [{buffer} + 128]
            vmovdqu32 zmm3, [{buffer} + 192]
            add rax, 256
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}
```

This example tests `6` reads of with width `32 bytes` using the same read register.

```rust
pub fn read_32_sameregx6(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 ymm0, [{buffer}]
            vmovdqu32 ymm0, [{buffer} + 32]
            vmovdqu32 ymm0, [{buffer} + 64]
            vmovdqu32 ymm0, [{buffer} + 96]
            vmovdqu32 ymm0, [{buffer} + 128]
            vmovdqu32 ymm0, [{buffer} + 160]
            add rax, 192
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}
```

