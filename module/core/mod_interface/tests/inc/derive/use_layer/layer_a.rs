
#[ allow( unused_imports ) ]
use super::tools::*;

/// Private namespace of the module.
mod private
{

  /// PrivateStruct1.
  #[ derive( Debug, PartialEq ) ]
  pub struct PrivateStruct1
  {
  }

}

/// Super struct.
#[ derive( Debug, PartialEq ) ]
pub struct SubStruct2
{
}

/// Super struct.
#[ derive( Debug, PartialEq ) ]
pub struct SubStruct3
{
}

/// Super struct.
#[ derive( Debug, PartialEq ) ]
pub struct SubStruct4
{
}

//

mod_interface!
{

  orphan use ::std::vec::Vec;
  orphan use super::private::PrivateStruct1;
  orphan use super::super::SuperStruct1;
  orphan use super::SubStruct2;
  orphan use super::{ SubStruct3, SubStruct4 };
  orphan use crate::CrateStructForTesting1;

}
