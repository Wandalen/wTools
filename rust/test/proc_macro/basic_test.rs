// #![ feature( proc_macro_span ) ]
// #![ feature( type_name_of_val ) ]

use wtest_basic::*;
use proc_macro_tools as TheModule;
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

fn _syn_err_basic()
{

  // test.case( "basic" );
  let err = TheModule::syn_err!( "abc" );
  assert_eq!( err.to_string(), "abc" );

  // test.case( "with span" );
  let code = quote!( core::option::Option< i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let err = TheModule::syn_err!( tree_type, "abc" );
  assert_eq!( err.to_string(), "abc" );
  // assert_eq!( err.span(), syn::spanned::Spanned::span( &tree_type ) );

  // test.case( "with span and args" );
  let code = quote!( core::option::Option< i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let err = TheModule::syn_err!( tree_type, "abc{}{}", "def", "ghi" );
  assert_eq!( err.to_string(), "abcdefghi" );
  // assert_eq!( err.span(), syn::spanned::Spanned::span( &tree_type ) );

  // test.case( "without span" );
  let err = TheModule::syn_err!( _, "abc" );
  assert_eq!( err.to_string(), "abc" );

  // test.case( "without span, but with args" );
  let err = TheModule::syn_err!( _, "abc{}{}", "def", "ghi" );
  assert_eq!( err.to_string(), "abcdefghi" );

}

//

fn _type_container_kind_basic()
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

  // test.case( "hash set" );
  let code = quote!( std::collections::HashSet< i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_container_kind( &tree_type );
  assert_eq!( got, TheModule::ContainerKind::HashSet );

}

//

fn _type_optional_container_kind_basic()
{

  // test.case( "non optional not container" );
  let code = quote!( i32 );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_optional_container_kind( &tree_type );
  assert_eq!( got, ( TheModule::ContainerKind::No, false ) );

  // test.case( "optional not container" );
  let code = quote!( core::option::Option< i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_optional_container_kind( &tree_type );
  assert_eq!( got, ( TheModule::ContainerKind::No, true ) );

  // test.case( "optional not container" );
  let code = quote!( Option< i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_optional_container_kind( &tree_type );
  assert_eq!( got, ( TheModule::ContainerKind::No, true ) );


  // test.case( "optional vector" );
  let code = quote!( core::option::Option< Vec > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_optional_container_kind( &tree_type );
  assert_eq!( got, ( TheModule::ContainerKind::Vector, true ) );

  // test.case( "optional vector" );
  let code = quote!( Option< Vec > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_optional_container_kind( &tree_type );
  assert_eq!( got, ( TheModule::ContainerKind::Vector, true ) );

  // test.case( "non optional vector" );
  let code = quote!( std::Vec< i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_optional_container_kind( &tree_type );
  assert_eq!( got, ( TheModule::ContainerKind::Vector, false ) );


  // test.case( "optional vector" );
  let code = quote!( core::option::Option< std::collections::HashMap< i32, i32 > > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_optional_container_kind( &tree_type );
  assert_eq!( got, ( TheModule::ContainerKind::HashMap, true ) );

  // test.case( "optional vector" );
  let code = quote!( Option< HashMap > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_optional_container_kind( &tree_type );
  assert_eq!( got, ( TheModule::ContainerKind::HashMap, true ) );

  // test.case( "non optional vector" );
  let code = quote!( HashMap< i32, i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_optional_container_kind( &tree_type );
  assert_eq!( got, ( TheModule::ContainerKind::HashMap, false ) );


  // test.case( "optional vector" );
  let code = quote!( core::option::Option< std::collections::HashSet< i32, i32 > > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_optional_container_kind( &tree_type );
  assert_eq!( got, ( TheModule::ContainerKind::HashSet, true ) );

  // test.case( "optional vector" );
  let code = quote!( Option< HashSet > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_optional_container_kind( &tree_type );
  assert_eq!( got, ( TheModule::ContainerKind::HashSet, true ) );

  // test.case( "non optional vector" );
  let code = quote!( HashSet< i32, i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = TheModule::type_optional_container_kind( &tree_type );
  assert_eq!( got, ( TheModule::ContainerKind::HashSet, false ) );

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

// fn attr_pair_single( attr : &syn::Attribute ) -> Result< ( String, syn::Lit, syn::Meta ), syn::Error >

#[test]
fn _attr_pair_single_basic() -> Result< (), syn::Error >
{
  use syn::spanned::Spanned;

  // test.case( "basic" );
  let input = quote!
  {
    #[derive( Former )]
    pub struct Struct1
    {
      #[former( default = 31 )]
      pub int_1 : i32,
    }
  };

  let ast = match syn::parse2::< syn::DeriveInput >( input )
  {
    Ok( syntax_tree ) => syntax_tree,
    Err( err ) => return Err( err ),
  };

  let fields = match ast.data
  {
    syn::Data::Struct( ref data_struct ) => match data_struct.fields
    {
      syn::Fields::Named( ref fields_named ) =>
      {
        &fields_named.named
      },
      _ => return Err( syn::Error::new( ast.span(), "Unknown format of data, expected syn::Fields::Named( ref fields_named )" ) ),
    },
    _ => return Err( syn::Error::new( ast.span(), "Unknown format of data, expected syn::Data::Struct( ref data_struct )" ) ),
  };

  let attr = fields.first().ok_or_else( || err( "No field" ) )?.attrs.first().ok_or_else( || err( "No attr" ) )?;

  let ( key, val, meta ) = TheModule::attr_pair_single( &attr )?;
  assert_eq!( key, "default".to_string() );
  assert_eq!( quote!( #val ).to_string(), "31".to_string() );
  let is = match meta
  {
    syn::Meta::List( _ ) => true,
    _ => false,
  };
  assert!( is );

  return Ok( () );

  fn err( src : &str ) -> syn::Error
  {
    syn::Error::new( proc_macro2::Span::call_site(), src )
  }
}

//

// #[test]
// fn path_of() -> Result< (), syn::Error >
// {
//
//   let input = quote!
//   {
//     This::is::path
//   };
//   let ast = match syn::parse2::< syn::Path >( input )
//   {
//     Ok( syntax_tree ) => syntax_tree,
//     Err( err ) => return Err( err ),
//   };
//
//   let got = proc_macro_tools::path_of( &ast );
//   assert_eq!( got, "This::is::path" );
//
//   return Ok( () );
// }

//

test_suite!
{
  tree_export_str_basic,
  syn_err_basic,
  type_container_kind_basic,
  type_optional_container_kind_basic,
  type_rightmost_basic,
  type_parameters_basic,
  // attr_pair_single_basic -> Result< (), syn::Error >,
  // path_of -> Result< (), syn::Error >,
}
