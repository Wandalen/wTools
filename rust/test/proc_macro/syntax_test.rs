
use super::*;

//

tests_impls!
{

  #[ test ]
  fn pair() -> Result< () >
  {

    // test.case( "basic" );
    let code = qt!( x core::option::Option< i32 > );
    let got = syn::parse2::< TheModule::Pair< syn::Ident, syn::Type > >( code ).unwrap();
    let exp = TheModule::Pair::< syn::Ident, syn::Type >::new
    (
      syn::Ident::new( "x", proc_macro2::Span::call_site() ),
      syn::parse2::< syn::Type >( qt!( core::option::Option< i32 > ) )?,
    );
    a_id!( got, exp );

    Ok( () )
  }

  //

  #[ test ]
  fn many() -> Result< () >
  {
    use proc_macro_tools::syn::parse::Parser;

    // test.case( "basic" );
    let code = qt!
    {
      #[ derive( Copy ) ]
      #[ derive( Clone ) ]
      //
      #[ derive( Debug ) ]
    };
    let got = syn::parse2::< TheModule::Many< TheModule::AttributeOuter > >( code ).unwrap();

    // syn::Attribute::parse_outer.parse2( qt!( #[ derive( Copy ) ] ) );

    let exp = TheModule::Many::< TheModule::AttributeOuter >::new_with( vec!
    [
      TheModule::AttributeOuter::from( syn::Attribute::parse_outer.parse2( qt!( #[ derive( Copy ) ] ) )? ),
      TheModule::AttributeOuter::from( syn::Attribute::parse_outer.parse2( qt!( #[ derive( Clone ) ] ) )? ),
      TheModule::AttributeOuter::from( syn::Attribute::parse_outer.parse2( qt!( #[ derive( Debug ) ] ) )? ),
    ]);
    a_id!( got, exp );

    Ok( () )
  }

}

//

tests_index!
{
  pair,
  many,
}

