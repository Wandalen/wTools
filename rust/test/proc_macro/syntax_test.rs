
use super::*;

//

tests_impls!
{

  //

  fn attribute() -> Result< () >
  {
    use proc_macro_tools::syn::parse::Parser;

    // test.case( "AttributesOuter" );
    let code = qt!
    {
      #[ derive( Copy ) ]
      #[ derive( Clone ) ]
      #[ derive( Debug ) ]
    };
    let got = syn::parse2::< TheModule::AttributesOuter >( code ).unwrap();
    let exp = TheModule::AttributesOuter::from( syn::Attribute::parse_outer.parse2( qt!
    {
      #[ derive( Copy ) ]
      #[ derive( Clone ) ]
      #[ derive( Debug ) ]
    } )? );
    a_id!( got, exp );

    // test.case( "AttributesInner" );
    let code = qt!
    {
      #![ warn( missing_docs ) ]
      #![ warn( something ) ]
    };
    let got = syn::parse2::< TheModule::AttributesInner >( code ).unwrap();
    let exp = TheModule::AttributesInner::from( syn::Attribute::parse_inner.parse2( qt!
    {
      #![ warn( missing_docs ) ]
      #![ warn( something ) ]
    } )? );
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
