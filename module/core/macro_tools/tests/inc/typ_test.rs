
use super::*;
use the_module::qt;

//

#[ test ]
fn is_optional_with_option_type()
{
  use syn::parse_str;
  use the_module::typ::is_optional;

  let type_string = "Option<i32>";
  let parsed_type : syn::Type = parse_str( type_string ).expect( "Type should parse correctly" );

  assert!( is_optional( &parsed_type ), "Expected type to be recognized as an Option" );
}

#[ test ]
fn is_optional_with_non_option_type()
{
  use syn::parse_str;
  use the_module::typ::is_optional;

  let type_string = "Vec<i32>";
  let parsed_type : syn::Type = parse_str( type_string ).expect( "Type should parse correctly" );

  assert!( !is_optional( &parsed_type ), "Expected type not to be recognized as an Option" );
}

#[ test ]
fn is_optional_with_nested_option_type()
{
  use syn::parse_str;
  use the_module::typ::is_optional;

  let type_string = "Option<Option<i32>>";
  let parsed_type : syn::Type = parse_str( type_string ).expect( "Type should parse correctly" );

  assert!( is_optional( &parsed_type ), "Expected nested Option type to be recognized as an Option" );
}

#[ test ]
fn is_optional_with_similar_name_type()
{
  use syn::parse_str;
  use the_module::typ::is_optional;

  let type_string = "OptionalValue";
  let parsed_type : syn::Type = parse_str( type_string ).expect( "Type should parse correctly" );

  assert!( !is_optional( &parsed_type ), "Expected type with similar name not to be recognized as an Option" );
}

#[ test ]
fn is_optional_with_empty_input()
{
  use syn::{ parse_str, Type };
  use the_module::typ::is_optional;

  let type_string = "";
  let parsed_type_result = parse_str::< Type >( type_string );

  assert!( parsed_type_result.is_err(), "Expected parsing to fail for empty input" );
}

//

#[ test ]
fn parameter_first_with_multiple_generics()
{
  use syn::{ parse_str, Type };
  use the_module::typ::parameter_first;

  let type_string = "Result<Option<i32>, Error>";
  let parsed_type : Type = parse_str( type_string ).expect( "Type should parse correctly" );

  let first_param = parameter_first( &parsed_type ).expect( "Expected to extract the first generic parameter" );

  let expected_type : Type = parse_str( "Option<i32>" ).expect( "Expected type to parse correctly" );
  assert_eq!( format!( "{:?}", expected_type ), format!( "{:?}", first_param ), "Extracted type does not match expected" );
}

#[ test ]
fn parameter_first_with_no_generics()
{
  use syn::{ parse_str, Type };
  use the_module::typ::parameter_first;

  let type_string = "i32";
  let parsed_type : Type = parse_str( type_string ).expect( "Type should parse correctly" );
  let got = parameter_first( &parsed_type ).expect( "Type should parse correctly" );

  // tree_print!( got.as_ref().unwrap() );

  let expected_type : Type = parse_str( "i32" ).expect( "Expected type to parse correctly" );
  assert_eq!( format!( "{:?}", expected_type ), format!( "{:?}", got ), "Extracted type does not match expected" );

}

#[ test ]
fn parameter_first_with_single_generic()
{
  use syn::{ parse_str, Type };
  use the_module::typ::parameter_first;

  let type_string = "Vec< i32 >";
  let parsed_type : Type = parse_str( type_string ).expect( "Type should parse correctly" );

  let first_param = parameter_first( &parsed_type ).expect( "Expected to extract the first generic parameter" );

  let expected_type : Type = parse_str( "i32" ).expect( "Expected type to parse correctly" );
  assert_eq!( format!( "{:?}", expected_type ), format!( "{:?}", first_param ), "Extracted type does not match expected" );
}

#[ test ]
fn parameter_first_with_deeply_nested_generics()
{
  use syn::{ parse_str, Type };
  use the_module::typ::parameter_first;

  let type_string = "Vec< HashMap< String, Option< i32 >  > >";
  let parsed_type : Type = parse_str( type_string ).expect( "Type should parse correctly" );

  let first_param = parameter_first( &parsed_type ).expect( "Expected to extract the first generic parameter" );

  let expected_type : Type = parse_str( "HashMap< String, Option< i32 > >" ).expect( "Expected type to parse correctly" );
  assert_eq!( format!( "{:?}", expected_type ), format!( "{:?}", first_param ), "Extracted type does not match expected" );
}

//

#[ test ]
fn type_rightmost_basic()
{

  // test.case( "core::option::Option< i32 >" );
  let code = qt!( core::option::Option< i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = the_module::typ::type_rightmost( &tree_type );
  a_id!( got, Some( "Option".to_string() ) );

}

//

#[ test ]
fn type_parameters_basic()
{

  macro_rules! q
  {
    ( $( $Src : tt )+ ) =>
    {
      syn::parse2::< syn::Type >( qt!( $( $Src )+ ) ).unwrap()
    }
  }

  // test.case( "core::option::Option< i8, i16, i32, i64 >" );
  let code = qt!( core::option::Option< i8, i16, i32, i64 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();

  let got : Vec< syn::Type > = the_module::typ::type_parameters( &tree_type, 0..=0 ).into_iter().cloned().collect();
  let exp = vec![ q!( i8 ) ];
  a_id!( got, exp );
  let got : Vec< syn::Type > = the_module::typ::type_parameters( &tree_type, 0..=1 ).into_iter().cloned().collect();
  let exp = vec![ q!( i8 ), q!( i16 ) ];
  a_id!( got, exp );
  let got : Vec< syn::Type > = the_module::typ::type_parameters( &tree_type, 0..=2 ).into_iter().cloned().collect();
  let exp = vec![ q!( i8 ), q!( i16 ), q!( i32 ) ];
  a_id!( got, exp );

  let got : Vec< syn::Type > = the_module::typ::type_parameters( &tree_type, 0..0 ).into_iter().cloned().collect();
  let exp : Vec< syn::Type > = vec![];
  a_id!( got, exp );
  let got : Vec< syn::Type > = the_module::typ::type_parameters( &tree_type, 0..1 ).into_iter().cloned().collect();
  let exp = vec![ q!( i8 ) ];
  a_id!( got, exp );
  let got : Vec< syn::Type > = the_module::typ::type_parameters( &tree_type, 0..2 ).into_iter().cloned().collect();
  let exp = vec![ q!( i8 ), q!( i16 ) ];
  a_id!( got, exp );

  // unbound
  let got : Vec< syn::Type > = the_module::typ::type_parameters( &tree_type, .. ).into_iter().cloned().collect();
  let exp = vec![ q!( i8 ), q!( i16 ), q!( i32 ), q!( i64 ) ];
  a_id!( got, exp );

  let got : Vec< syn::Type > = the_module::typ::type_parameters( &tree_type, .. ).into_iter().cloned().collect();
  let exp = vec![ q!( i8 ), q!( i16 ), q!( i32 ), q!( i64 ) ];
  a_id!( got, exp );

  let got : Vec< syn::Type > = the_module::typ::type_parameters( &tree_type, .. ).into_iter().cloned().collect();
  let exp = vec![ q!( i8 ), q!( i16 ), q!( i32 ), q!( i64 ) ];
  a_id!( got, exp );

}
