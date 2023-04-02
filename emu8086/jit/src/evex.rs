//! Provides a minimal EVEX assembler for AVX512 instructions

#[derive(Debug, Copy, Clone)]
pub struct Zmm(pub u8);
impl Zmm {
    /// Check if the zmm register neds a 4th bit set
    fn needs_4_bits(&self) -> bool {
        self.0 & 0b1000 != 0
    }

    /// Check if the zmm register needs a 5th bit set
    fn needs_5_bits(&self) -> bool {
        self.0 & 0b1_0000 != 0
    }
}

/// A K opmask register
#[derive(Debug, Copy, Clone)]
pub struct K(pub u8);

/// An AVX512 operand
#[derive(Debug, Copy, Clone)]
pub enum AvxOperand {
    Zmm(Zmm),
    Immediate(i16),
}

/// Opcodes for the avx512 instructions we are using
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AvxOpcode {
    Sub = 0xf9,
    Mov = 0x6f,
    Broadcast = 0x7b,
    Cmp = 0x3f,
    Add = 0xfd,
}

impl AvxOpcode {
    /// Returns the `mmm` field type for the opcode
    const fn mmm(&self) -> PrefixMmm {
        use AvxOpcode::*;
        match self {
            Sub => PrefixMmm::F,
            Mov => PrefixMmm::F,
            Broadcast => PrefixMmm::F38,
            Cmp => PrefixMmm::F3A,
            Add => PrefixMmm::F,
        }
    }

    /// Returns `true` if the opcode is a wide instruction (W1 prefix)
    const fn is_wide(&self) -> bool {
        matches!(*self, AvxOpcode::Cmp | AvxOpcode::Mov)
    }
}

/// From section 2.3.5 - The VEX prefix
/// Compaction of two-byte and three-byte opcode
enum PrefixMmm {
    F = 1,
    F38 = 2,
    F3A = 3,
}

/// From table 2-12. VEX.pp Interpretation
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum PrefixPp {
    None = 0,
    P_66 = 1,
    P_F3 = 2,
    P_F2 = 3,
}

/// The return type from an assembly of an AVX512 instruction
pub enum EvexResult {
    Bytes6([u8; 6]),
    Bytes7([u8; 7]),
}

impl EvexResult {
    pub fn as_slice(&self) -> &[u8] {
        match self {
            EvexResult::Bytes6(bytes) => bytes,
            EvexResult::Bytes7(bytes) => bytes,
        }
    }
}

/*
enum AvxOperand {
    Zmm(Zmm),
    K(K),
}
*/

#[derive(Default, Debug, Copy, Clone)]
pub struct Avx512Instruction {
    op1: Option<Zmm>,
    op2: Option<Zmm>,
    op3: Option<Zmm>,
    opcode: Option<AvxOpcode>,
    imm: Option<u8>,
}

impl Avx512Instruction {
    pub fn op1(mut self, op1: Zmm) -> Self {
        self.op1 = Some(op1);
        self
    }

    pub fn op2(mut self, op2: Zmm) -> Self {
        self.op2 = Some(op2);
        self
    }

    pub fn op3(mut self, op3: Zmm) -> Self {
        self.op3 = Some(op3);
        self
    }

    pub fn opcode(mut self, opcode: AvxOpcode) -> Self {
        self.opcode = Some(opcode);
        self
    }

    pub fn imm(mut self, imm: u8) -> Self {
        self.imm = Some(imm);
        self
    }

    pub fn assemble(self) -> EvexResult {
        assert!(
            self.op1.is_some(),
            "Cannot assemble AVX512 instruction without op1"
        );
        assert!(
            self.op2.is_some(),
            "Cannot assemble AVX512 instruction without op2"
        );
        assert!(
            self.opcode.is_some(),
            "Cannot assemble AVX512 instruction without opcode"
        );

        evex(
            self.opcode.unwrap(),
            self.op1.unwrap(),
            self.op2.unwrap(),
            self.op3,
            self.imm,
        )
    }
}

/// Reference: Section 2.7.1 in Intel® 64 and IA-32 Architectures Software Developer’s Manual
/// Volume 2 (2A, 2B, 2C, & 2D): Instruction Set Reference, A-Z
pub fn evex(
    opcode: AvxOpcode,
    op1: Zmm,
    op2: Zmm,
    op3: Option<Zmm>,
    imm: Option<u8>,
) -> EvexResult {
    // Always 0x62 evex prefix
    let evex_prefix = 0x62;

    let has_three_ops = op3.is_some();

    // (dst, src) => (dst, dst, src)
    let (op2, op3) = match op3 {
        Some(op3) => (op2, op3),
        None => (op1, op2),
    };

    // Construct p0 (the first evex payload byte)
    // 7 6 5 4 3 2 1 0
    // R X B R'0 0 m m
    // Figure 2-11. Bit Field Layout of the EVEX Prefix
    let r = !op1.needs_4_bits() as u8;
    let x = !op3.needs_5_bits() as u8;
    let b = !op3.needs_4_bits() as u8;
    let rprime = !op1.needs_5_bits() as u8;
    let mmm = opcode.mmm() as u8;
    let p0 = (r << 7) | (x << 6) | (b << 5) | (rprime << 4) | mmm;

    // Construct p1 (the second evex payload byte)
    // 7 6 5 4 3 2 1 0
    // W v v v v 1 p p
    // Figure 2-11. Bit Field Layout of the EVEX Prefix
    let w = opcode.is_wide() as u8;
    let vvvv = if has_three_ops {
        !op2.0 & 0xf
    } else {
        !0 & 0xf
    };

    let pp = PrefixPp::P_66 as u8;
    let p1 = (w << 7) | (vvvv << 3) | (1 << 2) | pp;

    // Construct p2 (the third evex payload byte)
    // 7 6 5 4 3 2 1 0
    // z L'L b V'a a a
    // Figure 2-11. Bit Field Layout of the EVEX Prefix
    let z = 0_u8; // Always zero in our cases
    let ll = 2; // 2 => 512 bits always
    let b = false as u8; // Only used when rounding is enabled
    let mut vprime = !0_u8;
    if has_three_ops {
        vprime = !op2.needs_5_bits() as u8;
    }
    vprime = vprime & 1;
    let aaa = 0;
    let p2 = (z << 7) | (ll << 5) | (b << 4) | (vprime << 3) | aaa;

    // Construct the modrm for this instruction
    let r1 = op1.0;
    let r2 = op3.0;
    let modrm = (3 << 6) | ((r1 & 0b111) << 3) | (r2 & 0b111);

    // Return the result of this assembly
    if let Some(imm) = imm {
        EvexResult::Bytes7([evex_prefix, p0, p1, p2, opcode as u8, modrm, imm])
    } else {
        EvexResult::Bytes6([evex_prefix, p0, p1, p2, opcode as u8, modrm])
    }
}
