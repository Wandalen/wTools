
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

  let struct_like : StructLike = parse_quote!
  {
    struct UnitStruct;
  };

  assert!( matches!( struct_like, StructLike::Unit( _ ) ), "Expected StructLike::Unit variant" );
  assert_eq!( struct_like.ident().to_string(), "UnitStruct", "Struct name mismatch" );
}

#[ test ]
fn structlike_struct()
{
  use syn::parse_quote;
  use the_module::struct_like::StructLike;

  let struct_like : StructLike = parse_quote!
  {
    struct RegularStruct
    {
      a : i32,
      b : String,
    }
  };

  assert!( matches!( struct_like, StructLike::Struct( _ ) ), "Expected StructLike::Struct variant" );
  assert_eq!( struct_like.ident().to_string(), "RegularStruct", "Struct name mismatch" );
  assert_eq!( struct_like.fields().count(), 2, "Expected two fields" );
}

#[ test ]
fn structlike_enum()
{
  use syn::parse_quote;
  use the_module::struct_like::StructLike;

  let struct_like : StructLike = parse_quote!
  {
    enum TestEnum
    {
      Variant1,
      Variant2 { x : i32, y : String },
    }
  };

  assert!( matches!( struct_like, StructLike::Enum( _ ) ), "Expected StructLike::Enum variant" );
  assert_eq!( struct_like.ident().to_string(), "TestEnum", "Enum name mismatch" );
}

#[ test ]
fn test_field_or_variant_field()
{
  use syn::parse_quote;
  use the_module::struct_like::{ FieldOrVariant, StructLike };

  let input : StructLike = parse_quote!
  {
    struct MyStruct
    {
      my_field : i32,
    }
  };

  let field = input.fields().next().expect( "Expected at least one field" );
  let field_or_variant = FieldOrVariant::from( field );

  match field_or_variant
  {
    FieldOrVariant::Field( f ) => assert_eq!( f.ty, parse_quote!( i32 ) ),
    _ => panic!( "Expected Field variant" ),
  }
}

#[ test ]
fn test_field_or_variant_variant()
{
  use syn::parse_quote;
  use the_module::struct_like::{ FieldOrVariant, StructLike };

  let input : StructLike = parse_quote!
  {
    enum MyEnum
    {
      Variant1,
    }
  };

  let variant = input.elements().next().expect( "Expected at least one variant" );
  let field_or_variant = FieldOrVariant::from( variant );

  match field_or_variant
  {
    FieldOrVariant::Variant( v ) =>
    {
      let exp : syn::Ident = parse_quote!( Variant1 );
      assert_eq!( v.ident, exp );
    },
    _ => panic!( "Expected Variant variant" ),
  }
}

#[ test ]
fn test_typ()
{
  use syn::parse_quote;
  use the_module::struct_like::{ FieldOrVariant, StructLike };

  let input : StructLike = parse_quote!
  {
    struct MyStruct
    {
      my_field : i32,
    }
  };

  let field = input.fields().next().expect( "Expected at least one field" );
  let field_or_variant = FieldOrVariant::from( field );
  assert_eq!( field_or_variant.typ(), Some( &parse_quote!( i32 ) ) );
}

#[ test ]
fn test_attrs()
{
  use syn::parse_quote;
  use the_module::struct_like::{ FieldOrVariant, StructLike };

  let input : StructLike = parse_quote!
  {
    struct MyStruct
    {
      #[ some_attr ]
      my_field : i32,
    }
  };

  let field = input.fields().next().expect( "Expected at least one field" );
  let field_or_variant = FieldOrVariant::from( field );
  assert!( field_or_variant.attrs().iter().any( | attr | attr.path().is_ident( "some_attr" ) ) );
}

#[ test ]
fn test_vis()
{
  use syn::parse_quote;
  use the_module::struct_like::{ FieldOrVariant, StructLike };

  let input : StructLike = parse_quote!
  {
    struct MyStruct
    {
      pub my_field : i32,
    }
  };

  let field = input.fields().next().expect( "Expected at least one field" );
  let field_or_variant = FieldOrVariant::from( field );
  assert!( matches!( field_or_variant.vis(), Some( syn::Visibility::Public( _ ) ) ) );
}

#[ test ]
fn test_ident()
{
  use the_module::struct_like::StructLike;
  use syn::parse_quote;
  use the_module::struct_like::FieldOrVariant;

  let input : StructLike = parse_quote!
  {
    struct MyStruct
    {
      my_field : i32,
    }
  };

  // Extract the first field using the fields iterator from StructLike
  let field = input.fields().next().expect( "Expected at least one field" );

  let field_or_variant = FieldOrVariant::from( field );
  assert_eq!( field_or_variant.ident().unwrap(), "my_field" );
}

//

#[ test ]
fn struct_with_attrs()
{
  use the_module::struct_like::StructLike;

  let input : proc_macro2::TokenStream = quote::quote!
  {
    #[ derive( From, InnerFrom, Display, FromStr, PartialEq, Debug ) ]
    #[ display( "{a}-{b}" ) ]
    pub struct Struct1
    {
      a : i32,
      b : i32,
    }
  };

  let ast : StructLike = syn::parse2( input ).unwrap();
  let field = ast.fields().next().unwrap();
  let field_or_variant = the_module::struct_like::FieldOrVariant::from( field );
  assert_eq!( field_or_variant.ident().unwrap(), "a" );
}

//

// #[ test ]
// fn struct_with_attrs2()
// {
//   use the_module::struct_like::StructLike;
//
//   let input : proc_macro2::TokenStream = quote::quote!
//   {
//     #[ derive( Debug, PartialEq, the_module::From ) ]
//     #[ debug ]
//     pub enum GetData
//     {
//       #[ allow( dead_code ) ]
//       Nothing,
//       FromString( String ),
//       FromBin( &'static [ u8 ] ),
//     }
//   };
//
//   let ast : StructLike = syn::parse2( input ).unwrap();
//   let field = ast.elements().next().unwrap();
//   let field_or_variant = the_module::struct_like::FieldOrVariant::from( field );
//   assert_eq!( field_or_variant.ident().unwrap().to_string(), "Nothing" );
//
// }

#[ test ]
fn struct_with_attrs2()
{
  use quote::ToTokens;
  use the_module::struct_like::{ StructLike, FieldOrVariant };

  let input : proc_macro2::TokenStream = quote::quote!
  {
    #[ derive( Debug, PartialEq, the_module::From ) ]
    #[ debug ]
    pub enum GetData
    {
      #[ allow( dead_code ) ]
      Nothing,
      FromString( String ),
      FromBin( & 'static [u8] ),
    }
  };

  // Parse the input into a StructLike enum
  let ast : StructLike = syn::parse2( input ).unwrap();

  // Ensure the parsed item is an enum
  assert!( matches!( ast, StructLike::Enum( _ ) ), "Expected StructLike::Enum variant" );

  // Check the attributes of the enum
  let attrs = ast.attrs();
  assert!( attrs.iter().any( | attr | attr.path().is_ident( "derive" ) ), "Missing derive attribute" );
  assert!( attrs.iter().any( | attr | attr.path().is_ident( "debug" ) ), "Missing debug attribute" );

  // Check the visibility of the enum
  assert!( matches!( ast.vis(), syn::Visibility::Public( _ ) ), "Expected public visibility" );

  // Check all elements
  let elements : Vec< FieldOrVariant< '_ > > = ast.elements().map( FieldOrVariant::from ).collect();

  // Check the first variant
  let first_field_or_variant = &elements[ 0 ];
  assert_eq!( first_field_or_variant.ident().unwrap().to_string(), "Nothing" );

  // Check the attributes of the first variant
  let variant_attrs = first_field_or_variant.attrs();
  assert!( variant_attrs.iter().any( | attr | attr.path().is_ident( "allow" ) ), "Missing allow attribute" );

  // Check all variant names
  let variant_names : Vec< String > = elements.iter().map( | elem | elem.ident().unwrap().to_string() ).collect();
  assert_eq!( variant_names, vec![ "Nothing", "FromString", "FromBin" ], "Variant names do not match" );

  // Check the types of the variants
  let variant_types : Vec< Option< &syn::Type > > = elements.iter().map( | elem | elem.typ() ).collect();

  // let variant_fields: Vec< syn::Fields > = ast.elements().map( | e | e.fields() ).collect();
  let variant_fields : Vec< syn::Fields > = elements.iter().filter_map( | elem | elem.fields().cloned() ).collect();
  // dbg!( &variant_types );

  assert_eq!( variant_types.len(), 3, "Expected three variants" );
  assert!( variant_types[ 0 ].is_none(), "First variant should have no type" );

  assert!( variant_types[ 0 ].is_none() );
  assert!( variant_types[ 1 ].is_none() );
  assert!( variant_types[ 2 ].is_none() );

  // tree_print!( variant_fields[1] );
  assert_eq!( variant_fields[ 1 ].to_token_stream().to_string(), "(String)", "Second variant should be of type String" );
  assert_eq!( variant_fields[ 2 ].to_token_stream().to_string(), "(& 'static [u8])", "Third variant should be of type & 'static [u8]" );
}
