
// #[ allow( unused_imports ) ]
// use super::TheModule;

/// Private namespace of the module.
mod private
{
}

pub mod mod_protected;
pub mod mod_orphan;
pub mod mod_exposed;
pub mod mod_prelude;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
  pub use super::mod_protected;
}

#[ doc( inline ) ]
pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
  pub use super::mod_orphan;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;
  pub use super::mod_exposed;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{

  pub use super::mod_prelude;
}

//

include!( "../../test/non_standard_micro_modules_only_test.rs" );
