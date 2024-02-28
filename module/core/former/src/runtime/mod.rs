
//!
//! Former - variation of builder pattern. Implementation of its runtime.
//!

/// Axiomatic things.
#[ cfg( not( feature = "no_std" ) ) ]
mod axiomatic;
/// Former of a vector.
#[ cfg( not( feature = "no_std" ) ) ]
mod vector;
/// Former of a hash map.
#[ cfg( not( feature = "no_std" ) ) ]
mod hash_map;
/// Former of a hash set.
#[ cfg( not( feature = "no_std" ) ) ]
mod hash_set;

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( not( feature = "no_std" ) ) ]
  pub use super::axiomatic::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( not( feature = "no_std" ) ) ]
  pub use super::vector::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( not( feature = "no_std" ) ) ]
  pub use super::hash_map::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( not( feature = "no_std" ) ) ]
  pub use super::hash_set::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
