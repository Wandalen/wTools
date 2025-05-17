//! Defines the core instruction and argument structures for unilang.

use crate::error::SourceLocation;
use std::borrow::Cow;
use std::collections::HashMap;

/// Represents an argument to a unilang instruction.
#[derive(Debug, PartialEq, Clone)]
pub struct Argument<'a> {
    /// The raw slice of the argument's name, if it's a named argument.
    pub name_slice: Option<&'a str>,
    /// The unescaped value of the argument.
    pub value: Cow<'a, str>,
    /// The location of the argument's name, if applicable.
    pub name_location: Option<SourceLocation>,
    /// The location of the argument's value.
    pub value_location: SourceLocation,
}

/// Represents a generic unilang instruction.
#[derive(Debug, PartialEq, Clone)]
pub struct GenericInstruction<'a> {
    /// The sequence of slices forming the command path.
    pub command_path_slices: Vec<&'a str>,
    /// Named arguments, mapped by their raw name slice.
    pub named_arguments: HashMap<&'a str, Argument<'a>>,
    /// Positional arguments, in the order they appear.
    pub positional_arguments: Vec<Argument<'a>>,
    /// Flag indicating if help was requested for this command (e.g., via a trailing '?').
    pub help_requested: bool,
    /// The overall location (span) of the entire instruction.
    pub overall_location: SourceLocation,
}