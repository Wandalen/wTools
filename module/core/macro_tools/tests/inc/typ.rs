
use super::*;

//

#[test]
fn test_is_optional_with_option_type()
{
  use syn::parse_str;
  use macro_tools::typ::is_optional;

  let type_string = "Option<i32>";
  let parsed_type: syn::Type = parse_str(type_string).expect("Type should parse correctly");

  assert!(is_optional(&parsed_type), "Expected type to be recognized as an Option");
}

#[test]
fn test_is_optional_with_non_option_type()
{
  use syn::parse_str;
  use macro_tools::typ::is_optional;

  let type_string = "Vec<i32>";
  let parsed_type: syn::Type = parse_str(type_string).expect("Type should parse correctly");

  assert!(!is_optional(&parsed_type), "Expected type not to be recognized as an Option");
}

#[test]
fn test_is_optional_with_nested_option_type()
{
  use syn::parse_str;
  use macro_tools::typ::is_optional;

  let type_string = "Option<Option<i32>>";
  let parsed_type: syn::Type = parse_str(type_string).expect("Type should parse correctly");

  assert!(is_optional(&parsed_type), "Expected nested Option type to be recognized as an Option");
}

#[test]
fn test_is_optional_with_similar_name_type()
{
  use syn::parse_str;
  use macro_tools::typ::is_optional;

  let type_string = "OptionalValue";
  let parsed_type: syn::Type = parse_str(type_string).expect("Type should parse correctly");

  assert!(!is_optional(&parsed_type), "Expected type with similar name not to be recognized as an Option");
}

#[test]
fn test_is_optional_with_empty_input()
{
  use syn::{parse_str, Type};
  use macro_tools::typ::is_optional;

  let type_string = "";
  let parsed_type_result = parse_str::<Type>(type_string);

  assert!(parsed_type_result.is_err(), "Expected parsing to fail for empty input");
}
