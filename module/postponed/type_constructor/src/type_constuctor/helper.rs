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

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use orphan::*;
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
  #[ allow( unused_imports ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use prelude::*;
}


/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use private::
  {
    _if_make,
  };
}
