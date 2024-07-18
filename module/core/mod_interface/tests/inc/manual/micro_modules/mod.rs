
use super::*;

/// Private namespace of the module.
mod private
{
}

pub mod mod_own;
pub mod mod_orphan;
pub mod mod_exposed;
pub mod mod_prelude;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
  pub use super::mod_own;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
  pub use super::mod_orphan;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use prelude::*;
  pub use super::mod_exposed;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  pub use super::mod_prelude;
}

//

include!( "../../only_test/micro_modules_only_test.rs" );
