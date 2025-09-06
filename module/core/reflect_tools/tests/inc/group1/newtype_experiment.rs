use test_tools::a_id;
// pub use the_module::reflect;

#[ test ]
fn basic()
{

  #[ derive( Debug, PartialEq ) ]
  pub struct Voltage( f32 );

  #[ derive( Debug, PartialEq ) ]
  pub struct Resistance( f32 );

  #[ derive( Debug, PartialEq ) ]
  pub struct Pair( f32, f32 );

  let voltage = Voltage( 1.0 );
  a_id!( voltage, Voltage( 1.0 ) );
  let resistance = Resistance( 2.0 );
  a_id!( resistance, Resistance( 2.0 ) );
  let pair = Pair( 3.0, 4.0 );
  a_id!( pair, Pair( 3.0, 4.0 ) );

  #[ derive( Debug, PartialEq ) ]
  #[ allow( dead_code ) ]
  pub struct Options3
  {
    voltage : Voltage,
    resistance : Resistance,
    pair : Pair,
  }

  // Options3::former()
  // .set( voltage )
  // .set( resistance )
  // .set( pair )
  // .form();

}