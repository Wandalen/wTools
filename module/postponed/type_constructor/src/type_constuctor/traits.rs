/// Internal namespace.
mod private
{

  ///
  /// Clone as tuple.
  ///

  pub trait CloneAsTuple< Tuple >
  {
    /// Clone as tuple.
    fn clone_as_tuple( &self ) -> Tuple;
  }

  ///
  /// Clone as array.
  ///

  pub trait CloneAsArray< T, const N : usize >
  {
    /// Clone as array.
    fn clone_as_array( &self ) -> [ T ; N ];
  }

  ///
  /// Reinterpret as tuple.
  ///

  pub trait AsTuple< Tuple >
  {
    /// Reinterpret as tuple.
    fn as_tuple( &self ) -> &Tuple;
  }

  ///
  /// Reinterpret as array.
  ///

  pub trait AsArray< T, const N : usize >
  {
    /// Reinterpret as array.
    fn as_array( &self ) -> &[ T ; N ];
  }

  ///
  /// Reinterpret as slice.
  ///

  pub trait AsSlice< T >
  {
    /// Reinterpret as slice.
    fn as_slice( &self ) -> &[ T ];
  }

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
  pub use private::
  {
    CloneAsTuple,
    CloneAsArray,
    AsTuple,
    AsArray,
    AsSlice,
  };
}
