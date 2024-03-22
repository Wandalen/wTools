
use super::*;

//

tests_impls!
{

  fn tree_diagnostics_str_basic()
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
    let code = qt!( std::collections::HashMap< i32, i32 > );
    let got = the_module::tree_diagnostics_str!( code );
    // println!( "{}", got );
    a_id!( got, exp );
    let got = the_module::tree_print!( code );
    // println!( "{}", got );
    a_id!( got, exp );

  }

  //

  fn syn_err_basic()
  {

    // test.case( "basic" );
    let err = the_module::syn_err!( "abc" );
    a_id!( err.to_string(), "abc" );

    // test.case( "basic, trailing comma" );
    let err = the_module::syn_err!( "abc", );
    a_id!( err.to_string(), "abc" );

    // test.case( "with span" );
    let code = qt!( core::option::Option< i32 > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let err = the_module::syn_err!( tree_type, "abc" );
    a_id!( err.to_string(), "abc" );
    // a_id!( err.span(), syn::spanned::Spanned::span( &tree_type ) );

    // test.case( "with span, trailing comma" );
    let code = qt!( core::option::Option< i32 > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let err = the_module::syn_err!( tree_type, "abc", );
    a_id!( err.to_string(), "abc" );

    // test.case( "with span and args" );
    let code = qt!( core::option::Option< i32 > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let err = the_module::syn_err!( tree_type, "abc{}{}", "def", "ghi" );
    a_id!( err.to_string(), "abcdefghi" );
    // a_id!( err.span(), syn::spanned::Spanned::span( &tree_type ) );

    // test.case( "with span and args, trailing comma" );
    let code = qt!( core::option::Option< i32 > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let err = the_module::syn_err!( tree_type, "abc{}{}", "def", "ghi", );
    a_id!( err.to_string(), "abcdefghi" );

    // test.case( "without span" );
    let err = the_module::syn_err!( _, "abc" );
    a_id!( err.to_string(), "abc" );

    // test.case( "without span, trailing comma" );
    let err = the_module::syn_err!( _, "abc", );
    a_id!( err.to_string(), "abc" );

    // test.case( "without span, but with args" );
    let err = the_module::syn_err!( _, "abc{}{}", "def", "ghi" );
    a_id!( err.to_string(), "abcdefghi" );

    // test.case( "without span, trailing comma" );
    let err = the_module::syn_err!( _, "abc{}{}", "def", "ghi", );
    a_id!( err.to_string(), "abcdefghi" );

  }

  //

  fn type_container_kind_basic()
  {
    use the_module::exposed::container_kind;

    // test.case( "core::option::Option< i32 >" );
    let code = qt!( core::option::Option< i32 > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = container_kind::of_type( &tree_type );
    a_id!( got, the_module::container_kind::ContainerKind::No );

    // test.case( "core::option::Option< Vec >" );
    let code = qt!( core::option::Option< Vec > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = container_kind::of_type( &tree_type );
    a_id!( got, the_module::container_kind::ContainerKind::No );

    // test.case( "alloc::vec::Vec< i32 >" );
    let code = qt!( alloc::vec::Vec< i32 > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = container_kind::of_type( &tree_type );
    a_id!( got, the_module::container_kind::ContainerKind::Vector );

    // test.case( "alloc::vec::Vec" );
    let code = qt!( alloc::vec::Vec );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = container_kind::of_type( &tree_type );
    a_id!( got, the_module::container_kind::ContainerKind::Vector );

    // test.case( "std::vec::Vec< i32 >" );
    let code = qt!( std::vec::Vec< i32 > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = container_kind::of_type( &tree_type );
    a_id!( got, the_module::container_kind::ContainerKind::Vector );

    // test.case( "std::vec::Vec" );
    let code = qt!( std::vec::Vec );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = container_kind::of_type( &tree_type );
    a_id!( got, the_module::container_kind::ContainerKind::Vector );

    // test.case( "std::Vec< i32 >" );
    let code = qt!( std::Vec< i32 > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = container_kind::of_type( &tree_type );
    a_id!( got, the_module::container_kind::ContainerKind::Vector );

    // test.case( "std::Vec" );
    let code = qt!( std::Vec );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = container_kind::of_type( &tree_type );
    a_id!( got, the_module::container_kind::ContainerKind::Vector );

    // test.case( "not vector" );
    let code = qt!( std::SomeVector< i32, i32 > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = container_kind::of_type( &tree_type );
    a_id!( got, the_module::container_kind::ContainerKind::No );

    // test.case( "hash map" );
    let code = qt!( std::collections::HashMap< i32, i32 > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = container_kind::of_type( &tree_type );
    a_id!( got, the_module::container_kind::ContainerKind::HashMap );

    // test.case( "hash set" );
    let code = qt!( std::collections::HashSet< i32 > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = container_kind::of_type( &tree_type );
    a_id!( got, the_module::container_kind::ContainerKind::HashSet );

  }

  //

  fn type_optional_container_kind_basic()
  {

    // test.case( "non optional not container" );
    let code = qt!( i32 );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = the_module::container_kind::of_optional( &tree_type );
    a_id!( got, ( the_module::container_kind::ContainerKind::No, false ) );

    // test.case( "optional not container" );
    let code = qt!( core::option::Option< i32 > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = the_module::container_kind::of_optional( &tree_type );
    a_id!( got, ( the_module::container_kind::ContainerKind::No, true ) );

    // test.case( "optional not container" );
    let code = qt!( Option< i32 > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = the_module::container_kind::of_optional( &tree_type );
    a_id!( got, ( the_module::container_kind::ContainerKind::No, true ) );


    // test.case( "optional vector" );
    let code = qt!( core::option::Option< Vec > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = the_module::container_kind::of_optional( &tree_type );
    a_id!( got, ( the_module::container_kind::ContainerKind::Vector, true ) );

    // test.case( "optional vector" );
    let code = qt!( Option< Vec > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = the_module::container_kind::of_optional( &tree_type );
    a_id!( got, ( the_module::container_kind::ContainerKind::Vector, true ) );

    // test.case( "non optional vector" );
    let code = qt!( std::Vec< i32 > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = the_module::container_kind::of_optional( &tree_type );
    a_id!( got, ( the_module::container_kind::ContainerKind::Vector, false ) );


    // test.case( "optional vector" );
    let code = qt!( core::option::Option< std::collections::HashMap< i32, i32 > > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = the_module::container_kind::of_optional( &tree_type );
    a_id!( got, ( the_module::container_kind::ContainerKind::HashMap, true ) );

    // test.case( "optional vector" );
    let code = qt!( Option< HashMap > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = the_module::container_kind::of_optional( &tree_type );
    a_id!( got, ( the_module::container_kind::ContainerKind::HashMap, true ) );

    // test.case( "non optional vector" );
    let code = qt!( HashMap< i32, i32 > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = the_module::container_kind::of_optional( &tree_type );
    a_id!( got, ( the_module::container_kind::ContainerKind::HashMap, false ) );


    // test.case( "optional vector" );
    let code = qt!( core::option::Option< std::collections::HashSet< i32, i32 > > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = the_module::container_kind::of_optional( &tree_type );
    a_id!( got, ( the_module::container_kind::ContainerKind::HashSet, true ) );

    // test.case( "optional vector" );
    let code = qt!( Option< HashSet > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = the_module::container_kind::of_optional( &tree_type );
    a_id!( got, ( the_module::container_kind::ContainerKind::HashSet, true ) );

    // test.case( "non optional vector" );
    let code = qt!( HashSet< i32, i32 > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = the_module::container_kind::of_optional( &tree_type );
    a_id!( got, ( the_module::container_kind::ContainerKind::HashSet, false ) );

  }

  //

  fn type_rightmost_basic()
  {

    // test.case( "core::option::Option< i32 >" );
    let code = qt!( core::option::Option< i32 > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
    let got = the_module::typ::type_rightmost( &tree_type );
    a_id!( got, Some( "Option".to_string() ) );

  }

  //

  fn type_parameters_basic()
  {

    macro_rules! q
    {
      ( $( $Src : tt )+ ) =>
      {
        syn::parse2::< syn::Type >( qt!( $( $Src )+ ) ).unwrap()
      }
    }

    // test.case( "core::option::Option< i8, i16, i32, i64 >" );
    let code = qt!( core::option::Option< i8, i16, i32, i64 > );
    let tree_type = syn::parse2::< syn::Type >( code ).unwrap();

    let got : Vec< syn::Type > = the_module::typ::type_parameters( &tree_type, 0..=0 ).into_iter().cloned().collect();
    let exp = vec![ q!( i8 ) ];
    a_id!( got, exp );
    let got : Vec< syn::Type > = the_module::typ::type_parameters( &tree_type, 0..=1 ).into_iter().cloned().collect();
    let exp = vec![ q!( i8 ), q!( i16 ) ];
    a_id!( got, exp );
    let got : Vec< syn::Type > = the_module::typ::type_parameters( &tree_type, 0..=2 ).into_iter().cloned().collect();
    let exp = vec![ q!( i8 ), q!( i16 ), q!( i32 ) ];
    a_id!( got, exp );

    let got : Vec< syn::Type > = the_module::typ::type_parameters( &tree_type, 0..0 ).into_iter().cloned().collect();
    let exp : Vec< syn::Type > = vec![];
    a_id!( got, exp );
    let got : Vec< syn::Type > = the_module::typ::type_parameters( &tree_type, 0..1 ).into_iter().cloned().collect();
    let exp = vec![ q!( i8 ) ];
    a_id!( got, exp );
    let got : Vec< syn::Type > = the_module::typ::type_parameters( &tree_type, 0..2 ).into_iter().cloned().collect();
    let exp = vec![ q!( i8 ), q!( i16 ) ];
    a_id!( got, exp );

    // unbound
    let got : Vec< syn::Type > = the_module::typ::type_parameters( &tree_type, .. ).into_iter().cloned().collect();
    let exp = vec![ q!( i8 ), q!( i16 ), q!( i32 ), q!( i64 ) ];
    a_id!( got, exp );

    let got : Vec< syn::Type > = the_module::typ::type_parameters( &tree_type, .. ).into_iter().cloned().collect();
    let exp = vec![ q!( i8 ), q!( i16 ), q!( i32 ), q!( i64 ) ];
    a_id!( got, exp );

    let got : Vec< syn::Type > = the_module::typ::type_parameters( &tree_type, .. ).into_iter().cloned().collect();
    let exp = vec![ q!( i8 ), q!( i16 ), q!( i32 ), q!( i64 ) ];
    a_id!( got, exp );

  }

  //

  // fn equation( attr : &syn::Attribute ) -> Result< ( String, syn::Lit, syn::Meta ), syn::Error >

  // qqq : xxx : fix
  // #[test]
  // fn attr_pair_single_basic() -> Result< (), syn::Error >
  // {
  //   use syn::spanned::Spanned;
  //
  //   // test.case( "basic" );
  //   let input = qt!
  //   {
  //     #[ derive( Former ) ]
  //     pub struct Struct1
  //     {
  //       #[former( default = 31 ) ]
  //       pub int_1 : i32,
  //     }
  //   };
  //
  //   let ast = match syn::parse2::< syn::DeriveInput >( input )
  //   {
  //     Ok( syntax_tree ) => syntax_tree,
  //     Err( err ) => return Err( err ),
  //   };
  //
  //   let fields = match ast.data
  //   {
  //     syn::Data::Struct( ref data_struct ) => match data_struct.fields
  //     {
  //       syn::Fields::Named( ref fields_named ) =>
  //       {
  //         &fields_named.named
  //       },
  //       _ => return Err( syn::Error::new( ast.span(), "Unknown format of data, expected syn::Fields::Named( ref fields_named )" ) ),
  //     },
  //     _ => return Err( syn::Error::new( ast.span(), "Unknown format of data, expected syn::Data::Struct( ref data_struct )" ) ),
  //   };
  //
  //   let attr = fields.first().ok_or_else( || err( "No field" ) )?.attrs.first().ok_or_else( || err( "No attr" ) )?;
  //
  //   let ( key, val, meta ) = the_module::equation( &attr )?;
  //   a_id!( key, "default".to_string() );
  //   a_id!( qt!( #val ).to_string(), "31".to_string() );
  //   let is = match meta
  //   {
  //     syn::Meta::List( _ ) => true,
  //     _ => false,
  //   };
  //   assert!( is );
  //
  //   return Ok( () );
  //
  //   fn err( src : &str ) -> syn::Error
  //   {
  //     syn::Error::new( proc_macro2::Span::call_site(), src )
  //   }
  // }
//
//   //
//
//   fn path_of() -> Result< (), syn::Error >
//   {
//
//     let input = qt!
//     {
//       This::is::path
//     };
//     let ast = match syn::parse2::< syn::Path >( input )
//     {
//       Ok( syntax_tree ) => syntax_tree,
//       Err( err ) => return Err( err ),
//     };
//
//     let got = macro_tools::path_of( &ast );
//     a_id!( got, "This::is::path" );
//
//     return Ok( () );
//   }

}

//

tests_index!
{
  tree_diagnostics_str_basic,
  syn_err_basic,
  type_container_kind_basic,
  type_optional_container_kind_basic,
  type_rightmost_basic,
  type_parameters_basic,
  // attr_pair_single_basic,
  // path_of,
}
