/// Internal namespace.
pub( crate ) mod private
{

  ///
  /// Type constructor of many.
  ///
  /// Should not be used directly. Instead use macro [crate::types!].
  /// Type constructor `many` is available if eiter feature `use_std` or feature `use_alloc` is enabled. Also feature `many` should be enabled.
  ///

  #[ macro_export ]
  macro_rules! _many
  {
    ( $( $Rest:tt )* )
    =>
    {
      compile_error!
      (
        concat!
        (
          "! Type constructor `many` is available if eiter feature `use_std` or feature `use_alloc` is enabled. Also feature `many` should be enabled.\n",
        )
      );
    }
  }

  pub use _many;
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

  #[ doc( inline ) ]
  pub use private::
  {
    _many,
  };

}


/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
