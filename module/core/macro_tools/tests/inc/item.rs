
use super::*;

#[ test ]
fn ensure_comma_named_struct_with_multiple_fields()
{
  use syn::{ parse_quote, ItemStruct };

  let input_struct : ItemStruct = parse_quote!
  {
    struct Example
    {
      field1 : i32,
      field2 : String
    }
  };

  let got = the_module::item::ensure_comma( &input_struct );
  // let exp = "struct Example { field1 : i32, field2 : String, }";
  let exp : syn::ItemStruct = parse_quote! {  struct Example { field1 : i32, field2 : String, } };
  // let got = quote!( #got ).to_string();
  // assert_eq!( exp, got );
  a_id!( got, exp );

}

#[ test ]
fn ensure_comma_named_struct_with_single_field()
{
  use syn::{ parse_quote, ItemStruct };

  let input_struct : ItemStruct = parse_quote!
  {
    struct Example
    {
      field1 : i32
    }
  };

  let got = the_module::item::ensure_comma( &input_struct );
  let exp : ItemStruct = parse_quote! { struct Example { field1 : i32, } };
  assert_eq!( got, exp );
}

#[ test ]
fn ensure_comma_named_struct_with_no_fields()
{
  use syn::{ parse_quote, ItemStruct };

  let input_struct : ItemStruct = parse_quote!
  {
    struct Example { }
  };

  let got = the_module::item::ensure_comma( &input_struct );
  let exp : ItemStruct = parse_quote! { struct Example { } };
  assert_eq!( got, exp );
}

#[ test ]
fn ensure_comma_unnamed_struct_with_multiple_fields()
{
  use syn::{ parse_quote, ItemStruct };

  let input_struct : ItemStruct = parse_quote!
  {
    struct Example( i32, String );
  };

  let got = the_module::item::ensure_comma( &input_struct );
  let exp : ItemStruct = parse_quote! { struct Example( i32, String, ); };
  assert_eq!( got, exp );
}

#[ test ]
fn ensure_comma_unnamed_struct_with_single_field()
{
  use syn::{ parse_quote, ItemStruct };

  let input_struct : ItemStruct = parse_quote!
  {
    struct Example( i32 );
  };

  let got = the_module::item::ensure_comma( &input_struct );
  let exp : ItemStruct = parse_quote! { struct Example( i32, ); };
  assert_eq!( got, exp );
}

#[ test ]
fn ensure_comma_unnamed_struct_with_no_fields()
{
  use syn::{ parse_quote, ItemStruct };

  let input_struct : ItemStruct = parse_quote!
  {
    struct Example( );
  };

  let got = the_module::item::ensure_comma( &input_struct );
  let exp : ItemStruct = parse_quote! { struct Example( ); };
  assert_eq!( got, exp );
}

#[ test ]
fn ensure_comma_unit_struct_with_no_fields()
{
  use syn::{ parse_quote, ItemStruct };

  let input_struct : ItemStruct = parse_quote!
  {
    struct Example;
  };

  let got = the_module::item::ensure_comma( &input_struct );
  let exp : ItemStruct = parse_quote! { struct Example; };
  assert_eq!( got, exp );
}
