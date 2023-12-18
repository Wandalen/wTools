
use super::*;
mod tools
{
  #[ allow( unused_imports ) ]
  pub use super::super::*;
}

/// Private namespace of the module.
mod private
{
}

mod_interface!
{

  /// layer_a
  layer layer_a;

}

// use macro1 as macro1b;
#[ allow( unused_imports ) ]
use macro2 as macro2b;
// use macro3 as macro3b;
