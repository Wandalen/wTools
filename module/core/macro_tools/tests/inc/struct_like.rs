
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

  // - unit

  let item : syn::ItemEnum = parse_quote!
  {
    struct Unit;
  };
  let exp = struct_like::StructLike::Unit( item );

  let got : struct_like::StructLike = parse_quote!
  {
    struct Unit;
  };
  a_id!( got, exp );

}
