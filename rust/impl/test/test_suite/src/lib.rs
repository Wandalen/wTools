#![ cfg_attr( feature = "nightly", feature( proc_macro_span ) ) ]
use quote::quote;
use proc_macro::TokenStream;
use syn::parse::{ Parse, ParseStream };
use syn::{ parse_macro_input, Result, Token };

///
/// Handle test suite data.
///

#[ derive( Debug ) ]
struct TestSuiteMacroInput
{
  test_name : syn::Ident,
  routines : Vec<syn::Ident>,
}

impl Parse for TestSuiteMacroInput
{
  fn parse( input : ParseStream ) -> Result< Self >
  {
    let first = syn::Ident::parse( input )?;

    let to_list = if let Ok( _to_list ) = < Token![ => ] >::parse( input )
    {
      false
    }
    else
    {
      true
    };

    let mut routines = vec![];

    let test_name = if to_list
    {
      routines.push( first );
      < Token![ , ] >::parse( input ).unwrap_or_default();
      #[ cfg( not( feature = "nightly" ) ) ]
      {
        syn::Ident::new( "test_routine", input.span() )
      }
      #[ cfg( feature = "nightly" ) ]
      {
        let span = input.span();
        let module_name = span.unwrap().source_file()
        .path().file_stem().unwrap().to_owned();
        let module_name = module_name.into_string().unwrap();
        syn::Ident::new( &module_name[ .. ], input.span() )
      }
    }
    else
    {
      first
    };

    while let Ok( ident ) = syn::Ident::parse( input )
    {
      routines.push( ident );
      < Token![ , ] >::parse( input ).unwrap_or_default();
    }

    Ok( TestSuiteMacroInput { test_name, routines } )
  }
}

impl Into< proc_macro2::TokenStream > for TestSuiteMacroInput
{
  fn into( self ) -> proc_macro2::TokenStream
  {
    self.expand()
  }
}

impl TestSuiteMacroInput
{
  fn expand( &self ) -> proc_macro2::TokenStream
  {
    let name = &self.test_name;
    let idents = self.routines.iter().map( | name |
    {
      quote!
      {
        #[ test ]
        fn #name()
        {
          super::#name();
        }
      }
    });
    quote! { mod #name { #(#idents)* } }
  }
}

///
/// Mechanism to define test suite.
/// This macro encourages refactoring the code of the test in the most readable way, gathering a list of all test routines at the end of the test file.
///
/// Test suite uses name :
/// - provided by user
/// - default without feature `nightly` - "test_routine"
/// - default with feature `nightly` - name of current module.
///
/// # Sample
/// use wtest_basic::*;
///
/// //
///
/// fn pass1()
/// {
///   assert_eq!( true, true );
/// }
///
/// //
///
/// fn pass2()
/// {
///   assert_eq!( 1, 1 );
/// }
///
/// //
///
/// // declaration with name
/// test_suite!
/// { simple =>
///   pass1,
///   pass2,
/// }
///
/// // declaration with name
/// // test_suite!
/// // {
/// //   pass1,
/// //   pass2,
/// // }
///

#[ proc_macro ]
pub fn test_suite( input : TokenStream ) -> TokenStream
{
  let input = parse_macro_input!( input as TestSuiteMacroInput );
  let output : proc_macro2::TokenStream = input.into();
  output.into()
}

