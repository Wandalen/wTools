// #![ allow( dead_code ) ]
#![ feature( trace_macros ) ]

use test_tools::*;
use werror::*;
use select_matching as TheModule;

//

fn basic_test()
{

  #[ derive( Debug, Clone, PartialEq ) ]
  enum Enum1
  {
    A( Struct1a ),
    B( Struct1b ),
  }

  #[ derive( Debug, Clone, PartialEq ) ]
  struct Struct1a
  {
    k1 : Enum2,
    k2 : i32,
  }

  #[ derive( Debug, Clone, PartialEq ) ]
  struct Struct1b
  {
    k1 : i32,
    k2 : Enum2,
  }

  #[ derive( Debug, Clone, PartialEq ) ]
  enum Enum2
  {
    A( i32 ),
    B( String ),
  }

  // test.case( "one" );

  let got = manual1().unwrap();
  assert_eq!( got, Enum2::A( 1 ) );
  let got = auto1().unwrap();
  assert_eq!( got, Enum2::A( 1 ) );

  // test.case( "two" );

  let got = manual2().unwrap();
  assert_eq!( got, 1 );
  // let got = auto2().unwrap();
  // assert_eq!( got, 1 );

  /* */

  fn manual1() -> Result< Enum2, Error >
  {
    let src = Enum1::A( Struct1a { k1 : Enum2::A( 1 ), k2 : 3 } );
    let got = match src
    {
      Enum1::A( ref struct1a ) => struct1a.k1.clone(),
      _ => return Err( Error::new( "Unknown format, expected Enum1::A( ref struct1a )" ) ),
    };
    // dbg!( &got );
    Ok( got )
  }

  fn auto1() -> Result< Enum2, Error >
  {
    let src = Enum1::A( Struct1a { k1 : Enum2::A( 1 ), k2 : 3 } );
    // trace_macros!( true );
    let got = TheModule::select_matching!
    (
      src,
      return Err( Error::new( "Unknown format, expected Enum1::A( ref struct1a )" ) ),
      Enum1::A( ref struct1a ) => struct1a.k1.clone(),
    );
    // trace_macros!( false );
    // dbg!( &got );
    Ok( got )
  }

  fn manual2() -> Result< i32, Error >
  {
    let src = Enum1::A( Struct1a { k1 : Enum2::A( 1 ), k2 : 3 } );
    let got = match src
    {
      Enum1::A( struct1a ) => match struct1a.k1
      {
        Enum2::A( integer ) => integer,
        _ => return Err( Error::new( "Unknown format, expected Enum1::A( ref struct1a )" ) ),
      }
      _ => return Err( Error::new( "Unknown format, expected Enum1::A( ref struct1a )" ) ),
    };
    // dbg!( &got );
    Ok( got )
  }

  // fn auto2() -> Result< i32, Error >
  // {
  //   let src = Enum1::A( Struct1a { k1 : Enum2::A( 1 ), k2 : 3 } );
  //   trace_macros!( true );
  //   let got = TheModule::select_matching!
  //   (
  //     src,
  //     return Err( Error::new( "Unknown format, expected Enum1::A( ref struct1a )" ) ),
  //     Enum1::A( struct1a ) => struct1a.k1,
  //     Enum2::A( integer ) => integer,
  //   );
  //   trace_macros!( false );
  //   // dbg!( &got );
  //   Ok( got )
  // }

}

// let x = TheModule::select_matching!
// (
//   meta,
//   return Err( syn::Error::new( attr.span(), format!( "Unknown format of attribute, expected {}", 13/*$SELECTOR*/ ) ) ),
//   {
//     syn::Meta::List( ref meta_list ) => meta_list.nested.first(),
//   }
// );
// dbg!( &x );

// let lit_str = select_matching!
// (
//   meta,
//   return Err( syn::Error::new( attr.span(), format!( "Unknown format of attribute, expected {}", 13/*$SELECTOR*/ ) ) ),
//   {
//     syn::Meta::List( meta_list ) => meta_list.nested.first(),
//     Some( nested_meta ) => nested_meta,
//     syn::NestedMeta::Meta( meta2 ) => meta2,
//     syn::Meta::NameValue( name_value ) => &name_value.lit,
//     syn::Lit::Str( lit_str ) => lit_str.clone(),
//   }
// );

// let lit_str = match meta
// {
//   syn::Meta::List( meta_list ) => match meta_list.nested.first()
//   {
//     Some( nested_meta ) => match nested_meta
//     {
//       syn::NestedMeta::Meta( meta2 ) => match meta2
//       {
//         syn::Meta::NameValue( name_value ) => match &name_value.lit
//         {
//           syn::Lit::Str( lit_str ) => lit_str.clone(),
//           _ => return Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected syn::Lit::Str( lit_str )" ) ),
//         },
//         _ => return Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected syn::Meta::NameValue( name_value )" ) ),
//       },
//       _ => return Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected syn::NestedMeta::Meta( meta2 )" ) ),
//     },
//     _ => return Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected Some( nested_meta )" ) ),
//   },
//   _ => return Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected syn::Meta::List( meta_list )" ) ),
// };
/* xxx */

//

test_suite!
{
  basic,
}
