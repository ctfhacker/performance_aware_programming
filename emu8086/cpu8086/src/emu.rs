//! An 8086 emulator

use anyhow::Result;

use std::path::Path;

use crate::const_checks::{is_valid_address_size, If, True};
use crate::memory::Memory;

pub struct Emulator<const MEMORY_SIZE: usize> {
    /// The memory in this emulator
    pub memory: Memory<MEMORY_SIZE>,

    /// Register state of the emulator
    pub registers: RegisterState,
}

/// The register state of the emulator
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct RegisterState {
    pub ip: u16,
}

impl<const MEMORY_SIZE: usize> Emulator<MEMORY_SIZE>
where
    If<{ is_valid_address_size(MEMORY_SIZE) }>: True,
{
    /// Create an emulator with blank memory
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: RegisterState::default(),
        }
    }

    /// Create an emulator
    pub fn with_memory(path: &Path) -> Result<Self> {
        Ok(Self {
            memory: Memory::from_file(path)?,
            registers: RegisterState::default(),
        })
    }
}
