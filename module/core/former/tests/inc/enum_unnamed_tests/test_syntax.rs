#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
#![allow(dead_code)] // Test structures are intentionally unused
use super::*;

#[ derive( Debug, PartialEq, Clone ) ]
pub enum TestEnum<T: Bound>
{
  Variant1(InnerScalar<T>),
}