//!
//! Nice print.
//!

/// Internal namespace.
pub( crate ) mod private
{

  use core::
  {
    ops::{ Add, Mul },
    cmp::PartialOrd,
  };

  pub trait MdOffset< T >
  {
    fn md_offset( &self, md_index : [ T ; 3 ] ) -> T;
  }

  impl< T > MdOffset< T > for [ T ; 3 ]
  where
    T : Mul< T, Output = T >,
    T : Add< T, Output = T >,
    T : PartialOrd,
    T : Copy,
  {
    fn md_offset( &self, md_index : [ T ; 3 ] ) -> T
    {
      debug_assert!( md_index[ 0 ] < self[ 0 ] );
      debug_assert!( md_index[ 1 ] < self[ 1 ] );
      debug_assert!( md_index[ 2 ] < self[ 2 ] );

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

