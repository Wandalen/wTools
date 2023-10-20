
use test_tools::exposed::*;

tests_impls!
{
  #[ cfg( feature = "time_now" ) ]
  fn basic()
  {
    // test.case( "wtools::now" );
    let got = wtools::now();
    a_true!( got > 0 );

    // test.case( "wtools::ms::now" );
    let got1 = wtools::now();
    let got2 = wtools::ms::now();
    a_true!( got2 - got2 <= 10 );

    // // test.case( "wtools::ns::now" );
    let got1 = wtools::now();
    let got2 = wtools::ns::now();
    a_true!( got2 / 1_000_000 - got1 <= 10 );
    // zzz : use equal!

    // test.case( "time::s::now" );
    let got1 = wtools::now();
    let got2 = wtools::s::now();
    a_id!( got1 / 1000, got2 );
  }
}

//

tests_index!
{
  basic,
}
