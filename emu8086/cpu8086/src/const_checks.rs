//! Constant checks

pub struct If<const CONDITION: bool>;
pub trait True {}
impl True for If<true> {}

/// Checks if a value is a power of two. Used for ensuring the memory range
/// is always a power of two
pub const fn is_power_of_two(val: usize) -> bool {
    val & (val - 1) == 0
}

/// Checks that the given address size is a power of two and can fit in a `u16`
pub const fn is_valid_address_size(val: usize) -> bool {
    is_power_of_two(val) && val <= u16::MAX as usize
}
