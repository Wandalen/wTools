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
#[ allow( unused_imports ) ]
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
}


/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    _if_make,
  };
}
