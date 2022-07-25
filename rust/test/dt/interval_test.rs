#[ allow( unused_imports ) ]
use super::*;

tests_impls!
{

  //

  #[ cfg( feature = "use_std" ) ]
  fn adapter_basic()
  {
    use TheModule::*;

    // test.case( "basic" );

    let src = TheModule::Interval::new( 2, 4 );

    a_id!( TheModule::IntervalAdapter::first( &src ), 2 );
    a_id!( TheModule::IntervalAdapter::last( &src ), 4 );
    a_id!( TheModule::IntervalAdapter::len( &src ), 3 );
    a_id!( TheModule::IntervalAdapter::closed( &src ), ( 2, 4 ) );
    a_id!( TheModule::IntervalAdapter::closed_open( &src ), ( 2, 5 ) );
    a_id!( TheModule::IntervalAdapter::first_len( &src ), ( 2, 3 ) );

    a_id!( src.first(), 2 );
    a_id!( src.last(), 4 );
    a_id!( src.len(), 3 );
    a_id!( src.closed(), ( 2, 4 ) );
    a_id!( src.closed_open(), ( 2, 5 ) );
    a_id!( src.first_len(), ( 2, 3 ) );

  }

  //

  #[ cfg( feature = "use_std" ) ]
  fn adapter_std_closed_open()
  {
    use TheModule::*;

    // test.case( "basic" );

    let src = 2..5;

    a_id!( TheModule::IntervalAdapter::first( &src ), 2 );
    a_id!( TheModule::IntervalAdapter::last( &src ), 4 );
    a_id!( TheModule::IntervalAdapter::len( &src ), 3 );
    a_id!( TheModule::IntervalAdapter::closed( &src ), ( 2, 4 ) );
    a_id!( TheModule::IntervalAdapter::closed_open( &src ), ( 2, 5 ) );
    a_id!( TheModule::IntervalAdapter::first_len( &src ), ( 2, 3 ) );

    a_id!( src.first(), 2 );
    // a_id!( src.last(), 4 );
    // a_id!( src.len(), 3 );
    a_id!( src.closed(), ( 2, 4 ) );
    a_id!( src.closed_open(), ( 2, 5 ) );
    a_id!( src.first_len(), ( 2, 3 ) );

  }

  //

  #[ cfg( feature = "use_std" ) ]
  fn adapter_std_closed()
  {
    use TheModule::*;

    // test.case( "basic" );

    let src = 2..=4;

    a_id!( TheModule::IntervalAdapter::first( &src ), 2 );
    a_id!( TheModule::IntervalAdapter::last( &src ), 4 );
    a_id!( TheModule::IntervalAdapter::len( &src ), 3 );
    a_id!( TheModule::IntervalAdapter::closed( &src ), ( 2, 4 ) );
    a_id!( TheModule::IntervalAdapter::closed_open( &src ), ( 2, 5 ) );
    a_id!( TheModule::IntervalAdapter::first_len( &src ), ( 2, 3 ) );

    a_id!( src.first(), 2 );
    // a_id!( src.last(), 4 );
    // a_id!( src.len(), 3 );
    a_id!( src.closed(), ( 2, 4 ) );
    a_id!( src.closed_open(), ( 2, 5 ) );
    a_id!( src.first_len(), ( 2, 3 ) );

  }

  //

  #[ cfg( feature = "use_std" ) ]
  fn into_interval()
  {
    use TheModule::*;

    // test.case( "from closed open std interval" );

    let src : Interval = ( 2..5 ).into();
    a_id!( src.closed(), ( 2, 4 ) );
    let src = Interval::from( 2..5 );
    a_id!( src.closed(), ( 2, 4 ) );

    // test.case( "from closed std interval" );

    let src : Interval = ( 2..=4 ).into();
    a_id!( src.closed(), ( 2, 4 ) );
    let src = Interval::from( 2..=4 );
    a_id!( src.closed(), ( 2, 4 ) );

  }

}

//

tests_index!
{
  adapter_basic,
  adapter_std_closed,
  adapter_std_closed_open,
  into_interval,
}
