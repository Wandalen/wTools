
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

}

//

tests_index!
{
  tree_diagnostics_str_basic,
  syn_err_basic,
}
