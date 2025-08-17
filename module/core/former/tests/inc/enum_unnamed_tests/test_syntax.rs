#![allow(dead_code)] // Test structures are intentionally unused
use super::*;

#[ derive( Debug, PartialEq, Clone ) ]
pub enum TestEnum<T: Bound>
{
  Variant1(InnerScalar<T>),
}