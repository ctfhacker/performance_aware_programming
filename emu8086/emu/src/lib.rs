#![feature(stdsimd)]
#![feature(portable_simd)]
#![feature(concat_idents)]
use cpu8086::flags::EFlags;
use std::simd::u16x32;

const LANES: u8 = 32;

/// An vectorized emulator state of 32 simulated 8086 processors
#[derive(Default, Debug)]
pub struct JitEmulatorState {
    pub ax: u16x32,
    pub bx: u16x32,
    pub cx: u16x32,
    pub dx: u16x32,
    pub si: u16x32,
    pub di: u16x32,
    pub sp: u16x32,
    pub bp: u16x32,
    pub ip: u16x32,
    pub flags: u16x32,
}

/*
#[derive(Default, Debug)]
pub struct HostState {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsp: u64,
    rbp: u64,
    rsi: u64,
    rdi: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
}
*/

impl JitEmulatorState {
    /// Get the state for a single core
    pub fn get_cpu_state(&self, cpu: Core) -> CpuState {
        let cpu = usize::from(*cpu);
        CpuState {
            ax: self.ax.as_array()[cpu],
            bx: self.bx.as_array()[cpu],
            cx: self.cx.as_array()[cpu],
            dx: self.dx.as_array()[cpu],
            si: self.si.as_array()[cpu],
            di: self.di.as_array()[cpu],
            sp: self.sp.as_array()[cpu],
            bp: self.bp.as_array()[cpu],
            ip: self.ip.as_array()[cpu],
            flags: self.flags.as_array()[cpu],
        }
    }

    /// Print the CPU state for the given [`Core`]
    pub fn print_cpu_state(&self, core: Core) {
        // Get the CPU state for this core
        let CpuState {
            ax,
            bx,
            cx,
            dx,
            si,
            di,
            sp,
            bp,
            ip,
            flags,
        } = self.get_cpu_state(core);

        // Create the FLAGS string
        let mut eflags = String::new();
        for (flag, ch) in [
            (EFlags::Carry, "C"),
            (EFlags::Parity, "P"),
            (EFlags::Auxillary, "A"),
            (EFlags::Zero, "Z"),
            (EFlags::Sign, "S"),
            (EFlags::Overflow, "O"),
        ] {
            if flags & (1 << flag as usize) > 0 {
                eflags.push_str(ch);
            }
        }

        // Pretty print this core's register state
        println!("Core {:02}", *core);
        println!("    IP: {ip:04x} FLAGS: {flags:04x} {eflags}");
        println!("    AX: {ax:04x} BX: {bx:04x} CX: {cx:04x} DX: {dx:04x}");
        println!("    SP: {sp:04x} BP: {bp:04x} SI: {si:04x} DI: {di:04x}");
    }

    /// Print all CPU states in the emulator
    pub fn print_states(&self) {
        for i in 0..32 {
            self.print_cpu_state(Core(i));
        }
    }
}

macro_rules! impl_offset {
    (8086 $field:ident, $func:ident) => {
        impl JitEmulatorState {
            #[allow(non_upper_case_globals)]
            pub const fn $func() -> isize {
                // Get the address of the base of the EmulatorState struct
                const BASE: *const JitEmulatorState =
                    core::mem::MaybeUninit::<JitEmulatorState>::uninit().as_ptr();

                // Get the address to a struct field
                const $field: *const core::simd::Simd<u16, 32> =
                    unsafe { core::ptr::addr_of!((*BASE).$field) };

                // Return the offset of the struct field from the base address
                unsafe { $field.cast::<u8>().offset_from(BASE.cast::<u8>()) }
            }
        }
    };
    (host $field:ident, $func:ident) => {
        impl HostState {
            pub const fn $func() -> isize {
                // Get the address of the base of the EmulatorState struct
                const BASE: *const HostState =
                    core::mem::MaybeUninit::<HostState>::uninit().as_ptr();

                // Get the address to a struct field
                const $field: *const u64 = unsafe { core::ptr::addr_of!((*BASE).$field) };

                // Return the offset of the struct field from the base address
                unsafe { $field.cast::<u8>().offset_from(BASE.cast::<u8>()) }
            }
        }
    };
}

impl_offset!(8086 ax, ax_offset);
impl_offset!(8086 bx, bx_offset);
impl_offset!(8086 cx, cx_offset);
impl_offset!(8086 dx, dx_offset);
impl_offset!(8086 si, si_offset);
impl_offset!(8086 di, di_offset);
impl_offset!(8086 sp, sp_offset);
impl_offset!(8086 bp, bp_offset);
impl_offset!(8086 ip, ip_offset);
impl_offset!(8086 flags, flags_offset);

// impl_offset!(host rax, rax_offset);
// impl_offset!(host rbx, rbx_offset);
// impl_offset!(host rcx, rcx_offset);
// impl_offset!(host rdx, rdx_offset);
// impl_offset!(host rsi, rsi_offset);
// impl_offset!(host rdi, rdi_offset);

/// A single lane's CPU state
#[derive(Default, Debug, PartialEq, Eq)]
pub struct CpuState {
    ax: u16,
    bx: u16,
    cx: u16,
    dx: u16,
    si: u16,
    di: u16,
    sp: u16,
    bp: u16,
    ip: u16,
    flags: u16,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Core(pub u8);

impl std::ops::Deref for Core {
    type Target = u8;

    /// Dereferences the value.
    fn deref(&self) -> &Self::Target {
        assert!(
            self.0 < LANES,
            "Given CPU id ({self:?}) is larger than number of lanes ({LANES})"
        );

        &self.0
    }
}

macro_rules! impl_reg {
    ($reg:ident, $set_func:ident, $set_single_func:ident) => {
        impl JitEmulatorState {
            pub fn $reg(&mut self) -> u16x32 {
                self.$reg
            }

            pub fn $set_func(&mut self, val: u16) {
                self.$reg = u16x32::splat(val);
            }

            pub fn $set_single_func(&mut self, cpu: Core, val: u16) {
                self.$reg.as_mut_array()[usize::from(*cpu)] = val;
            }
        }
    };
}

impl_reg!(ax, set_ax, set_ax_in);
impl_reg!(bx, set_bx, set_bx_in);
impl_reg!(cx, set_cx, set_cx_in);
impl_reg!(dx, set_dx, set_dx_in);
