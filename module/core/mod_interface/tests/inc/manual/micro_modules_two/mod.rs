
use super::*;

/// Private namespace of the module.
mod private
{
}

pub mod mod_protected1;
pub mod mod_orphan1;
pub mod mod_exposed1;
pub mod mod_prelude1;

pub mod mod_protected2;
pub mod mod_orphan2;
pub mod mod_exposed2;
pub mod mod_prelude2;

/// Protected namespace of the module.
#[ allow( unused_imports ) ]
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
  pub use super::mod_protected1;
  pub use super::mod_protected2;
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
  pub use super::mod_orphan1;
  pub use super::mod_orphan2;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use super::prelude::*;
  pub use super::mod_exposed1;
  pub use super::mod_exposed2;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  pub use super::mod_prelude1;
  pub use super::mod_prelude2;
}

//

include!( "../../only_test/micro_modules_two_only_test.rs" );
