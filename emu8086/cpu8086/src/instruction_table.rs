//! The 8086 instruction decoding table

/// The purpose of a subset of bits during an instruction decoding
#[derive(Debug, Copy, Clone)]
pub enum BitPurpose {
    /// These bits must literally be a part of the encoding
    Literal,

    /// This bit signals a word/byte operation
    W,

    /// This bit signals the direction to or from a register
    D,

    /// This bit signals a signed operation
    S,

    /// Shift/rotate
    /// 0 - Shift/rotate count is 1
    /// 1 - Shift/rotate count is specified in CL register
    V,

    /// Zero flag
    /// Repeat/loop while zero flag is clear
    /// Repeat/loop while zero flag is set
    Z,

    /// These bits are for the mod encoding field
    Mod,

    /// These bits are for the reg encoding field
    Reg,

    /// These bits are for the rm encoding field
    Rm,

    /// These bits are for the segment register encoding field
    Sr,

    /// These bits are data bits
    Data,

    /// These bits are data bits only if the W bit is also set
    DataIfW,

    /// The low bits of an address
    AddressLow,

    /// The high bits of an address
    AddressHigh,
}

#[derive(Debug, Copy, Clone)]
pub struct BitsEncoding {
    // How the bits are encoded and what they are used for
    pub encoding: BitPurpose,

    /// The number of bits for this encoding
    pub count: u8,

    /// Shift value (used for ESC instruction)
    pub shift: u8,

    /// The constant value (if any) for this encoding
    pub value: Option<u8>,
}

/// Calculate the minimum number of bits to represent the given `val`
///
/// Iterate from most significant bit to least and return the length when
/// the highest set bit is found
const fn bit_count(val: u8) -> u8 {
    let mut i = 7;

    loop {
        // Reached the end of the value, break
        if i == 0 {
            break;
        }

        // If we've found the top set bit, return that as the length
        if val & (1 << i) > 0 {
            return i + 1;
        }

        // Top bit not found, decrement and continue
        i -= 1;
    }

    0
}

/// Number of bit fields that can be encoded in a single instruction
const ENCODING_FIELDS: usize = 8;

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Mov,
    Push,
    Pop,
    Xchg,
    In,
    Out,
    Xlat,
    Lea,
    Lds,
    Les,
    Lahf,
    Sahf,
    Pushf,
    Popf,
    Add,
    Adc,
    Inc,
    Aaa,
    Daa,
    Sub,
    Sbb,
    Dec,
    Cmp,
    Aas,
    Das,
    Mul,
    Imul,
    Aam,
    Div,
    Idiv,
    Aad,
    Cbw,
    Cwd,
    Not,
    Shl,
    Shr,
    Sar,
    Rol,
    Ror,
    Rcl,
    Rcr,
    And,
    Test,
    Or,
    Xor,
    Rep,
    Movs,
    Cmps,
    Scas,
    Lods,
    Stos,
    Call,
    Jmp,
    Je,
    Jl,
    Jle,
    Jb,
    Jbe,
    Jp,
    Jo,
    Js,
    Jne,
    Jnl,
    Jnle,
    Jnb,
    Jnbe,
    Jnp,
    Jno,
    Ret,
    Jns,
    Loop,
    Loopz,
    Loopnz,
    Jcxz,
    Int,
    Into,
    Iret,
    Clc,
    Cmc,
    Stc,
    Cld,
    Std,
    Cli,
    Sti,
    Nlt,
    Wait,
    Esc,
    Lock,
    Segment,
}

macro_rules! bits {
    ($bits:literal, $len:literal) => {
        BitsEncoding {
            encoding: BitPurpose::Literal,
            count: $len,
            shift: 0,
            value: Some($bits),
        }
    };
}

macro_rules! impl_bit {
    ($purpose:ident, $count:literal) => {
        macro_rules! $purpose {
            () => {
                BitsEncoding {
                    encoding: BitPurpose::$purpose,
                    count: $count,
                    shift: 0,
                    value: None,
                }
            };
            ($bits:literal) => {
                BitsEncoding {
                    encoding: BitPurpose::$purpose,
                    count: $count,
                    shift: 0,
                    value: Some($bits),
                }
            };
        }
    };
}

impl_bit!(D, 1);
impl_bit!(W, 1);
impl_bit!(S, 1);
impl_bit!(V, 1);
impl_bit!(Z, 1);
impl_bit!(Mod, 2);
impl_bit!(Reg, 3);
impl_bit!(Sr, 2);
impl_bit!(Rm, 3);

macro_rules! FakeD {
    ($val:expr) => {
        BitsEncoding {
            encoding: BitPurpose::D,
            count: 0,
            shift: 0,
            value: Some($val),
        }
    };
}

macro_rules! FakeReg {
    ($val:expr) => {
        BitsEncoding {
            encoding: BitPurpose::Reg,
            count: 0,
            shift: 0,
            value: Some($val),
        }
    };
}

macro_rules! FakeRm {
    ($val:expr) => {
        BitsEncoding {
            encoding: BitPurpose::Rm,
            count: 0,
            shift: 0,
            value: Some($val),
        }
    };
}

macro_rules! FakeW {
    ($val:expr) => {
        BitsEncoding {
            encoding: BitPurpose::W,
            count: 0,
            shift: 0,
            value: Some($val),
        }
    };
}

macro_rules! FakeMod {
    ($val:expr) => {
        BitsEncoding {
            encoding: BitPurpose::Mod,
            count: 0,
            shift: 0,
            value: Some($val),
        }
    };
}

macro_rules! Data {
    () => {
        BitsEncoding {
            encoding: BitPurpose::Data,
            count: 8,
            shift: 0,
            value: None,
        }
    };
}

macro_rules! DataIfW {
    () => {
        BitsEncoding {
            encoding: BitPurpose::DataIfW,
            count: 8,
            shift: 0,
            value: None,
        }
    };
}

#[derive(Debug, Copy, Clone)]
pub struct InstructionEncoding {
    /// The opcode for this instruction
    pub opcode: Opcode,

    /// The bit encodings for this type of instruction
    pub bit_encodings: [Option<BitsEncoding>; ENCODING_FIELDS],
}

macro_rules! encode {
    ([$op:ident, $a:expr]) => {{
        let mut bit_encodings = [None; ENCODING_FIELDS];
        bit_encodings[0] = Some($a);

        InstructionEncoding {
            opcode: Opcode::$op,
            bit_encodings,
        }
    }};
    ([$op:ident, $a:expr, $b:expr]) => {{
        let mut bit_encodings = [None; ENCODING_FIELDS];
        bit_encodings[0] = Some($a);
        bit_encodings[1] = Some($b);

        InstructionEncoding {
            opcode: Opcode::$op,
            bit_encodings,
        }
    }};
    ([$op:ident, $a:expr, $b:expr, $c:expr]) => {{
        let mut bit_encodings = [None; ENCODING_FIELDS];
        bit_encodings[0] = Some($a);
        bit_encodings[1] = Some($b);
        bit_encodings[2] = Some($c);

        InstructionEncoding {
            opcode: Opcode::$op,
            bit_encodings,
        }
    }};
    ([$op:ident, $a:expr, $b:expr, $c:expr, $d:expr]) => {{
        let mut bit_encodings = [None; ENCODING_FIELDS];
        bit_encodings[0] = Some($a);
        bit_encodings[1] = Some($b);
        bit_encodings[2] = Some($c);
        bit_encodings[3] = Some($d);

        InstructionEncoding {
            opcode: Opcode::$op,
            bit_encodings,
        }
    }};
    ([$op:ident, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr]) => {{
        let mut bit_encodings = [None; ENCODING_FIELDS];
        bit_encodings[0] = Some($a);
        bit_encodings[1] = Some($b);
        bit_encodings[2] = Some($c);
        bit_encodings[3] = Some($d);
        bit_encodings[4] = Some($e);

        InstructionEncoding {
            opcode: Opcode::$op,
            bit_encodings,
        }
    }};
    ([$op:ident, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr]) => {{
        let mut bit_encodings = [None; ENCODING_FIELDS];
        bit_encodings[0] = Some($a);
        bit_encodings[1] = Some($b);
        bit_encodings[2] = Some($c);
        bit_encodings[3] = Some($d);
        bit_encodings[4] = Some($e);
        bit_encodings[5] = Some($f);

        InstructionEncoding {
            opcode: Opcode::$op,
            bit_encodings,
        }
    }};
    ([$op:ident, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr]) => {{
        let mut bit_encodings = [None; ENCODING_FIELDS];
        bit_encodings[0] = Some($a);
        bit_encodings[1] = Some($b);
        bit_encodings[2] = Some($c);
        bit_encodings[3] = Some($d);
        bit_encodings[4] = Some($e);
        bit_encodings[5] = Some($f);
        bit_encodings[6] = Some($g);

        InstructionEncoding {
            opcode: Opcode::$op,
            bit_encodings,
        }
    }};
    ([$op:ident, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr]) => {{
        let mut bit_encodings = [None; ENCODING_FIELDS];
        bit_encodings[0] = Some($a);
        bit_encodings[1] = Some($b);
        bit_encodings[2] = Some($c);
        bit_encodings[3] = Some($d);
        bit_encodings[4] = Some($e);
        bit_encodings[5] = Some($f);
        bit_encodings[6] = Some($g);
        bit_encodings[7] = Some($h);

        InstructionEncoding {
            opcode: Opcode::$op,
            bit_encodings,
        }
    }};
}

#[rustfmt::skip]
pub static INSTRUCTION_TABLE: &'static [InstructionEncoding] = &[
    encode!([Mov, bits!(0b100010, 6), D!(), W!(), Mod!(), Reg!(), Rm!()]),
    encode!([Mov, bits!(0b1100_011, 7), W!(), Mod!(), bits!(0b000, 3), Rm!(), Data!(), DataIfW!(), FakeD!(0)]),
    encode!([Mov, bits!(0b1011, 4), W!(), Reg!(), Data!(), DataIfW!(), FakeD!(1), FakeMod!(0xff), FakeRm!(0)]),
    encode!([Mov, bits!(0b1010_000, 7), W!(), FakeD!(1), FakeMod!(0), FakeRm!(0b110), FakeReg!(0)]),
    encode!([Mov, bits!(0b1010_001, 7), W!(), FakeD!(0), FakeMod!(0), FakeRm!(0b110), FakeReg!(0)]),

    encode!([Push, bits!(0b1111_1111, 8), Mod!(), bits!(0b110, 3), Rm!(), FakeW!(1), FakeD!(0)]),
    encode!([Push, bits!(0b01010, 5), Reg!(), FakeW!(1), FakeMod!(0), FakeRm!(0), FakeD!(1)]),
    encode!([Push, bits!(0b000, 3), Sr!(), bits!(0b110, 3), FakeW!(1), FakeD!(1), FakeMod!(0), FakeRm!(0)]),

    encode!([Pop, bits!(0b1000_1111, 8), Mod!(), bits!(0b000, 3), Rm!(), FakeW!(1), FakeD!(0)]),
    encode!([Pop, bits!(0b01011, 5), Reg!(), FakeW!(1), FakeMod!(0), FakeRm!(0), FakeD!(1)]),
    encode!([Pop, bits!(0b000, 3), Sr!(), bits!(0b111, 3), FakeW!(1), FakeD!(1), FakeMod!(0), FakeRm!(0)]),

    encode!([Xchg, bits!(0b1000011, 7), W!(), Mod!(), Reg!(), Rm!(), FakeD!(1)]),
    encode!([Xchg, bits!(0b10010, 5), Reg!(), FakeW!(1), FakeMod!(0b11), FakeRm!(0), FakeD!(1)]),

    encode!([In, bits!(0b1110010, 7), W!(), FakeMod!(0xff), FakeRm!(0), FakeD!(0)]),
];
