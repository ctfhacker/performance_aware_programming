use anyhow::{ensure, Result};
use thiserror::Error;

use std::mem::size_of;
use std::ops::{Add, Deref};
use std::path::Path;

use crate::const_checks::{is_valid_address_size, If, True};

/// The memory for the emulator
pub struct Memory<const SIZE: usize> {
    pub memory: [u8; SIZE],

    /// Length of valid memory
    pub length: usize,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Attempted to read an out of bounds address: {0:x?}")]
    OutOfBoundsRead(Address),
}

/// An address to reference memory
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Address(pub usize);

impl Address {
    #[allow(dead_code)]
    pub fn offset(self, offset: usize) -> Address {
        Address(self.0 + offset)
    }
}

impl Deref for Address {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Add for Address {
    type Output = Address;

    fn add(self, rhs: Address) -> Self::Output {
        Address(self.0 + rhs.0)
    }
}

impl<const SIZE: usize> Memory<SIZE>
where
    If<{ is_valid_address_size(SIZE) }>: True,
{
    /// Create a new blank
    #[allow(dead_code)]
    pub fn new() -> Memory<SIZE> {
        Memory {
            memory: [0x0_u8; SIZE],
            length: 0,
        }
    }

    /// Create a new [`Memory`] initialized with bytes from the given [`Path`]
    pub fn from_file(path: &Path) -> Result<Memory<SIZE>> {
        // Read the data from disk
        let data = std::fs::read(path)?;

        // Ensure the given data can fit into the expected memory size
        ensure!(
            data.len() <= SIZE,
            "Given memory is too large. Increase memory size"
        );

        // Read the input data into the memory
        let mut memory = [0x0_u8; SIZE];
        memory[..data.len()].copy_from_slice(&data);

        // Return the read in memory
        Ok(Memory {
            memory,
            length: data.len(),
        })
    }

    /// Read the given [`T`] from the [`Address`] location in the memory
    pub fn read<T: Sized + Copy + std::fmt::LowerHex>(&self, address: Address) -> Result<T> {
        // Ensure the memory can be read in bounds.
        // Reminder: SIZE is always a power of two, so this add will work
        let inclusive_end_addr = address.0 + size_of::<T>() - 1;
        ensure!(
            inclusive_end_addr == inclusive_end_addr & (SIZE - 1),
            Error::OutOfBoundsRead(address)
        );

        // Read the value from the memory
        let res = unsafe { *(self.memory[address.0..].as_ptr().cast()) };

        Ok(res)
    }
}
