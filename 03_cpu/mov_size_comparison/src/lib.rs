#![feature(portable_simd)]
use core::arch::asm;

pub fn read_4x1(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            mov r8d, [{buffer}]
            add rax, 4
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_4x2(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            mov r8d, [{buffer}]
            mov r8d, [{buffer} + 4]
            add rax, 8
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_4x3(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            mov r8d, [{buffer}]
            mov r8d, [{buffer} + 4]
            mov r8d, [{buffer} + 8]
            add rax, 12
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_4x4(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            mov r8d, [{buffer}]
            mov r8d, [{buffer} + 4]
            mov r8d, [{buffer} + 8]
            mov r8d, [{buffer} + 12]
            add rax, 16
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}
pub fn read_4x5(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            mov r8d, [{buffer}]
            mov r8d, [{buffer} + 4]
            mov r8d, [{buffer} + 8]
            mov r8d, [{buffer} + 12]
            mov r8d, [{buffer} + 16]
            add rax, 20
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}
pub fn read_4x6(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            mov r8d, [{buffer}]
            mov r8d, [{buffer} + 4]
            mov r8d, [{buffer} + 8]
            mov r8d, [{buffer} + 12]
            mov r8d, [{buffer} + 16]
            mov r8d, [{buffer} + 20]
            add rax, 24
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_8x1(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            mov r8, [{buffer}]
            add rax, 8
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_8x2(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            mov r8, [{buffer}]
            mov r8, [{buffer} + 8]
            add rax, 16
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_8x3(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            mov r8, [{buffer}]
            mov r8, [{buffer} + 8]
            mov r8, [{buffer} + 16]
            add rax, 24
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_8x4(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            mov r8, [{buffer}]
            mov r8, [{buffer} + 8]
            mov r8, [{buffer} + 16]
            mov r8, [{buffer} + 24]
            add rax, 32
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_8x5(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            mov r8, [{buffer}]
            mov r8, [{buffer} + 8]
            mov r8, [{buffer} + 16]
            mov r8, [{buffer} + 24]
            mov r8, [{buffer} + 32]
            add rax, 48
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_8x6(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            mov r8, [{buffer}]
            mov r8, [{buffer} + 8]
            mov r8, [{buffer} + 16]
            mov r8, [{buffer} + 24]
            mov r8, [{buffer} + 32]
            mov r8, [{buffer} + 48]
            add rax, 64
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_16_sameregx1(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 xmm0, [{buffer}]
            add rax, 16
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_16_sameregx2(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 xmm0, [{buffer}]
            vmovdqu32 xmm0, [{buffer} + 16]
            add rax, 32
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_16_sameregx3(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 xmm0, [{buffer}]
            vmovdqu32 xmm0, [{buffer} + 16]
            vmovdqu32 xmm0, [{buffer} + 32]
            add rax, 48
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_16_sameregx4(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 xmm0, [{buffer}]
            vmovdqu32 xmm0, [{buffer} + 16]
            vmovdqu32 xmm0, [{buffer} + 32]
            vmovdqu32 xmm0, [{buffer} + 48]
            add rax, 64
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}
pub fn read_16_sameregx5(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 xmm0, [{buffer}]
            vmovdqu32 xmm0, [{buffer} + 16]
            vmovdqu32 xmm0, [{buffer} + 32]
            vmovdqu32 xmm0, [{buffer} + 48]
            vmovdqu32 xmm0, [{buffer} + 64]
            add rax, 80
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_16_sameregx6(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 xmm0, [{buffer}]
            vmovdqu32 xmm0, [{buffer} + 16]
            vmovdqu32 xmm0, [{buffer} + 32]
            vmovdqu32 xmm0, [{buffer} + 48]
            vmovdqu32 xmm0, [{buffer} + 64]
            vmovdqu32 xmm0, [{buffer} + 80]
            add rax, 64
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_32_sameregx1(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 ymm0, [{buffer}]
            add rax, 32
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_32_sameregx2(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 ymm0, [{buffer}]
            vmovdqu32 ymm0, [{buffer} + 32]
            add rax, 64
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_32_sameregx3(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 ymm0, [{buffer}]
            vmovdqu32 ymm0, [{buffer} + 32]
            vmovdqu32 ymm0, [{buffer} + 64]
            add rax, 96
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_32_sameregx4(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 ymm0, [{buffer}]
            vmovdqu32 ymm0, [{buffer} + 32]
            vmovdqu32 ymm0, [{buffer} + 64]
            vmovdqu32 ymm0, [{buffer} + 96]
            add rax, 128
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_32_sameregx5(buffer: &[u8]) {
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
            add rax, 160
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

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

pub fn read_64_sameregx1(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 zmm0, [{buffer}]
            add rax, 64
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_64_sameregx2(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 zmm0, [{buffer}]
            vmovdqu32 zmm0, [{buffer} + 64]
            add rax, 128
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_64_sameregx3(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 zmm0, [{buffer}]
            vmovdqu32 zmm0, [{buffer} + 64]
            vmovdqu32 zmm0, [{buffer} + 128]
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

pub fn read_64_sameregx4(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 zmm0, [{buffer}]
            vmovdqu32 zmm0, [{buffer} + 64]
            vmovdqu32 zmm0, [{buffer} + 128]
            vmovdqu32 zmm0, [{buffer} + 192]
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

pub fn read_64_sameregx5(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 zmm0, [{buffer}]
            vmovdqu32 zmm0, [{buffer} + 64]
            vmovdqu32 zmm0, [{buffer} + 128]
            vmovdqu32 zmm0, [{buffer} + 192]
            vmovdqu32 zmm0, [{buffer} + 256]
            add rax, 320
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_64_sameregx6(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 zmm0, [{buffer}]
            vmovdqu32 zmm0, [{buffer} + 64]
            vmovdqu32 zmm0, [{buffer} + 128]
            vmovdqu32 zmm0, [{buffer} + 192]
            vmovdqu32 zmm0, [{buffer} + 256]
            vmovdqu32 zmm0, [{buffer} + 320]
            add rax, 384
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_16x1(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 xmm0, [{buffer}]
            add rax, 16
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_16_diffregx2(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 xmm0, [{buffer}]
            vmovdqu32 xmm1, [{buffer} + 16]
            add rax, 32
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_16_diffregx3(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 xmm0, [{buffer}]
            vmovdqu32 xmm1, [{buffer} + 16]
            vmovdqu32 xmm2, [{buffer} + 32]
            add rax, 48
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_16_diffregx4(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 xmm0, [{buffer}]
            vmovdqu32 xmm1, [{buffer} + 16]
            vmovdqu32 xmm2, [{buffer} + 32]
            vmovdqu32 xmm3, [{buffer} + 48]
            add rax, 64
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_16_diffregx5(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 xmm0, [{buffer}]
            vmovdqu32 xmm1, [{buffer} + 16]
            vmovdqu32 xmm2, [{buffer} + 32]
            vmovdqu32 xmm3, [{buffer} + 48]
            vmovdqu32 xmm4, [{buffer} + 64]
            add rax, 80
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}
pub fn read_16_diffregx6(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 xmm0, [{buffer}]
            vmovdqu32 xmm1, [{buffer} + 16]
            vmovdqu32 xmm2, [{buffer} + 32]
            vmovdqu32 xmm3, [{buffer} + 48]
            vmovdqu32 xmm4, [{buffer} + 64]
            vmovdqu32 xmm5, [{buffer} + 80]
            add rax, 96
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_32x1(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 ymm0, [{buffer}]
            add rax, 32
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_32_diffregx2(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 ymm0, [{buffer}]
            vmovdqu32 ymm1, [{buffer} + 32]
            add rax, 64
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_32_diffregx3(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 ymm0, [{buffer}]
            vmovdqu32 ymm1, [{buffer} + 32]
            vmovdqu32 ymm2, [{buffer} + 64]
            add rax, 96
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_32_diffregx4(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 ymm0, [{buffer}]
            vmovdqu32 ymm1, [{buffer} + 32]
            vmovdqu32 ymm2, [{buffer} + 64]
            vmovdqu32 ymm3, [{buffer} + 96]
            add rax, 128
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_32_diffregx5(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 ymm0, [{buffer}]
            vmovdqu32 ymm1, [{buffer} + 32]
            vmovdqu32 ymm2, [{buffer} + 64]
            vmovdqu32 ymm3, [{buffer} + 96]
            vmovdqu32 ymm4, [{buffer} + 128]
            add rax, 160
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_32_diffregx6(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 ymm0, [{buffer}]
            vmovdqu32 ymm1, [{buffer} + 32]
            vmovdqu32 ymm2, [{buffer} + 64]
            vmovdqu32 ymm3, [{buffer} + 96]
            vmovdqu32 ymm4, [{buffer} + 128]
            vmovdqu32 ymm5, [{buffer} + 160]
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

pub fn read_64x1(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 zmm0, [{buffer}]
            add rax, 64
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_64_diffregx2(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 zmm0, [{buffer}]
            vmovdqu32 zmm1, [{buffer} + 64]
            add rax, 128
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_64_diffregx3(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 zmm0, [{buffer}]
            vmovdqu32 zmm1, [{buffer} + 64]
            vmovdqu32 zmm2, [{buffer} + 128]
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

pub fn read_64_diffregx5(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 zmm0, [{buffer}]
            vmovdqu32 zmm1, [{buffer} + 64]
            vmovdqu32 zmm2, [{buffer} + 128]
            vmovdqu32 zmm3, [{buffer} + 192]
            vmovdqu32 zmm4, [{buffer} + 256]
            add rax, 320
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn read_64_diffregx6(buffer: &[u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            vmovdqu32 zmm0, [{buffer}]
            vmovdqu32 zmm1, [{buffer} + 64]
            vmovdqu32 zmm2, [{buffer} + 128]
            vmovdqu32 zmm3, [{buffer} + 192]
            vmovdqu32 zmm4, [{buffer} + 256]
            vmovdqu32 zmm5, [{buffer} + 320]
            add rax, 384
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub const FUNCS: &[(fn(&[u8]), &str)] = &[
    (read_4x1, "read_4_sameregx1"),
    (read_4x2, "read_4_sameregx2"),
    (read_4x3, "read_4_sameregx3"),
    (read_4x4, "read_4_sameregx4"),
    (read_4x5, "read_4_sameregx5"),
    (read_4x6, "read_4_sameregx6"),
    (read_8x1, "read_8_sameregx1"),
    (read_8x2, "read_8_sameregx2"),
    (read_8x3, "read_8_sameregx3"),
    (read_8x4, "read_8_sameregx4"),
    (read_8x5, "read_8_sameregx5"),
    (read_8x6, "read_8_sameregx6"),
    (read_16x1, "read_16_sameregx1"),
    (read_16x1, "read_16_diffregx1"),
    (read_32x1, "read_32_sameregx1"),
    (read_32x1, "read_32_diffregx1"),
    (read_64x1, "read_64_sameregx1"),
    (read_64x1, "read_64_diffregx1"),
    (read_16_sameregx2, "read_16_sameregx2"),
    (read_16_sameregx3, "read_16_sameregx3"),
    (read_16_sameregx4, "read_16_sameregx4"),
    (read_16_sameregx5, "read_16_sameregx5"),
    (read_16_sameregx6, "read_16_sameregx6"),
    (read_32_sameregx2, "read_32_sameregx2"),
    (read_32_sameregx3, "read_32_sameregx3"),
    (read_32_sameregx4, "read_32_sameregx4"),
    (read_32_sameregx5, "read_32_sameregx5"),
    (read_32_sameregx6, "read_32_sameregx6"),
    (read_64_sameregx2, "read_64_sameregx2"),
    (read_64_sameregx3, "read_64_sameregx3"),
    (read_64_sameregx4, "read_64_sameregx4"),
    (read_64_sameregx5, "read_64_sameregx5"),
    (read_64_sameregx6, "read_64_sameregx6"),
    (read_16_diffregx2, "read_16_diffregx2"),
    (read_16_diffregx3, "read_16_diffregx3"),
    (read_16_diffregx4, "read_16_diffregx4"),
    (read_16_diffregx5, "read_16_diffregx5"),
    (read_16_diffregx6, "read_16_diffregx6"),
    (read_32_diffregx2, "read_32_diffregx2"),
    (read_32_diffregx3, "read_32_diffregx3"),
    (read_32_diffregx4, "read_32_diffregx4"),
    (read_32_diffregx5, "read_32_diffregx5"),
    (read_32_diffregx6, "read_32_diffregx6"),
    (read_64_diffregx2, "read_64_diffregx2"),
    (read_64_diffregx3, "read_64_diffregx3"),
    (read_64_diffregx4, "read_64_diffregx4"),
    (read_64_diffregx5, "read_64_diffregx5"),
    (read_64_diffregx6, "read_64_diffregx6"),
];
