
use test_tools::*;

#[cfg( feature = "in_wtools" )]
use wtools::interval as TheModule;
#[cfg( not( feature = "in_wtools" ) )]
use winterval as TheModule;

//

fn adapter_basic_test()
{
  use TheModule::*;

  // test.case( "basic" );

  let src = TheModule::Interval::new( 2, 4 );

  assert_eq!( TheModule::IntervalAdapter::first( &src ), 2 );
  assert_eq!( TheModule::IntervalAdapter::last( &src ), 4 );
  assert_eq!( TheModule::IntervalAdapter::len( &src ), 3 );
  assert_eq!( TheModule::IntervalAdapter::closed( &src ), ( 2, 4 ) );
  assert_eq!( TheModule::IntervalAdapter::closed_open( &src ), ( 2, 5 ) );
  assert_eq!( TheModule::IntervalAdapter::first_len( &src ), ( 2, 3 ) );

  assert_eq!( src.first(), 2 );
  assert_eq!( src.last(), 4 );
  assert_eq!( src.len(), 3 );
  assert_eq!( src.closed(), ( 2, 4 ) );
  assert_eq!( src.closed_open(), ( 2, 5 ) );
  assert_eq!( src.first_len(), ( 2, 3 ) );

}

//

fn adapter_std_closed_open_test()
{
  use TheModule::*;

  // test.case( "basic" );

  let src = 2..5;

  assert_eq!( TheModule::IntervalAdapter::first( &src ), 2 );
  assert_eq!( TheModule::IntervalAdapter::last( &src ), 4 );
  assert_eq!( TheModule::IntervalAdapter::len( &src ), 3 );
  assert_eq!( TheModule::IntervalAdapter::closed( &src ), ( 2, 4 ) );
  assert_eq!( TheModule::IntervalAdapter::closed_open( &src ), ( 2, 5 ) );
  assert_eq!( TheModule::IntervalAdapter::first_len( &src ), ( 2, 3 ) );

  assert_eq!( src.first(), 2 );
  // assert_eq!( src.last(), 4 );
  // assert_eq!( src.len(), 3 );
  assert_eq!( src.closed(), ( 2, 4 ) );
  assert_eq!( src.closed_open(), ( 2, 5 ) );
  assert_eq!( src.first_len(), ( 2, 3 ) );

}

//

fn adapter_std_closed_test()
{
  use TheModule::*;

  // test.case( "basic" );

  let src = 2..=4;

  assert_eq!( TheModule::IntervalAdapter::first( &src ), 2 );
  assert_eq!( TheModule::IntervalAdapter::last( &src ), 4 );
  assert_eq!( TheModule::IntervalAdapter::len( &src ), 3 );
  assert_eq!( TheModule::IntervalAdapter::closed( &src ), ( 2, 4 ) );
  assert_eq!( TheModule::IntervalAdapter::closed_open( &src ), ( 2, 5 ) );
  assert_eq!( TheModule::IntervalAdapter::first_len( &src ), ( 2, 3 ) );

  assert_eq!( src.first(), 2 );
  // assert_eq!( src.last(), 4 );
  // assert_eq!( src.len(), 3 );
  assert_eq!( src.closed(), ( 2, 4 ) );
  assert_eq!( src.closed_open(), ( 2, 5 ) );
  assert_eq!( src.first_len(), ( 2, 3 ) );

}

//

fn into_interval_test()
{
  use TheModule::*;

  // test.case( "from closed open std interval" );

  let src : Interval = ( 2..5 ).into();
  assert_eq!( src.closed(), ( 2, 4 ) );
  let src = Interval::from( 2..5 );
  assert_eq!( src.closed(), ( 2, 4 ) );

  // test.case( "from closed std interval" );

  let src : Interval = ( 2..=4 ).into();
  assert_eq!( src.closed(), ( 2, 4 ) );
  let src = Interval::from( 2..=4 );
  assert_eq!( src.closed(), ( 2, 4 ) );

}

//

test_suite!
{
  adapter_basic,
  adapter_std_closed,
  adapter_std_closed_open,
  into_interval,
}
