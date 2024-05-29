
use super::*;

//

tests_impls!
{

  fn pair() -> Result< () >
  {
    use macro_tools::syn::parse::Parser;

    // test.case( "basic" );
    let code = qt!( x core::option::Option< i32 > );
    let got = syn::parse2::< the_module::Pair< syn::Ident, syn::Type > >( code )?;
    let exp = the_module::Pair::< syn::Ident, syn::Type >::new
    (
      syn::Ident::new( "x", proc_macro2::Span::call_site() ),
      syn::parse2::< syn::Type >( qt!( core::option::Option< i32 > ) )?,
    );
    a_id!( got, exp );

    // test.case( "pair of many" );
    let code = qt!
    {
      #[ derive( Copy ) ]
      #[ derive( Clone ) ]
      x1
    };
    let got = syn::parse2::< the_module::Pair< the_module::Many< the_module::AttributesOuter >, syn::Ident > >( code )?;
    let exp = the_module::Pair::< the_module::Many< the_module::AttributesOuter >, syn::Ident >
    (
      the_module::Many( vec!
      [
        the_module::AttributesOuter::from( syn::Attribute::parse_outer.parse2( qt!
        {
          #[ derive( Copy ) ]
          #[ derive( Clone ) ]
        } )? ),
      ]),
      syn::Ident::new( "x1", proc_macro2::Span::call_site() ),
    );
    a_id!( got, exp );

    // test.case( "punctuated of pairs" );
    let code = qt!
    {
      #[ derive( Copy ) ]
      x1,
      #[ derive( Clone ) ]
      x2,
      x3
    };
    type PunctuatedPairs = syn::punctuated::Punctuated
    <
      the_module::Pair
      <
        the_module::AttributesOuter,
        syn::Ident,
      >,
      syn::token::Comma
    >;

    let got = PunctuatedPairs::parse_terminated.parse2( code )?;
    let mut exp = PunctuatedPairs::new();
    exp.push( the_module::Pair::new
    (
      the_module::AttributesOuter::from( syn::Attribute::parse_outer.parse2( qt!( #[ derive( Copy ) ] ) )? ),
      syn::Ident::new( "x1", proc_macro2::Span::call_site() ),
    ));
    exp.push( the_module::Pair::new
    (
      the_module::AttributesOuter::from( syn::Attribute::parse_outer.parse2( qt!( #[ derive( Clone ) ] ) )? ),
      syn::Ident::new( "x2", proc_macro2::Span::call_site() ),
    ));
    exp.push( the_module::Pair::new
    (
      // from!(),
      Default::default(),
      syn::Ident::new( "x3", proc_macro2::Span::call_site() ),
    ));
    a_id!( got, exp );

    //

    Ok( () )
  }

  //

  fn many() -> Result< () >
  {
    use macro_tools::syn::parse::Parser;

    // test.case( "AttributesOuter" );
    let code = qt!
    {
      #[ derive( Copy ) ]
      #[ derive( Clone ) ]
      #[ derive( Debug ) ]
    };
    let got = syn::parse2::< the_module::Many< the_module::AttributesOuter > >( code ).unwrap();
    let exp = the_module::Many::< the_module::AttributesOuter >::new_with( vec!
    [
      the_module::AttributesOuter::from( syn::Attribute::parse_outer.parse2( qt!
      {
        #[ derive( Copy ) ]
        #[ derive( Clone ) ]
        #[ derive( Debug ) ]
      } )? ),
    ]);
    a_id!( got, exp );

    // test.case( "AttributesInner" );
    let code = qt!
    {
      // #![ deny( missing_docs ) ]
      #![ warn( something ) ]
    };
    let got = syn::parse2::< the_module::Many< the_module::AttributesInner > >( code ).unwrap();
    let exp = the_module::Many::< the_module::AttributesInner >::new_with( vec!
    [
      the_module::AttributesInner::from( syn::Attribute::parse_inner.parse2( qt!
      {
        // #![ deny( missing_docs ) ]
        #![ warn( something ) ]
      } )? ),
    ]);
    a_id!( got, exp );

    // test.case( "Item" );
    let code = qt!
    {
      fn f1(){}
      fn f2(){}
    };
    let got = syn::parse2::< the_module::Many< the_module::syn::Item > >( code ).unwrap();
    let exp = the_module::Many::< the_module::syn::Item >::new_with( vec!
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
