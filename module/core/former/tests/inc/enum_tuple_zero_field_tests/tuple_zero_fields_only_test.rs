// Shared test logic for zero-field tuple variants.
use super::*;

// Test enum will be:
// pub enum ZeroTuple { Variant() }
// Or with #[scalar]:
// #[derive(Former)] #[former(scalar)] pub enum ZeroTupleScalar { Variant() }

#[test]
fn static_constructor_default()
{
  // Test Matrix Row: T8.1 (Default behavior)
  // Expects: ZeroTuple::variant() -> ZeroTuple
  assert_eq!( ZeroTuple::variant(), ZeroTuple::Variant() );
}

#[test]
fn standalone_constructor_default()
{
  // Test Matrix Row: T8.1 (Default behavior)
  // Expects: zero_tuple_variant() -> ZeroTuple
  assert_eq!( zero_tuple_variant(), ZeroTuple::Variant() );
}

#[test]
fn static_constructor_scalar()
{
  // Test Matrix Row: T8.2 (#[scalar] attribute)
  // Expects: ZeroTupleScalar::variant() -> ZeroTupleScalar
  assert_eq!( ZeroTupleScalar::variant(), ZeroTupleScalar::Variant() );
}

#[test]
fn standalone_constructor_scalar()
{
  // Test Matrix Row: T8.2 (#[scalar] attribute)
  // Expects: zero_tuple_scalar_variant() -> ZeroTupleScalar
  assert_eq!( zero_tuple_scalar_variant(), ZeroTupleScalar::Variant() );
}