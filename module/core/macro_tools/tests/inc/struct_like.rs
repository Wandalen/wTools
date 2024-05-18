
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

#[ test ]
fn struct_with_attrs()
{
  let input : proc_macro2::TokenStream = quote::quote!
  {
    #[ derive( From, InnerFrom, Display, FromStr, PartialEq, Debug ) ]
    #[ display( "{a}-{b}" ) ]
    struct Struct1
    {
      a : i32,
      b : i32,
    }
  };

  let ast : syn::ItemStruct = syn::parse2( input ).unwrap();
  let field = ast.fields.iter().next().unwrap();
  let field_or_variant = the_module::struct_like::FieldOrVariant::from( field );
  assert_eq!( field_or_variant.ident().unwrap(), "a" );
}
