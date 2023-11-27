use core::arch::asm;

pub fn test_mov_all_bytes(buffer: &mut [u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            mov [{buffer} + rax], al
            inc rax
            cmp rax, {count}
            jb 2b
        "#,
            buffer = in(reg) buffer.as_ptr(),
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn test_three_byte_nop_all_bytes(buffer: &mut [u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            nop dword ptr [rax] // 3 byte, single instruction NOP
            inc rax
            cmp rax, {count}
            jb 2b
        "#,
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn test_3_single_byte_nop_all_bytes(buffer: &mut [u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            nop // 1 byte NOP, three times
            nop // 1 byte NOP, three times
            nop // 1 byte NOP, three times
            inc rax
            cmp rax, {count}
            jb 2b
        "#,
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn test_1_single_byte_nop_all_bytes(buffer: &mut [u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            nop // 1 byte NOP, once
            inc rax
            cmp rax, {count}
            jb 2b
        "#,
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn test_cmp_all_bytes(buffer: &mut [u8]) {
    unsafe {
        asm!(
            r#"
            xor rax, rax
        2:
            inc rax
            cmp rax, {count}
            jb 2b
        "#,
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}

pub fn test_dec_all_bytes(buffer: &mut [u8]) {
    unsafe {
        asm!(
            r#"
        2:
            dec {count}
            jnz 2b
        "#,
            count = in(reg) buffer.len(),
            options(nostack),
        );
    }
}
