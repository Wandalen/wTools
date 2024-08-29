//! Multidimensional math utilities.
//!
//! Provides functionality for converting multidimensional indices into flat offsets,
//! useful for operations involving multidimensional arrays or grids.

/// Internal namespace.
mod private
{
  use core::
  {
    fmt,
    ops::{ Add, Mul },
    cmp::PartialOrd,
  };

  /// Trait for converting a multidimensional index into a flat offset.
  ///
  /// This trait is implemented for 3-dimensional arrays, allowing conversion of a
  /// 3D index into a single linear offset. It is useful for mapping coordinates in
  /// a 3D space to a flat array.
  pub trait MdOffset< T >
  {
    /// Converts a 3D index into a flat offset.
    ///
    /// # Arguments
    ///
    /// - `md_index`: A 3-element array representing the multidimensional index.
    ///
    /// # Returns
    ///
    /// A value of type `T` representing the flat offset.
    fn md_offset( & self, md_index : [ T ; 3 ] ) -> T;
  }

  impl< T > MdOffset< T > for [ T ; 3 ]
  where
    T : Mul< T, Output = T > + Add< T, Output = T > + PartialOrd + Copy + fmt::Debug,
  {
    fn md_offset( & self, md_index : [ T ; 3 ] ) -> T
    {
      debug_assert!( md_index[ 0 ] < self[ 0 ], "md_index : {md_index:?} | md_size : {self:?}" );
      debug_assert!( md_index[ 1 ] < self[ 1 ], "md_index : {md_index:?} | md_size : {self:?}" );
      debug_assert!( md_index[ 2 ] < self[ 2 ], "md_index : {md_index:?} | md_size : {self:?}" );
      let m1 = self[ 0 ];
      let m2 = m1 * self[ 1 ];
      md_index[ 0 ] + m1 * md_index[ 1 ] + m2 * md_index[ 2 ]
    }
  }

}

#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;

  #[ doc( inline ) ]
  pub use private::
  {
    MdOffset,
  };

}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use super::super::md_math;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}

