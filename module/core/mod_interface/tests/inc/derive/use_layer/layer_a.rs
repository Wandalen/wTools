#![allow(dead_code)]
#[ allow( unused_imports ) ]
use super ::tools :: *;

/// Private namespace of the module.
mod private
{

  /// `PrivateStruct1`.
  #[ derive( Debug, PartialEq ) ]
  pub struct PrivateStruct1 {}

}

/// Super struct.
#[ derive( Debug, PartialEq ) ]
pub struct SubStruct2 {}

/// Super struct.
#[ derive( Debug, PartialEq ) ]
pub struct SubStruct3 {}

/// Super struct.
#[ derive( Debug, PartialEq ) ]
pub struct SubStruct4 {}

//

the_module ::mod_interface! {

  orphan use PrivateStruct1;
  orphan use SubStruct2;
  orphan use SubStruct3;
  orphan use SubStruct4;

}
