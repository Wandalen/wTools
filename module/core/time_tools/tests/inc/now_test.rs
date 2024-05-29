
#[ allow( unused_imports ) ]
use super::*;

//

tests_impls!
{

  #[ cfg( any( feature = "chrono", feature = "time_chrono" ) ) ]
  fn basic()
  {
    use the_module::*;

    // test.case( "time::now" );
    let got = time::now();
    a_true!( got > 0 );

    // test.case( "time::ms::now" );
    let got1 = time::now();
    let got2 = time::ms::now();
    a_true!( got2 - got2 <= 10 );

    // // test.case( "time::ns::now" );
    let got1 = time::now();
    let got2 = time::ns::now();
    a_true!( got2 / 1_000_000 - got1 <= 10 );
    // zzz : use equal!

    // test.case( "time::s::now" );
    let got1 = time::now();
    let got2 = time::s::now();
    a_id!( got1 / 1000, got2 );
  }
}

//

tests_index!
{
  basic,
}
