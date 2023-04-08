use cpu8086::register::Register;
use jit_emu::JitEmulatorState;

fn main() {
    // Get the struct offsets for the EmulatorState struct
    let ax_offset = JitEmulatorState::ax_offset();
    let bx_offset = JitEmulatorState::bx_offset();
    let cx_offset = JitEmulatorState::cx_offset();
    let dx_offset = JitEmulatorState::dx_offset();
    let si_offset = JitEmulatorState::si_offset();
    let di_offset = JitEmulatorState::di_offset();
    let bp_offset = JitEmulatorState::bp_offset();
    let sp_offset = JitEmulatorState::sp_offset();
    let ip_offset = JitEmulatorState::ip_offset();
    let flags_offset = JitEmulatorState::flags_offset();

    let ax = Register::Ax.as_zmm();
    let bx = Register::Bx.as_zmm();
    let cx = Register::Cx.as_zmm();
    let dx = Register::Dx.as_zmm();
    let si = Register::Si.as_zmm();
    let di = Register::Di.as_zmm();
    let bp = Register::Bp.as_zmm();
    let sp = Register::Sp.as_zmm();
    let ip = Register::Ip.as_zmm();
    let flags = Register::Flags.as_zmm();

    // Write the assembly for entering and exiting the emulator

    // Registers
    // R8  - Scratch used by the JIT itself
    // R9  - Scratch used by the JIT itself
    // R10 - Scratch used by the JIT itself
    // R11 - Scratch used by the JIT itself
    // R12 - Scratch used by the JIT itself
    // R13 - 1 for debug break, 0 for no debug break
    // R14 - JIT buffer to call
    // R15 - Input/Output 8086 Context
    let asm = format!(
        "
        # Restore the 8086 context via the avx512 registers
        vmovdqa64 zmm{ax}, [r15 + {ax_offset:#05x}] # ax offset is {ax_offset:#05x}
        vmovdqa64 zmm{bx}, [r15 + {bx_offset:#05x}] # bx offset is {bx_offset:#05x}
        vmovdqa64 zmm{cx}, [r15 + {cx_offset:#05x}] # cx offset is {cx_offset:#05x}
        vmovdqa64 zmm{dx}, [r15 + {dx_offset:#05x}] # dx offset is {dx_offset:#05x}
        vmovdqa64 zmm{si}, [r15 + {si_offset:#05x}] # si offset is {si_offset:#05x}
        vmovdqa64 zmm{di}, [r15 + {di_offset:#05x}] # di offset is {di_offset:#05x}
        vmovdqa64 zmm{bp}, [r15 + {bp_offset:#05x}] # bp offset is {bp_offset:#05x}
        vmovdqa64 zmm{sp}, [r15 + {sp_offset:#05x}] # sp offset is {sp_offset:#05x}
        vmovdqa64 zmm{ip}, [r15 + {ip_offset:#05x}] # ip offset is {ip_offset:#05x}
        vmovdqa64 zmm{flags}, [r15 + {flags_offset:#05x}] # flags offset is {flags_offset:#05x}

        # If debug is enabled, break
        test r13, r13
        jz 4f
        int 3

        # Call the JIT buffer
        4:
        call r14

        # Save the 8086 context via the avx512 registers
        vmovdqa64 [r15 + {ax_offset:#05x}], zmm{ax} # ax offset is {ax_offset:#05x}
        vmovdqa64 [r15 + {bx_offset:#05x}], zmm{bx} # bx offset is {bx_offset:#05x}
        vmovdqa64 [r15 + {cx_offset:#05x}], zmm{cx} # cx offset is {cx_offset:#05x}
        vmovdqa64 [r15 + {dx_offset:#05x}], zmm{dx} # dx offset is {dx_offset:#05x}
        vmovdqa64 [r15 + {si_offset:#05x}], zmm{si} # si offset is {si_offset:#05x}
        vmovdqa64 [r15 + {di_offset:#05x}], zmm{di} # di offset is {di_offset:#05x}
        vmovdqa64 [r15 + {bp_offset:#05x}], zmm{bp} # bp offset is {bp_offset:#05x}
        vmovdqa64 [r15 + {sp_offset:#05x}], zmm{sp} # sp offset is {sp_offset:#05x}
        vmovdqa64 [r15 + {ip_offset:#05x}], zmm{ip} # ip offset is {ip_offset:#05x}
        vmovdqa64 [r15 + {flags_offset:#05x}], zmm{flags} # ip offset is {flags_offset:#05x}

        vmovdqa64 zmm1, zmm2
        vmovdqa64 zmm2, zmm1

       
        vpbroadcastw zmm1 {{{{k1}}}}, esi
        vpbroadcastw zmm1 {{{{k2}}}}, esi
        vpbroadcastw zmm1 {{{{k3}}}}, esi
        vpbroadcastw zmm1 {{{{k4}}}}, esi
        vpbroadcastw zmm4, edi
        vpbroadcastw zmm4, eax
        vpbroadcastw zmm4, ebx
        vpbroadcastw zmm4, ecx
        vpbroadcastw zmm4, edx
        vpbroadcastw zmm4, ebp
        vpbroadcastw zmm4, esp
        vpbroadcastw zmm4, r8d
        vpbroadcastw zmm4, r9d
        vpbroadcastw zmm4, r10d
        vpbroadcastw zmm4, r11d
        vpbroadcastw zmm4, r12d
        vpbroadcastw zmm4, r13d
        vpbroadcastw zmm4, r14d
        vpbroadcastw zmm4, r15d

        vpsubw zmm1, zmm2, zmm1
        vpaddw zmm1, zmm2, zmm1

        vpcmpw k0, zmm8, zmm7, 0
        vpcmpw k1, zmm8, zmm7, 0
        vpcmpw k2, zmm8, zmm7, 0
        vpcmpw k3, zmm8, zmm7, 0
        vpcmpw k4, zmm8, zmm7, 0
        vpcmpw k5, zmm8, zmm7, 0
        vpcmpw k6, zmm8, zmm7, 0

        vpaddw zmm8, zmm24, zmm28
        vpaddw zmm8, zmm8, zmm28
        "
    );

    // Save the formatted assembly to findme.rs for emu/src/lib.rs to use with include_str!
    #[cfg(feature = "vecemu")]
    let _ = std::fs::write("./.tmp_files/findme.rs", asm);
}
