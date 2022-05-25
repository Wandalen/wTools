
use test_tools::*;

use super::TheModule;

//

tests_impls!
{
  #[ test ]
  fn basic()
  {
    use TheModule::*;

    // test.case( "time::now" );
    let got = time::now();
    assert!( got > 0 );

    // test.case( "time::ms::now" );
    let got1 = time::now();
    let got2 = time::ms::now();
    assert_eq!( got1, got2 );

    // // test.case( "time::ns::now" );
    // let got1 = time::now();
    // let got2 = time::ns::now();
    // assert_eq!( got1, got2 / 1000000 );
    // zzz : use equal!

    // test.case( "time::s::now" );
    let got1 = time::now();
    let got2 = time::s::now();
    assert_eq!( got1 / 1000, got2 );
  }
}

//

tests_index!
{
  basic,
}
