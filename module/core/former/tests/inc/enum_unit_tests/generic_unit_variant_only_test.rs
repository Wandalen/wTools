/// Shared test logic for unit variants in generic enums.
use super::*;

#[test]
fn generic_static_constructor()
{
  // Test Matrix Row: T4.1 / T4.2
  assert_eq!(GenericOption::<i32>::no_value(), GenericOption::<i32>::NoValue);
  assert_eq!(GenericOption::<String>::no_value(), GenericOption::<String>::NoValue);
}

#[test]
fn generic_standalone_constructor()
{
  // Test Matrix Row: T4.2
  assert_eq!(no_value::<i32>(), GenericOption::<i32>::NoValue);
  assert_eq!(no_value::<String>(), GenericOption::<String>::NoValue);
}