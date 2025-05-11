/// Shared test logic for unit variants in enums with mixed variant kinds.

use super::*;

#[test]
fn mixed_static_constructor()
{
  // Test Matrix Row: T5.1 / T5.2
  assert_eq!(MixedEnum::simple_unit(), MixedEnum::SimpleUnit);
}

#[test]
fn mixed_standalone_constructor() // Restore test
{
  // Test Matrix Row: T5.2
  assert_eq!(simple_unit(), MixedEnum::SimpleUnit);
}