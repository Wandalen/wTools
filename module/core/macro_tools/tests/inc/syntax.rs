
use super::*;

//

tests_impls!
{

  //

  fn attribute() -> Result< () >
  {
    use macro_tools::syn::parse::Parser;

    // test.case( "AttributesOuter" );
    let code = qt!
    {
      #[ derive( Copy ) ]
      #[ derive( Clone ) ]
      #[ derive( Debug ) ]
    };
    let got = syn::parse2::< the_module::AttributesOuter >( code ).unwrap();
    let exp = the_module::AttributesOuter::from( syn::Attribute::parse_outer.parse2( qt!
    {
      #[ derive( Copy ) ]
      #[ derive( Clone ) ]
      #[ derive( Debug ) ]
    } )? );
    a_id!( got, exp );

    // test.case( "AttributesInner" );
    let code = qt!
    {
      // #![ deny( missing_docs ) ]
      #![ warn( something ) ]
    };
    let got = syn::parse2::< the_module::AttributesInner >( code ).unwrap();
    let exp = the_module::AttributesInner::from( syn::Attribute::parse_inner.parse2( qt!
    {
      // #![ deny( missing_docs ) ]
      #![ warn( something ) ]
    } )? );
    a_id!( got, exp );

    // test.case( "AttributesInner" );
    let code = qt!
    {
      #![ warn( missing_docs1 ) ]
      #![ warn( missing_docs2 ) ]
      #[ warn( something1 ) ]
      #[ warn( something2 ) ]
    };
    let got = syn::parse2::< the_module::Pair< the_module::AttributesInner, the_module::AttributesOuter > >( code ).unwrap();
    let exp = the_module::Pair::from
    ((
      the_module::AttributesInner::from( syn::Attribute::parse_inner.parse2( qt!
      {
        #![ warn( missing_docs1 ) ]
        #![ warn( missing_docs2 ) ]
      } )? ),
      the_module::AttributesOuter::from( syn::Attribute::parse_outer.parse2( qt!
      {
        #[ warn( something1 ) ]
        #[ warn( something2 ) ]
      } )? ),
    ));
    a_id!( got, exp );

    //

    Ok( () )
  }

  //

}

//

tests_index!
{
  attribute,
}
