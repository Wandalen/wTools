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
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
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
pub mod prelude
{
}
