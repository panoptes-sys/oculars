//! Utility macros.

/// Implements formatting for opcodes and mnemonics by forwarding the implementation to `u8`.
macro_rules! impl_byte_fmt {
    ($name: ident, $($fmt: ident),+) => {
        $(
            impl std::fmt::$fmt for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    std::fmt::$fmt::fmt(&u8::from(*self), f)
                }
            }
        )+
    };
}

pub(crate) use impl_byte_fmt;
