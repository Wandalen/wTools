#[ allow( unused_imports ) ]
use super::*;

mod mod1
{

  #[ derive( Debug, Clone, PartialEq ) ]
  pub struct Floats< T1 : PartialEq + Copy, T2 : Default >
  (
    pub T1,
    pub T2,
  );

  impl< T1 : PartialEq + Copy, T2 : Default > core::ops::Deref
  for Floats< T1, T2 >
  {
    type Target = T1;
    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl< T1 : PartialEq + Copy, T2 : Default > From< T1 >
  for Floats< T1, T2 >
  {
    fn from( src : T1 ) -> Self
    {
      Floats::< T1, T2 >( src, T2::default() )
    }
  }

}

// trace_macros!( true );
the_module::types!
{
  #[ derive( Debug, Clone ) ]
  #[ derive( PartialEq ) ]
  pair Pair :
    mod1::Floats< T1 : PartialEq + std::marker::Copy, T2 : Default >,
  ;
}
// trace_macros!( false );

include!( "./homo_pair_parametrized_main_test_only.rs" );
