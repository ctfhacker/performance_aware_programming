//! Utilities for the JIT

/// Allocate a Read|Write|Exec buffer used for the JIT buffer
pub fn alloc_rwx(size: usize) -> *mut u8 {
    extern "C" {
        fn mmap(
            addr: *mut u8,
            length: usize,
            prot: i32,
            flags: i32,
            fd: i32,
            offset: usize,
        ) -> *mut u8;
    }

    const PROT_EXEC: i32 = 4;
    const PROT_READ: i32 = 2;
    const PROT_WRITE: i32 = 1;
    const PROT_RWX: i32 = PROT_EXEC | PROT_READ | PROT_WRITE;
    const MAP_ANONYMOUS: i32 = 0x20;
    const MAP_PRIVATE: i32 = 2;

    // Return an RWX Priv/Anon allocated buffer of `size` bytes
    unsafe {
        let res = mmap(
            std::ptr::null_mut(),
            size,
            PROT_RWX,
            MAP_PRIVATE | MAP_ANONYMOUS,
            -1,
            0,
        );
        assert!(!res.is_null(), "Failed to allocate buffer of size {size}");

        res
    }
}
