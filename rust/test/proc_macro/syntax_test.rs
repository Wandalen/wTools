
use super::*;

//

tests_impls!
{


  fn pair() -> Result< () >
  {
    use proc_macro_tools::syn::parse::Parser;

    // test.case( "basic" );
    let code = qt!( x core::option::Option< i32 > );
    let got = syn::parse2::< TheModule::Pair< syn::Ident, syn::Type > >( code )?;
    let exp = TheModule::Pair::< syn::Ident, syn::Type >::new
    (
      syn::Ident::new( "x", proc_macro2::Span::call_site() ),
      syn::parse2::< syn::Type >( qt!( core::option::Option< i32 > ) )?,
    );
    a_id!( got, exp );

    // test.case( "pair of many" );
    let code = qt!
    {
      #[ derive( Copy ) ]
      x1
      // #[ derive( Clone ) ]
      // x2
    };
    let got = syn::parse2::< TheModule::Pair< TheModule::Many< TheModule::AttributesOuter >, syn::Ident > >( code )?;
    let exp = TheModule::Pair::< TheModule::Many< TheModule::AttributesOuter >, syn::Ident >
    (
      TheModule::Many( vec![ TheModule::AttributesOuter::from( syn::Attribute::parse_outer.parse2( qt!( #[ derive( Copy ) ] ) )? ) ] ),
      syn::Ident::new( "x1", proc_macro2::Span::call_site() ),
    );
    a_id!( got, exp );

    // test.case( "punctuated of pairs" );
    let code = qt!
    {
      #[ derive( Copy ) ]
      x1,
      #[ derive( Clone ) ]
      x2
    };
    type PunctuatedPairs = syn::punctuated::Punctuated
    <
      TheModule::Pair
      <
        TheModule::Many< TheModule::AttributesOuter >,
        syn::Ident,
      >,
      syn::token::Comma
    >;

    let got = PunctuatedPairs::parse_terminated.parse2( code )?;
    let mut exp = PunctuatedPairs::new();
    exp.push( TheModule::Pair::new
    (
      TheModule::Many( vec![ TheModule::AttributesOuter::from( syn::Attribute::parse_outer.parse2( qt!( #[ derive( Copy ) ] ) )? ) ] ),
      syn::Ident::new( "x1", proc_macro2::Span::call_site() ),
    ));
    exp.push( TheModule::Pair::new
    (
      TheModule::Many( vec![ TheModule::AttributesOuter::from( syn::Attribute::parse_outer.parse2( qt!( #[ derive( Clone ) ] ) )? ) ] ),
      syn::Ident::new( "x2", proc_macro2::Span::call_site() ),
    ));
    a_id!( got, exp );

    //

    Ok( () )
  }

  //

  fn many() -> Result< () >
  {
    use proc_macro_tools::syn::parse::Parser;

    // test.case( "AttributesOuter" );
    let code = qt!
    {
      #[ derive( Copy ) ]
      #[ derive( Clone ) ]
      #[ derive( Debug ) ]
    };
    let got = syn::parse2::< TheModule::Many< TheModule::AttributesOuter > >( code ).unwrap();
    let exp = TheModule::Many::< TheModule::AttributesOuter >::new_with( vec!
    [
      TheModule::AttributesOuter::from( syn::Attribute::parse_outer.parse2( qt!( #[ derive( Copy ) ] ) )? ),
      TheModule::AttributesOuter::from( syn::Attribute::parse_outer.parse2( qt!( #[ derive( Clone ) ] ) )? ),
      TheModule::AttributesOuter::from( syn::Attribute::parse_outer.parse2( qt!( #[ derive( Debug ) ] ) )? ),
    ]);
    a_id!( got, exp );

    // test.case( "AttributesInner" );
    let code = qt!
    {
      #![ warn( missing_docs ) ]
      #![ warn( something ) ]
    };
    let got = syn::parse2::< TheModule::Many< TheModule::AttributesInner > >( code ).unwrap();
    let exp = TheModule::Many::< TheModule::AttributesInner >::new_with( vec!
    [
      TheModule::AttributesInner::from( syn::Attribute::parse_inner.parse2( qt!( #![ warn( missing_docs ) ] ) )? ),
      TheModule::AttributesInner::from( syn::Attribute::parse_inner.parse2( qt!( #![ warn( something ) ] ) )? ),
    ]);
    a_id!( got, exp );

    // test.case( "Item" );
    let code = qt!
    {
      fn f1(){}
      fn f2(){}
    };
    let got = syn::parse2::< TheModule::Many< TheModule::syn::Item > >( code ).unwrap();
    let exp = TheModule::Many::< TheModule::syn::Item >::new_with( vec!
    [
      syn::parse2::< syn::Item >( qt!( fn f1(){} ) )?,
      syn::parse2::< syn::Item >( qt!( fn f2(){} ) )?,
    ]);
    a_id!( got, exp );

    //

    Ok( () )
  }

}

//

tests_index!
{
  pair,
  many,
}

