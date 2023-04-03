//! EFlags implementation

pub enum EFlags {
    Carry = 0,
    Parity = 2,
    Auxillary = 4,
    Zero = 6,
    Sign = 7,
    Overflow = 11,
}
