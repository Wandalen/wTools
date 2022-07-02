
use super::*;

//

tests_impls!
{


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

  fn many() -> Result< () >
  {
    use proc_macro_tools::syn::parse::Parser;

    // test.case( "AttributeOuter" );
    let code = qt!
    {
      #[ derive( Copy ) ]
      #[ derive( Clone ) ]
      #[ derive( Debug ) ]
    };
    let got = syn::parse2::< TheModule::Many< TheModule::AttributeOuter > >( code ).unwrap();
    let exp = TheModule::Many::< TheModule::AttributeOuter >::new_with( vec!
    [
      TheModule::AttributeOuter::from( syn::Attribute::parse_outer.parse2( qt!( #[ derive( Copy ) ] ) )? ),
      TheModule::AttributeOuter::from( syn::Attribute::parse_outer.parse2( qt!( #[ derive( Clone ) ] ) )? ),
      TheModule::AttributeOuter::from( syn::Attribute::parse_outer.parse2( qt!( #[ derive( Debug ) ] ) )? ),
    ]);
    a_id!( got, exp );

    // test.case( "AttributeInner" );
    let code = qt!
    {
      #![ warn( missing_docs ) ]
      #![ warn( something ) ]
    };
    let got = syn::parse2::< TheModule::Many< TheModule::AttributeInner > >( code ).unwrap();
    let exp = TheModule::Many::< TheModule::AttributeInner >::new_with( vec!
    [
      TheModule::AttributeInner::from( syn::Attribute::parse_inner.parse2( qt!( #![ warn( missing_docs ) ] ) )? ),
      TheModule::AttributeInner::from( syn::Attribute::parse_inner.parse2( qt!( #![ warn( something ) ] ) )? ),
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

