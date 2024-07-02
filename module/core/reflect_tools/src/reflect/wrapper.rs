//!
//! Collection of wrappers.
//!

/// Internal namespace.
pub( crate ) mod private
{
}

mod aref;
mod maybe_as;

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
#[ allow( unused_imports ) ]
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    aref::IntoRef,
    aref::Ref,
    maybe_as::IntoMaybeAs,
    maybe_as::MaybeAs,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
}
