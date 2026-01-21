use super :: *;
use the_module ::qt;

//
// | TC011 | Test type parameter extraction with various range patterns | `type_parameters_basic` |

//

#[ test ]
fn is_optional_with_option_type() 
{
  use syn ::parse_str;
  use the_module ::typ ::is_optional;

  let type_string = "Option< i32 >";
  let parsed_type: syn ::Type = parse_str(type_string).expect("Type should parse correctly");

  assert!(is_optional(&parsed_type), "Expected type to be recognized as an Option");
}

#[ test ]
fn is_optional_with_non_option_type() 
{
  use syn ::parse_str;
  use the_module ::typ ::is_optional;

  let type_string = "Vec< i32 >";
  let parsed_type: syn ::Type = parse_str(type_string).expect("Type should parse correctly");

  assert!(!is_optional(&parsed_type), "Expected type not to be recognized as an Option");
}

#[ test ]
fn is_optional_with_nested_option_type() 
{
  use syn ::parse_str;
  use the_module ::typ ::is_optional;

  let type_string = "Option< Option<i32 >>";
  let parsed_type: syn ::Type = parse_str(type_string).expect("Type should parse correctly");

  assert!(
  is_optional(&parsed_type),
  "Expected nested Option type to be recognized as an Option"
 );
}

#[ test ]
fn is_optional_with_similar_name_type() 
{
  use syn ::parse_str;
  use the_module ::typ ::is_optional;

  let type_string = "OptionalValue";
  let parsed_type: syn ::Type = parse_str(type_string).expect("Type should parse correctly");

  assert!(
  !is_optional(&parsed_type),
  "Expected type with similar name not to be recognized as an Option"
 );
}

#[ test ]
fn is_optional_with_empty_input() 
{
  use syn :: { parse_str, Type };
  use the_module ::typ ::is_optional;

  let type_string = "";
  let parsed_type_result = parse_str :: < Type >(type_string);

  assert!(parsed_type_result.is_err(), "Expected parsing to fail for empty input");
}

//

#[ test ]
fn parameter_first_with_multiple_generics() 
{
  use syn :: { parse_str, Type };
  use the_module ::typ ::parameter_first;

  let type_string = "Result< Option<i32 >, Error>";
  let parsed_type: Type = parse_str(type_string).expect("Type should parse correctly");

  let first_param = parameter_first(&parsed_type).expect("Expected to extract the first generic parameter");

  let expected_type: Type = parse_str("Option< i32 >").expect("Expected type to parse correctly");
  assert_eq!(
  format!("{expected_type:?}"),
  format!("{:?}", first_param),
  "Extracted type does not match expected"
 );
}

#[ test ]
fn parameter_first_with_no_generics() 
{
  use syn :: { parse_str, Type };
  use the_module ::typ ::parameter_first;

  let type_string = "i32";
  let parsed_type: Type = parse_str(type_string).expect("Type should parse correctly");
  let got = parameter_first(&parsed_type).expect("Type should parse correctly");

  // tree_print!( got.as_ref().unwrap() );

  let expected_type: Type = parse_str("i32").expect("Expected type to parse correctly");
  assert_eq!(
  format!("{expected_type:?}"),
  format!("{:?}", got),
  "Extracted type does not match expected"
 );
}

#[ test ]
fn parameter_first_with_single_generic() 
{
  use syn :: { parse_str, Type };
  use the_module ::typ ::parameter_first;

  let type_string = "Vec<  i32  >";
  let parsed_type: Type = parse_str(type_string).expect("Type should parse correctly");

  let first_param = parameter_first(&parsed_type).expect("Expected to extract the first generic parameter");

  let expected_type: Type = parse_str("i32").expect("Expected type to parse correctly");
  assert_eq!(
  format!("{expected_type:?}"),
  format!("{:?}", first_param),
  "Extracted type does not match expected"
 );
}

#[ test ]
fn parameter_first_with_deeply_nested_generics() 
{
  use syn :: { parse_str, Type };
  use the_module ::typ ::parameter_first;

  let type_string = "Vec< HashMap< String, Option< i32 > > >";
  let parsed_type: Type = parse_str(type_string).expect("Type should parse correctly");

  let first_param = parameter_first(&parsed_type).expect("Expected to extract the first generic parameter");

  let expected_type: Type = parse_str("HashMap< String, Option< i32 > >").expect("Expected type to parse correctly");
  assert_eq!(
  format!("{expected_type:?}"),
  format!("{:?}", first_param),
  "Extracted type does not match expected"
 );
}

//

#[ test ]
fn type_rightmost_basic() 
{
  // test.case( "core ::option ::Option< i32 >" );
  let code = qt!(core ::option ::Option< i32 >);
  let tree_type = syn ::parse2 :: < syn ::Type >(code).unwrap();
  let got = the_module ::typ ::type_rightmost(&tree_type);
  assert_eq!(got, Some("Option".to_string()));
}

//

#[ test ]
fn type_parameters_basic() 
{
  macro_rules! q
  {
  ( $( $Src: tt )+ ) =>
  {
   syn ::parse2 :: < syn ::Type >( qt!( $( $Src )+ ) ).unwrap()
 }
 }

  // test.case( "core ::option ::Option< i8, i16, i32, i64 >" );
  let code = qt!( core ::option ::Option< i8, i16, i32, i64 > );
  let tree_type = syn ::parse2 :: < syn ::Type >(code).unwrap();

  let got: Vec< syn ::Type > = the_module ::typ ::type_parameters(&tree_type, 0..=0)
  .into_iter()
  .cloned()
  .collect();
  let exp = vec![q!(i8)];
  assert_eq!(got, exp);
  let got: Vec< syn ::Type > = the_module ::typ ::type_parameters(&tree_type, 0..=1)
  .into_iter()
  .cloned()
  .collect();
  let exp = vec![q!(i8), q!(i16)];
  assert_eq!(got, exp);
  let got: Vec< syn ::Type > = the_module ::typ ::type_parameters(&tree_type, 0..=2)
  .into_iter()
  .cloned()
  .collect();
  let exp = vec![q!(i8), q!(i16), q!(i32)];
  assert_eq!(got, exp);

  let got: Vec< syn ::Type > = the_module ::typ ::type_parameters(&tree_type, 0..0)
  .into_iter()
  .cloned()
  .collect();
  let exp: Vec< syn ::Type > = vec![];
  assert_eq!(got, exp);
  let got: Vec< syn ::Type > = the_module ::typ ::type_parameters(&tree_type, 0..1)
  .into_iter()
  .cloned()
  .collect();
  let exp = vec![q!(i8)];
  assert_eq!(got, exp);
  let got: Vec< syn ::Type > = the_module ::typ ::type_parameters(&tree_type, 0..2)
  .into_iter()
  .cloned()
  .collect();
  let exp = vec![q!(i8), q!(i16)];
  assert_eq!(got, exp);

  // unbound
  let got: Vec< syn ::Type > = the_module ::typ ::type_parameters(&tree_type, ..)
  .into_iter()
  .cloned()
  .collect();
  let exp = vec![q!(i8), q!(i16), q!(i32), q!(i64)];
  assert_eq!(got, exp);

  let got: Vec< syn ::Type > = the_module ::typ ::type_parameters(&tree_type, ..)
  .into_iter()
  .cloned()
  .collect();
  let exp = vec![q!(i8), q!(i16), q!(i32), q!(i64)];
  assert_eq!(got, exp);

  let got: Vec< syn ::Type > = the_module ::typ ::type_parameters(&tree_type, ..)
  .into_iter()
  .cloned()
  .collect();
  let exp = vec![q!(i8), q!(i16), q!(i32), q!(i64)];
  assert_eq!(got, exp);
}

//
// | TC012 | Test nested type parameter extraction without panicking | `type_parameters_nested_extraction` |
//

// Fix(issue-catch_unwind_removal)
// Root cause: Example used std::panic::catch_unwind for error handling when extracting
// nested type parameters, but type_parameters() is infallible and never panics.
// This created bad pattern in example code teaching users to use panic handling for control flow.
// Pitfall: Using catch_unwind when function signature shows it returns Vec (never fails).
// Always check function signature first - if it returns T instead of Result<T>, it won't panic
// except for logic bugs. Don't add defensive panic catching without understanding failure modes.

#[ test ]
fn type_parameters_nested_extraction_no_panic()
{
  use syn ::parse_str;
  macro_rules! q
  {
  ( $( $Src: tt )+ ) =>
  {
   syn ::parse2 :: < syn ::Type >( qt!( $( $Src )+ ) ).unwrap()
 }
 }

  // Test that we can extract parameters from nested generic types
  // without any panic handling (validates fix for catch_unwind removal)
  let code = qt!( Result< Option< String >, std ::io ::Error > );
  let tree_type = syn ::parse2 :: < syn ::Type >(code).unwrap();

  // Extract outer type parameters
  let outer_params = the_module ::typ ::type_parameters(&tree_type, 0..=1);
  assert_eq!(outer_params.len(), 2);

  // Extract inner parameters from first param (Option<String>)
  // This should work without any catch_unwind
  let first_param = outer_params[0];
  let inner_params = the_module ::typ ::type_parameters(first_param, 0..=0);
  assert_eq!(inner_params.len(), 1);

  // Verify the inner parameter is String
  let expected = q!(String);
  assert_eq!(format!("{:?}", inner_params[0]), format!("{:?}", expected));
}

#[ test ]
fn type_parameters_non_generic_inner_type()
{
  // Test extracting parameters from non-generic inner types
  // to validate no panic occurs when inner type has no parameters
  macro_rules! q
  {
  ( $( $Src: tt )+ ) =>
  {
   syn ::parse2 :: < syn ::Type >( qt!( $( $Src )+ ) ).unwrap()
 }
 }

  let code = qt!( Result< String, std ::io ::Error > );
  let tree_type = syn ::parse2 :: < syn ::Type >(code).unwrap();

  // Extract outer parameters
  let outer_params = the_module ::typ ::type_parameters(&tree_type, 0..=1);
  assert_eq!(outer_params.len(), 2);

  // Try to extract inner parameters from String (has none)
  // Should return empty vector, not panic
  let first_param = outer_params[0];
  let inner_params = the_module ::typ ::type_parameters(first_param, 0..=0);

  // String is not a generic type, so type_parameters returns the type itself
  assert_eq!(inner_params.len(), 1);
  let expected = q!(String);
  assert_eq!(format!("{:?}", inner_params[0]), format!("{:?}", expected));
}
