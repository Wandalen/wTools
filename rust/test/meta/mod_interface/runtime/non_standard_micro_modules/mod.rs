
/// Private namespace of the module.
mod private
{
  // #[ doc( inline ) ]
  // pub use super::protected::*;
  pub use super::slim_private;

  pub fn slim_protected() -> bool { false }
  pub fn slim_orphan() -> bool { false }
  pub fn slim_exposed() -> bool { false }
  pub fn slim_prelude() -> bool { false }

}

pub mod slim_private;
pub mod slim_protected;
pub mod slim_orphan;
pub mod slim_exposed;
pub mod slim_prelude;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
  pub use super::slim_protected;
}

#[ doc( inline ) ]
pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
  pub use super::slim_orphan;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;
  pub use super::slim_exposed;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use super::slim_prelude;
}

//

include!( "../../test/non_standard_micro_modules_only_test.rs" );
