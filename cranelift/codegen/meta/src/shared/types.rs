//! This module predefines all the Cranelift scalar types.

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum Bool {
    /// 1-bit bool.
    B1 = 1,
    /// 8-bit bool.
    B8 = 8,
    /// 16-bit bool.
    B16 = 16,
    /// 32-bit bool.
    B32 = 32,
    /// 64-bit bool.
    B64 = 64,
    /// 128-bit bool.
    B128 = 128,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum Int {
    /// 8-bit int.
    I8 = 8,
    /// 16-bit int.
    I16 = 16,
    /// 32-bit int.
    I32 = 32,
    /// 64-bit int.
    I64 = 64,
    /// 128-bit int.
    I128 = 128,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum Float {
    F32 = 32,
    F64 = 64,
}

/// A type representing CPU flags.
///
/// Flags can't be stored in memory.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum Flag {
    /// CPU flags from an integer comparison.
    IFlags,
    /// CPU flags from a floating point comparison.
    FFlags,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum Reference {
    /// 32-bit reference.
    R32 = 32,
    /// 64-bit reference.
    R64 = 64,
}
