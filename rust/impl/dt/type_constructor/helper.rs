/// Internal namespace.
pub( crate ) mod private
{
  use crate::exposed::*;

  ///
  /// Generate code only if feature::make is enabled.
  ///
  /// Do not use manually.
  ///

  #[ cfg( feature = "make" ) ]
  #[ macro_export ]
  macro_rules! _if_make
  {
    ( $( $Rest : tt )* ) =>
    {
      $( $Rest )*
    };
  }

  ///
  /// Generate code only if feature::make is disabled.
  ///
  /// Do not use manually.
  ///

  #[ cfg( not( feature = "make" ) ) ]
  #[ macro_export ]
  macro_rules! _if_make
  {
    ( $( $Rest : tt )* ) =>
    {
    };
  }

  pub use _if_make;
}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::
  {
    _if_make,
  };
}
