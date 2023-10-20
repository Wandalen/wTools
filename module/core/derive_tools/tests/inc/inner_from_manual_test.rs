use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq ) ]
pub struct IsTransparent( bool );

impl From< IsTransparent > for bool
{
  #[ inline( always ) ]
  fn from( src : IsTransparent ) -> Self
  {
    src.0
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Age{ age: u32 }

impl From< Age >  for u32
{
  #[ inline( always ) ]
  fn from(value: Age) -> Self 
  {
    value.age    
  }    
}

include!( "./only_test/inner_from.rs" );
