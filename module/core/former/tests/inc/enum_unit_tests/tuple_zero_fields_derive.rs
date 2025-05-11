//! Derive implementation for testing zero-field tuple variants.
use super::*; // To find the _only_test.rs file via include
use former::Former;

// For Test Matrix Row: T8.1 (Default behavior)
#[derive(Debug, PartialEq, Former)]
#[former(standalone_constructors, debug)] // Add debug for diagnostics if needed
pub enum ZeroTuple { Variant() }

// For Test Matrix Row: T8.2 (#[scalar] attribute)
// Default and scalar behavior for zero-field tuples are identical (Rule 1b, 3b)
// The 'scalar' key is not valid inside #[former(...)].
// If variant-specific scalar behavior was different and intended, it would be #[scalar] on the Variant.
#[derive(Debug, PartialEq, Former)]
#[former(standalone_constructors, debug)] // Removed invalid 'scalar' key
pub enum ZeroTupleScalar { Variant() }

include!("tuple_zero_fields_only_test.rs");