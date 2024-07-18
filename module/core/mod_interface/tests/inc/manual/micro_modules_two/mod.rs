
use super::*;

/// Private namespace of the module.
mod private
{
}

pub mod mod_own1;
pub mod mod_orphan1;
pub mod mod_exposed1;
pub mod mod_prelude1;

pub mod mod_own2;
pub mod mod_orphan2;
pub mod mod_exposed2;
pub mod mod_prelude2;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
  pub use super::mod_own1;
  pub use super::mod_own2;
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
  pub use super::mod_orphan1;
  pub use super::mod_orphan2;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use prelude::*;
  pub use super::mod_exposed1;
  pub use super::mod_exposed2;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  pub use super::mod_prelude1;
  pub use super::mod_prelude2;
}

//

include!( "../../only_test/micro_modules_two_only_test.rs" );
