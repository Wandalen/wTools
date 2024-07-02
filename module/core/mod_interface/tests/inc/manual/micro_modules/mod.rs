
use super::*;

/// Private namespace of the module.
mod private
{
}

pub mod mod_protected;
pub mod mod_orphan;
pub mod mod_exposed;
pub mod mod_prelude;

/// Protected namespace of the module.
#[ allow( unused_imports ) ]
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
  pub use super::mod_protected;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
  pub use super::mod_orphan;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use super::prelude::*;
  pub use super::mod_exposed;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  pub use super::mod_prelude;
}

//

include!( "../../only_test/micro_modules_only_test.rs" );
