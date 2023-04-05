//! EFlags implementation

pub enum EFlags {
    Carry = (1 << 0),
    Parity = (1 << 2),
    Auxillary = (1 << 4),
    Zero = (1 << 6),
    Sign = (1 << 7),
    Overflow = (1 << 11),
}
