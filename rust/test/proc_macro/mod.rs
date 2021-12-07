#![feature( proc_macro_span )]

use wtest_basic::*;
use wproc_macro as TheModule;
use quote::*;

//

fn _tree_export_str_basic()
{

  let exp = r#"code : std :: collections :: HashMap < i32 , i32 > :
TokenStream [
    Ident {
        sym: std,
    },
    Punct {
        char: ':',
        spacing: Joint,
    },
    Punct {
        char: ':',
        spacing: Alone,
    },
    Ident {
        sym: collections,
    },
    Punct {
        char: ':',
        spacing: Joint,
    },
    Punct {
        char: ':',
        spacing: Alone,
    },
    Ident {
        sym: HashMap,
    },
    Punct {
        char: '<',
        spacing: Alone,
    },
    Ident {
        sym: i32,
    },
    Punct {
        char: ',',
        spacing: Alone,
    },
    Ident {
        sym: i32,
    },
    Punct {
        char: '>',
        spacing: Alone,
    },
]"#;
  let code = quote!( std::collections::HashMap< i32, i32 > );
  let got = TheModule::tree_export_str!( code );
  // println!( "{}", got );
  assert_eq!( got, exp );
  let got = TheModule::tree_print!( code );
  // println!( "{}", got );
  assert_eq!( got, exp );

}

//

fn _container_kind_basic()
{

  // test.case( "core::option::Option< i32 >" );
  let code = quote!( core::option::Option< i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_container_kind( &tree_type );
  assert_eq!( got, TheModule::ContainerKind::No );

  // test.case( "core::option::Option< Vec >" );
  let code = quote!( core::option::Option< Vec > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_container_kind( &tree_type );
  assert_eq!( got, TheModule::ContainerKind::No );

  // test.case( "alloc::vec::Vec< i32 >" );
  let code = quote!( alloc::vec::Vec< i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_container_kind( &tree_type );
  assert_eq!( got, TheModule::ContainerKind::Vector );

  // test.case( "alloc::vec::Vec" );
  let code = quote!( alloc::vec::Vec );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_container_kind( &tree_type );
  assert_eq!( got, TheModule::ContainerKind::Vector );

  // test.case( "std::vec::Vec< i32 >" );
  let code = quote!( std::vec::Vec< i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_container_kind( &tree_type );
  assert_eq!( got, TheModule::ContainerKind::Vector );

  // test.case( "std::vec::Vec" );
  let code = quote!( std::vec::Vec );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_container_kind( &tree_type );
  assert_eq!( got, TheModule::ContainerKind::Vector );

  // test.case( "std::Vec< i32 >" );
  let code = quote!( std::Vec< i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_container_kind( &tree_type );
  assert_eq!( got, TheModule::ContainerKind::Vector );

  // test.case( "std::Vec" );
  let code = quote!( std::Vec );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_container_kind( &tree_type );
  assert_eq!( got, TheModule::ContainerKind::Vector );

  // test.case( "not vector" );
  let code = quote!( std::SomeVector< i32, i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_container_kind( &tree_type );
  assert_eq!( got, TheModule::ContainerKind::No );

  // test.case( "hash map" );
  let code = quote!( std::collections::HashMap< i32, i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_container_kind( &tree_type );
  assert_eq!( got, TheModule::ContainerKind::HashMap );

}

//

fn _type_rightmost_basic()
{

  // test.case( "core::option::Option< i32 >" );
  let code = quote!( core::option::Option< i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_rightmost( &tree_type );
  assert_eq!( got, Some( "Option".to_string() ) );

}

//

fn _type_parameters_basic()
{

  macro_rules! q
  {
    ( $( $Src : tt )+ ) =>
    {
      syn::parse2::< syn::Type >( quote!( $( $Src )+ ) ).unwrap()
    }
  }

  // test.case( "core::option::Option< i8, i16, i32, i64 >" );
  let code = quote!( core::option::Option< i8, i16, i32, i64 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();

  let got : Vec< syn::Type > = TheModule::type_parameters( &tree_type, 0..=0 ).into_iter().cloned().collect();
  let exp = vec![ q!( i8 ) ];
  assert_eq!( got, exp );
  let got : Vec< syn::Type > = TheModule::type_parameters( &tree_type, 0..=1 ).into_iter().cloned().collect();
  let exp = vec![ q!( i8 ), q!( i16 ) ];
  assert_eq!( got, exp );
  let got : Vec< syn::Type > = TheModule::type_parameters( &tree_type, 0..=2 ).into_iter().cloned().collect();
  let exp = vec![ q!( i8 ), q!( i16 ), q!( i32 ) ];
  assert_eq!( got, exp );

  let got : Vec< syn::Type > = TheModule::type_parameters( &tree_type, 0..0 ).into_iter().cloned().collect();
  let exp : Vec< syn::Type > = vec![];
  assert_eq!( got, exp );
  let got : Vec< syn::Type > = TheModule::type_parameters( &tree_type, 0..1 ).into_iter().cloned().collect();
  let exp = vec![ q!( i8 ) ];
  assert_eq!( got, exp );
  let got : Vec< syn::Type > = TheModule::type_parameters( &tree_type, 0..2 ).into_iter().cloned().collect();
  let exp = vec![ q!( i8 ), q!( i16 ) ];
  assert_eq!( got, exp );

}

//

fn _span_of_basic()
{

  // use proc_macro2::Span;
  use wproc_macro::Span2;
  use wproc_macro::Span1;

  // test.case( "core::option::Option< i8, i16, i32, i64 >" );
  let code = quote!
  (
    #[derive( Debug, PartialEq )]
    pub struct Struct1
    {
      pub int_1 : i32,
    }
  );
  let ast = syn::parse2::< syn::DeriveInput >( code ).unwrap();
  // TheModule::tree_print!( ast );

  let got = TheModule::_span_of( &ast.data ).act();
  print( got );
  let got = TheModule::span_of!( ast.data );
  print( got );

  let code = quote!( core::option::Option< i8, i16, i32, i64 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::_span_of( &tree_type ).act();
  print( got );
  let got = TheModule::span_of!( tree_type );
  print( got );

  assert_eq!( true, true );

  fn print( src : proc_macro2::Span )
  {
    // println!( "span : {:?} {:?} {:?}", src, src.start(), src.end() );
    println!( "span : {:?}", src );
  }
}

//

fn _span_of_with_use()
{

  use wproc_macro::*;

  // test.case( "core::option::Option< i8, i16, i32, i64 >" );
  let code = quote!
  (
    #[derive( Debug, PartialEq )]
    pub struct Struct1
    {
      pub int_1 : i32,
    }
  );
  let ast = syn::parse2::< syn::DeriveInput >( code ).unwrap();

  let got = TheModule::_span_of( &ast.data ).act();
  print( got );
  let got = TheModule::span_of!( ast.data );
  print( got );

  let code = quote!( core::option::Option< i8, i16, i32, i64 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::_span_of( &tree_type ).act();
  print( got );
  let got = TheModule::span_of!( tree_type );
  print( got );

  assert_eq!( true, true );

  fn print( src : proc_macro2::Span )
  {
    // println!( "span : {:?} {:?} {:?}", src, src.start(), src.end() );
    println!( "span : {:?}", src );
  }
}

//

test_suite!
{
  tree_export_str_basic,
  container_kind_basic,
  type_rightmost_basic,
  type_parameters_basic,
  span_of_basic,
  span_of_with_use,
}
