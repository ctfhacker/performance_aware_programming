
        pub fn read_8x2(buffer: &mut [u8]) {
            unsafe {
                asm!(
                    r#"
                    xor rax, rax
                2:
                    mov r8, [{buffer} + 0]
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
    