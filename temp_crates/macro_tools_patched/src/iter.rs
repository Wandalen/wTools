//!
//! Tailored iterator.
//!

/// Define a private namespace for all its items.
mod private
{
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Tailoted iterator.
#[ allow( unused_imports ) ]
pub mod own
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use iter_tools::own::*;
}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  // pub use super::super::iter;

  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use iter_tools::exposed::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use iter_tools::prelude::*;

}
