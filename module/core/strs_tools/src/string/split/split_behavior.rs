//! Provides a custom implementation of bitflags for controlling string splitting behavior.

use core::ops::{ BitOr, BitAnd, Not };

/// Flags to control the behavior of the split iterators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SplitFlags(pub u8);

impl SplitFlags
{
    /// Preserves empty segments.
    pub const PRESERVING_EMPTY: SplitFlags = SplitFlags(1 << 0);
    /// Preserves delimiter segments.
    pub const PRESERVING_DELIMITERS: SplitFlags = SplitFlags(1 << 1);
    /// Preserves quoting characters in the output.
    pub const PRESERVING_QUOTING: SplitFlags = SplitFlags(1 << 2);
    /// Strips leading/trailing whitespace from delimited segments.
    pub const STRIPPING: SplitFlags = SplitFlags(1 << 3);
    /// Enables handling of quoted sections.
    pub const QUOTING: SplitFlags = SplitFlags(1 << 4);

    /// Creates a new `SplitFlags` instance from a raw `u8` value.
    #[ must_use ]
    pub const fn from_bits(bits: u8) -> Option<Self> {
        Some(Self(bits))
    }

    /// Returns the raw `u8` value of the flags.
    #[ must_use ]
    pub const fn bits(&self) -> u8 {
        self.0
    }

    /// Returns `true` if all of `other`'s flags are contained within `self`.
    #[ must_use ]
    pub const fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Inserts the flags from `other` into `self`.
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }

    /// Removes the flags from `other` from `self`.
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.0;
    }
}

impl BitOr for SplitFlags
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAnd for SplitFlags
{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl Not for SplitFlags
{
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl From<u8> for SplitFlags
{
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<SplitFlags> for u8
{
    fn from(value: SplitFlags) -> Self {
        value.0
    }
}