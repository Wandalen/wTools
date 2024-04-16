//!
//! Manipulations on generic arguments.
//!

/// Internal namespace.
pub( crate ) mod private
{

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
  };
}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
  };
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::protected as generics;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    prelude::*,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
