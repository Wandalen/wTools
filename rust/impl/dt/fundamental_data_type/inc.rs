
/// Type constructor of pair.
pub mod pair;
/// Type constructor of single.
pub mod single;
/// Type constructors.
pub mod types;
/// Generic traits.
pub mod traits;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  pub use super::pair::orphan::*;
  #[ doc( inline ) ]
  pub use super::single::orphan::*;
  #[ doc( inline ) ]
  pub use super::types::orphan::*;
  #[ doc( inline ) ]
  pub use super::traits::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  pub use super::pair::exposed::*;
  #[ doc( inline ) ]
  pub use super::single::exposed::*;
  #[ doc( inline ) ]
  pub use super::types::exposed::*;
  #[ doc( inline ) ]
  pub use super::traits::exposed::*;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  pub use super::pair::prelude::*;
  #[ doc( inline ) ]
  pub use super::single::prelude::*;
  #[ doc( inline ) ]
  pub use super::types::prelude::*;
  #[ doc( inline ) ]
  pub use super::traits::prelude::*;
}
