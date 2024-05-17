
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
  let names = names.unwrap();
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
  assert!( names.is_none(), "Expected None for unit struct" );
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
  let names = names.unwrap();
  assert_eq!( names.len(), 2, "Expected two field names" );
  assert_eq!( names[ 0 ], "type", "First field name mismatch" );
  assert_eq!( names[ 1 ], "fn", "Second field name mismatch" );
}
