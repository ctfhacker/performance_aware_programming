//! The intermediate language for executing avx512 instructions
use super::Zmm;
use crate::evex::AvxOperand;

#[derive(Debug, Copy, Clone)]
pub enum JitIL {
    /// vmovdqa64
    Mov { dest: Zmm, src: AvxOperand },

    /// vpsubw
    Sub {
        dest: Zmm,
        op1: Zmm,
        op2: AvxOperand,
    },

    /// vpaddw
    Add {
        dest: Zmm,
        op1: Zmm,
        op2: AvxOperand,
    },

    /// vpcmpw k, zmm, zmm, imm8
    Cmp {
        k: Zmm,
        left: AvxOperand,
        right: AvxOperand,
        op: CmpOp,
    },
}

// CASE (imm8[2:0]) OF
// 0: OP := _MM_CMPINT_EQ
// 1: OP := _MM_CMPINT_LT
// 2: OP := _MM_CMPINT_LE
// 3: OP := _MM_CMPINT_FALSE
// 4: OP := _MM_CMPINT_NE
// 5: OP := _MM_CMPINT_NLT
// 6: OP := _MM_CMPINT_NLE
// 7: OP := _MM_CMPINT_TRUE
// ESAC
// FOR j := 0 to 7
//     i := j*16
//     k[j] := ( a[i+15:i] OP b[i+15:i] ) ? 1 : 0
// ENDFOR
// [MAX:8] := 0
#[derive(Debug, Copy, Clone)]
pub enum CmpOp {
    Equal,
    LessThan,
    LessThanEqual,
    False,
    NotEqual,
    GreaterThanEqual,
    GreaterThan,
    True,
}

#[macro_export]
macro_rules! vpsubw {
    ($op1:expr, $op2:expr, $op3:expr) => {
        Avx512Instruction::default()
            .opcode(AvxOpcode::Sub)
            .op1($op1)
            .op2($op2)
            .op3($op3)
    };
}

#[macro_export]
macro_rules! vpaddw {
    ($op1:expr, $op2:expr, $op3:expr) => {
        Avx512Instruction::default()
            .opcode(AvxOpcode::Add)
            .op1($op1)
            .op2($op2)
            .op3($op3)
    };
}

#[macro_export]
macro_rules! vpcmpw {
    ($op1:expr, $op2:expr, $op3:expr, $cmp:expr) => {
        Avx512Instruction::default()
            .opcode(AvxOpcode::Cmp)
            .op1($op1)
            .op2($op2)
            .op3($op3)
            .imm($cmp as u8)
    };
}

#[macro_export]
macro_rules! vpbroadcastw {
    ($op1:expr, $reg:ident) => {
        Avx512Instruction::default()
            .opcode(AvxOpcode::Broadcast)
            .op1($op1)
            .op2(HostRegister::$reg.as_zmm())
    };
}

#[macro_export]
macro_rules! vpmovdqa64 {
    ($op1:expr, $op2:expr) => {
        Avx512Instruction::default()
            .opcode(AvxOpcode::Mov)
            .op1($op1)
            .op2($op2)
    };
}
