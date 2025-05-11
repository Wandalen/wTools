/// Shared test logic for unit variants in generic enums.

use super::*;

#[test]
fn generic_static_constructor()
{
  // Test Matrix Row: T4.1 / T4.2
  assert_eq!(GenericOption::<i32>::unit_none(), GenericOption::<i32>::UnitNone);
  assert_eq!(GenericOption::<String>::unit_none(), GenericOption::<String>::UnitNone);
}

#[test]
fn generic_standalone_constructor()
{
  // Test Matrix Row: T4.2
  assert_eq!(unit_none::<i32>(), GenericOption::<i32>::UnitNone);
  assert_eq!(unit_none::<String>(), GenericOption::<String>::UnitNone);
}