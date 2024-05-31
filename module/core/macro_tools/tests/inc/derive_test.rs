
use super::*;

//

#[test]
fn named_fields_with_named_fields()
{
  use syn::{parse_quote, punctuated::Punctuated, Field, token::Comma};
  use the_module::derive;

  let ast: syn::DeriveInput = parse_quote!
  {
    struct Test
    {
      a : i32,
      b : String,
    }
  };

  let result = derive::named_fields( &ast ).expect( "Expected successful extraction of named fields" );

  let mut expected_fields = Punctuated::new();
  let field_a : Field = parse_quote! { a: i32 };
  let field_b : Field = parse_quote! { b: String };
  expected_fields.push_value( field_a);
  expected_fields.push_punct( Comma::default() );
  expected_fields.push_value( field_b );
  expected_fields.push_punct( Comma::default() );

  a_id!( format!( "{:?}", result ), format!( "{:?}", expected_fields ), "Fields did not match expected output" );
}

//

#[ test ]
fn named_fields_with_tuple_struct()
{
  use syn::{ parse_quote };
  use the_module::derive::named_fields;

  let ast : syn::DeriveInput = parse_quote!
  {
    struct Test( i32, String );
  };

  let result = named_fields( &ast );

  assert!( result.is_err(), "Expected an error for tuple struct, but extraction was successful" );
}

//

#[ test ]
fn named_fields_with_enum()
{
  use syn::{ parse_quote };
  use the_module::derive::named_fields;

  let ast : syn::DeriveInput = parse_quote!
  {
    enum Test
    {
      Variant1,
      Variant2,
    }
  };

  let result = named_fields( &ast );

  assert!( result.is_err(), "Expected an error for enum, but extraction was successful" );
}
