
use super::*;

#[ test ]
fn basic()
{
  use syn::{ parse_quote, ItemStruct };
  use the_module::struct_like;

  // - struct

  let item : ItemStruct = parse_quote!
  {
    struct Example
    {
      field1 : i32,
      field2 : String
    }
  };
  let exp = struct_like::StructLike::Struct( item );

  let got : struct_like::StructLike = parse_quote!
  {
    struct Example
    {
      field1 : i32,
      field2 : String
    }
  };
  a_id!( got, exp );

  // - pub struct

  let item : ItemStruct = parse_quote!
  {
    pub( crate ) struct Example
    {
      field1 : i32,
      field2 : String
    }
  };
  let exp = struct_like::StructLike::Struct( item );

  let got : struct_like::StructLike = parse_quote!
  {
    pub( crate ) struct Example
    {
      field1 : i32,
      field2 : String
    }
  };
  a_id!( got, exp );

  // - enum

  let item : syn::ItemEnum = parse_quote!
  {
    enum Example
    {
      field1,
      field2( i32 ),
    }
  };
  let exp = struct_like::StructLike::Enum( item );

  let got : struct_like::StructLike = parse_quote!
  {
    enum Example
    {
      field1,
      field2( i32 ),
    }
  };
  a_id!( got, exp );

  // - pub enum

  let item : syn::ItemEnum = parse_quote!
  {
    pub( crate ) enum Example
    {
      field1,
      field2( i32 ),
    }
  };
  let exp = struct_like::StructLike::Enum( item );

  let got : struct_like::StructLike = parse_quote!
  {
    pub( crate ) enum Example
    {
      field1,
      field2( i32 ),
    }
  };
  a_id!( got, exp );

  // - unit

  let item : syn::ItemStruct = parse_quote!
  {
    struct Unit;
  };
  let exp = struct_like::StructLike::Unit( item );

  let got : struct_like::StructLike = parse_quote!
  {
    struct Unit;
  };
  a_id!( got, exp );

  // - pub unit

  let item : syn::ItemStruct = parse_quote!
  {
    pub( crate ) struct Unit;
  };
  let exp = struct_like::StructLike::Unit( item );

  let got : struct_like::StructLike = parse_quote!
  {
    pub( crate ) struct Unit;
  };
  a_id!( got, exp );

// xxx

}

//

#[ test ]
fn structlike_unit_struct()
{
  use syn::parse_quote;
  use the_module::struct_like::StructLike;

  let item_struct : syn::ItemStruct = parse_quote!
  {
    struct UnitStruct;
  };

  let struct_like = StructLike::from( item_struct );

  assert!( matches!( struct_like, StructLike::Unit( _ ) ), "Expected StructLike::Unit variant" );
  assert_eq!( struct_like.ident().to_string(), "UnitStruct", "Struct name mismatch" );
}

#[ test ]
fn structlike_struct()
{
  use syn::parse_quote;
  use the_module::struct_like::StructLike;

  let item_struct : syn::ItemStruct = parse_quote!
  {
    struct RegularStruct
    {
      a : i32,
      b : String,
    }
  };

  let struct_like = StructLike::from( item_struct );

  assert!( matches!( struct_like, StructLike::Struct( _ ) ), "Expected StructLike::Struct variant" );
  assert_eq!( struct_like.ident().to_string(), "RegularStruct", "Struct name mismatch" );
  assert_eq!( struct_like.fields().count(), 2, "Expected two fields" );
}

#[ test ]
fn structlike_enum()
{
  use syn::parse_quote;
  use the_module::struct_like::StructLike;

  let item_enum : syn::ItemEnum = parse_quote!
  {
    enum TestEnum
    {
      Variant1,
      Variant2 { x : i32, y : String },
    }
  };

  let struct_like = StructLike::from( item_enum );

  assert!( matches!( struct_like, StructLike::Enum( _ ) ), "Expected StructLike::Enum variant" );
  assert_eq!( struct_like.ident().to_string(), "TestEnum", "Enum name mismatch" );
}
