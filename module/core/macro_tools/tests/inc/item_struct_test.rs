
use super::*;

#[ test ]
fn field_names_with_named_fields()
{
  use syn::parse_quote;
  use the_module::item_struct::field_names;

  let item_struct : syn::ItemStruct = parse_quote!
  {
    struct Test
    {
      a : i32,
      b : String,
    }
  };

  let names = field_names( &item_struct );
  assert!( names.is_some(), "Expected to extract field names" );
  let names : Vec< _ > = names.unwrap().collect();
  assert_eq!( names.len(), 2, "Expected two field names" );
  assert_eq!( names[ 0 ], "a", "First field name mismatch" );
  assert_eq!( names[ 1 ], "b", "Second field name mismatch" );
}

#[ test ]
fn field_names_with_unnamed_fields()
{
  use syn::parse_quote;
  use the_module::item_struct::field_names;

  let item_struct : syn::ItemStruct = parse_quote!
  {
    struct Test( i32, String );
  };

  let names = field_names( &item_struct );
  assert!( names.is_none(), "Expected None for unnamed fields" );
}

#[ test ]
fn field_names_with_unit_struct()
{
  use syn::parse_quote;
  use the_module::item_struct::field_names;

  let item_struct : syn::ItemStruct = parse_quote!
  {
    struct Test;
  };

  let names = field_names( &item_struct );
  assert!( names.is_some() );
  let names : Vec< _ > = names.unwrap().collect();
  assert_eq!( names.len(), 0 );

}

#[ test ]
fn field_names_with_reserved_keywords()
{
  use syn::parse_quote;
  use the_module::item_struct::field_names;

  let item_struct : syn::ItemStruct = parse_quote!
  {
    struct Test
    {
      r#type : i32,
      r#fn : String,
    }
  };

  let names = field_names( &item_struct );
  assert!( names.is_some(), "Expected to extract field names" );
  let names : Vec< _ > = names.unwrap().collect();
  assert_eq!( names.len(), 2, "Expected two field names" );
  assert_eq!( names[ 0 ], &syn::Ident::new_raw( "type", proc_macro2::Span::call_site() ), "First field name mismatch" );
  assert_eq!( names[ 1 ], &syn::Ident::new_raw( "fn", proc_macro2::Span::call_site() ), "Second field name mismatch" );

}

#[ test ]
fn test_field_or_variant_field()
{
  let input : proc_macro2::TokenStream = quote::quote!
  {
    struct MyStruct
    {
      my_field : i32,
    }
  };

  let ast : syn::ItemStruct = syn::parse2( input ).unwrap();
  let field = ast.fields.iter().next().unwrap();
  let field_or_variant = the_module::struct_like::FieldOrVariant::from( field );

  match field_or_variant
  {
    the_module::struct_like::FieldOrVariant::Field( f ) =>
    {
      assert_eq!( f.ty, syn::parse_quote!( i32 ) );
    },
    _ => panic!( "Expected Field variant" ),
  }
}

#[ test ]
fn test_field_or_variant_variant()
{
  let input : proc_macro2::TokenStream = quote::quote!
  {
    enum MyEnum
    {
      Variant1,
    }
  };

  let ast : syn::ItemEnum = syn::parse2( input ).unwrap();
  let variant = ast.variants.iter().next().unwrap();
  let field_or_variant = the_module::struct_like::FieldOrVariant::from( variant );

  match field_or_variant
  {
    the_module::struct_like::FieldOrVariant::Variant( v ) =>
    {
      let exp : syn::Ident = syn::parse_quote!( Variant1 );
      assert_eq!( v.ident, exp );
    },
    _ => panic!( "Expected Variant variant" ),
  }
}

#[ test ]
fn test_typ()
{
  let input : proc_macro2::TokenStream = quote::quote!
  {
    struct MyStruct
    {
      my_field : i32,
    }
  };

  let ast : syn::ItemStruct = syn::parse2( input ).unwrap();
  let field = ast.fields.iter().next().unwrap();
  let field_or_variant = the_module::struct_like::FieldOrVariant::from( field );
  assert_eq!( field_or_variant.typ(), Some( &syn::parse_quote!( i32 ) ) );
}

#[ test ]
fn test_attrs()
{
  let input : proc_macro2::TokenStream = quote::quote!
  {
    struct MyStruct
    {
      #[ some_attr ]
      my_field : i32,
    }
  };

  let ast : syn::ItemStruct = syn::parse2( input ).unwrap();
  let field = ast.fields.iter().next().unwrap();
  let field_or_variant = the_module::struct_like::FieldOrVariant::from( field );
  assert!( field_or_variant.attrs().iter().any( | attr | attr.path().is_ident( "some_attr" ) ) );
}

#[ test ]
fn test_vis()
{
  let input : proc_macro2::TokenStream = quote::quote!
  {
    struct MyStruct
    {
      pub my_field : i32,
    }
  };

  let ast : syn::ItemStruct = syn::parse2( input ).unwrap();
  let field = ast.fields.iter().next().unwrap();
  let field_or_variant = the_module::struct_like::FieldOrVariant::from( field );
  assert!( matches!( field_or_variant.vis(), Some( syn::Visibility::Public( _ ) ) ) );
}

#[ test ]
fn test_ident()
{
  let input : proc_macro2::TokenStream = quote::quote!
  {
    struct MyStruct
    {
      my_field : i32,
    }
  };

  let ast : syn::ItemStruct = syn::parse2( input ).unwrap();
  let field = ast.fields.iter().next().unwrap();
  let field_or_variant = the_module::struct_like::FieldOrVariant::from( field );
  assert_eq!( field_or_variant.ident().unwrap(), "my_field" );
}
