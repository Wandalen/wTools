use super::*;
use derive_tools::From;
// pub use the_module::reflect;

#[ test ]
fn basic()
{

  #[ derive( From, Debug, PartialEq ) ]
  pub struct Voltage( f32 );

  #[ derive( From, Debug, PartialEq ) ]
  pub struct Resistance( f32 );

  #[ derive( Debug, PartialEq ) ]
  pub struct Pair( f32, f32 );

  impl From< (f32, f32) > for Pair
  {
    fn from( ( a, b ) : ( f32, f32 ) ) -> Self
    {
      Self( a, b )
    }
  }

  let voltage : Voltage = 1.0.into();
  a_id!( voltage, Voltage( 1.0 ) );
  let resistance : Resistance = 2.0.into();
  a_id!( resistance, Resistance( 2.0 ) );
  let pair : Pair = ( 3.0, 4.0 ).into();
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