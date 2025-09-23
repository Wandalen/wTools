#[allow(clippy::used_underscore_binding, clippy::all, warnings)]
/// Shared test logic for unit variants in enums with mixed variant kinds.
use super::*;

#[ test ]
fn mixed_static_constructor()
{
  assert_eq!(MixedEnum::simple_unit(), MixedEnum::SimpleUnit);
}

#[ test ]
fn mixed_standalone_constructor() // Test present
{
  assert_eq!(simple_unit(), MixedEnum::SimpleUnit);
}