/// Internal namespace.
mod private
{
  // use crate::own::*;

  ///
  /// Are two pointers points on the same data.
  ///
  /// Does not require arguments to have the same type.
  ///

  pub fn same_data< T1 : ?Sized, T2 : ?Sized >( src1 : &T1, src2 : &T2 ) -> bool
  {
    extern "C" { fn memcmp( s1 : *const u8, s2 : *const u8, n : usize ) -> i32; }

    let mem1 = src1 as *const _ as *const u8;
    let mem2 = src2 as *const _ as *const u8;

    if !same_size( src1, src2 )
    {
      return false;
    }

    // Unsafe block is required because we're calling a foreign function (memcmp)
    // and manually managing memory addresses.
    #[ allow( unsafe_code ) ]
    unsafe { memcmp( mem1, mem2, core::mem::size_of_val( src1 ) ) == 0 }
  }

  /* zzz : qqq : implement mem::same_data, comparing data. discuss */

  ///
  /// Are two pointers are the same, not taking into accoint type.
  ///
  /// Unlike `std::ptr::eq()` does not require arguments to have the same type.
  ///

  pub fn same_ptr< T1 : ?Sized, T2 : ?Sized >( src1 : &T1, src2 : &T2 ) -> bool
  {
    let mem1 = src1 as *const _ as *const ();
    let mem2 = src2 as *const _ as *const ();
    mem1 == mem2
  }

  ///
  /// Are two pointers points on data of the same size.
  ///

  pub fn same_size< T1 : ?Sized, T2 : ?Sized >( _src1 : &T1, _src2 : &T2 ) -> bool
  {
    core::mem::size_of_val( _src1 ) == core::mem::size_of_val( _src2 )
  }

  ///
  /// Are two pointers points on the same region, ie same size and same pointer.
  ///
  /// Does not require arguments to have the same type.
  ///

  pub fn same_region< T1 : ?Sized, T2 : ?Sized >( src1 : &T1, src2 : &T2 ) -> bool
  {
    same_ptr( src1, src2 ) && same_size( src1, src2 )
  }

}

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::
  {
    orphan::*,
  };
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
  pub use super::
  {
    exposed::*,
    private::same_data,
    private::same_ptr,
    private::same_size,
    private::same_region,
  };
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
