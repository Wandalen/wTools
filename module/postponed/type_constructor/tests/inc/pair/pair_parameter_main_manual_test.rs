#[ allow( unused_imports ) ]
use super::*;

struct Pair1( pub f64, pub f32 );
impl From< ( f64, f32 ) > for Pair1
{
  #[ inline ]
  fn from( src : ( f64, f32 ) ) -> Self { Self( src.0, src.1 ) }
}
impl From< Pair1 > for ( f64, f32 )
{
  #[ inline ]
  fn from( src : Pair1 ) -> Self { ( src.0, src.1 ) }
}
impl the_module::From_2< f64, f32 > for Pair1
{
  #[ inline ]
  fn from_2( _0 : f64, _1 : f32 ) -> Self { Self( _0, _1 ) }
}

#[ derive( Debug, Clone, PartialEq ) ]
struct Pair2( pub f32, pub f64 );
impl From<( f32, f64 )> for Pair2
{
  #[ inline ]
  fn from( src : ( f32, f64 ) ) -> Self { Self( src.0, src.1 ) }
}
impl From< Pair2 > for ( f32, f64 )
{
  #[ inline ]
  fn from( src : Pair2 ) -> Self { ( src.0, src.1 ) }
}
impl the_module::From_2< f32, f64 > for Pair2
{
  #[ inline ]
  fn from_2( _0 : f32, _1 : f64 ) -> Self { Self( _0, _1 ) }
}

include!("./pair_parameter_main_test_only.rs");
