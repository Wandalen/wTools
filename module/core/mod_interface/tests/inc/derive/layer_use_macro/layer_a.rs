
use super::tools::*;

/// Private namespace of the module.
mod private
{

  #[ allow( unused_macros ) ]
  #[ macro_export ]
  /// macro1
  macro_rules! macro1
  {
    () => {};
  }

  #[ allow( unused_macros ) ]
  /// macro2
  macro_rules! macro2
  {
    () => {};
  }

  #[ allow( unused_macros ) ]
  /// macro3
  macro_rules! macro3
  {
    () => {};
  }

  #[ allow( unused_imports ) ]
  pub( crate ) use macro2;
  #[ allow( unused_imports ) ]
  use macro3;
}

//

mod_interface!
{

  // exposed( crate ) use macro1;
  exposed( crate ) use macro2;
  // exposed( crate ) use macro3;

}
