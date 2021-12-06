
use wtest_basic::*;
use wproc_macro as TheModule;
use quote::quote;

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

}

//

fn _container_kind_basic()
{

  // test.case( "core::option::Option< i32 >" );
  let code = quote!( core::option::Option< i32 > );
  let tree = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::container_kind( &tree );
  assert_eq!( got, TheModule::ContainerKind::No );

  // test.case( "core::option::Option< Vec >" );
  let code = quote!( core::option::Option< Vec > );
  let tree = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::container_kind( &tree );
  assert_eq!( got, TheModule::ContainerKind::No );

  // test.case( "alloc::vec::Vec< i32 >" );
  let code = quote!( alloc::vec::Vec< i32 > );
  let tree = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::container_kind( &tree );
  assert_eq!( got, TheModule::ContainerKind::Vector );

  // test.case( "alloc::vec::Vec" );
  let code = quote!( alloc::vec::Vec );
  let tree = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::container_kind( &tree );
  assert_eq!( got, TheModule::ContainerKind::Vector );

  // test.case( "std::vec::Vec< i32 >" );
  let code = quote!( std::vec::Vec< i32 > );
  let tree = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::container_kind( &tree );
  assert_eq!( got, TheModule::ContainerKind::Vector );

  // test.case( "std::vec::Vec" );
  let code = quote!( std::vec::Vec );
  let tree = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::container_kind( &tree );
  assert_eq!( got, TheModule::ContainerKind::Vector );

  // test.case( "std::Vec< i32 >" );
  let code = quote!( std::Vec< i32 > );
  let tree = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::container_kind( &tree );
  assert_eq!( got, TheModule::ContainerKind::Vector );

  // test.case( "std::Vec" );
  let code = quote!( std::Vec );
  let tree = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::container_kind( &tree );
  assert_eq!( got, TheModule::ContainerKind::Vector );

  // test.case( "not vector" );
  let code = quote!( std::SomeVector< i32, i32 > );
  let tree = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::container_kind( &tree );
  assert_eq!( got, TheModule::ContainerKind::No );

  // test.case( "hash map" );
  let code = quote!( std::collections::HashMap< i32, i32 > );
  let tree = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::container_kind( &tree );
  assert_eq!( got, TheModule::ContainerKind::HashMap );

}

//

test_suite!
{
  tree_export_str_basic,
  container_kind_basic,
}
