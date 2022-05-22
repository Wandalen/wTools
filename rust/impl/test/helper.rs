/// Internal namespace.
mod internal
{
  // use meta_tools::*;

  /* xxx : qqq : move to mem_tools. discuss */

  ///
  /// Are two pointers are the same, not taking into accoint type.
  ///
  /// Unlike `std::ptr::eq()` does not require arguments to have the same type.
  ///

  pub fn mem_same_ptr< T1 : ?Sized, T2 : ?Sized >( src1 : &T1, src2 : &T2 ) -> bool
  {
    let mem1 = src1 as *const _ as *const ();
    let mem2 = src2 as *const _ as *const ();
    // let mem1 = src1.as_ptr().cast::<()>();
    // let mem2 = src2.as_ptr().cast::<()>();
    mem1 == mem2
  }

  ///
  /// Are two pointers points on data of the same size.
  ///

  pub fn mem_same_size< T1 : ?Sized, T2 : ?Sized >( _src1 : &T1, _src2 : &T2 ) -> bool
  {
    core::mem::size_of_val( _src1 ) == core::mem::size_of_val( _src2 )
  }

//   /// Get size of memory.
//   trait MemSize< T >
//   {
//     /// Get size of memory.
//     fn size( src : &T ) -> usize;
//   }
//
//   impl< T > MemSize< T > for T
//   where
//     T : Sized,
//   {
//     fn size( src : &T ) -> usize
//     {
//       0
//     }
//   }
//
//   impl< T > MemSize< T > for T
//   where
//     T : ?Sized,
//   {
//     fn size( src : &T ) -> usize
//     {
//       0
//     }
//   }

  ///
  /// Are two pointers points on the same region.
  ///

  pub fn mem_same_region< T1 : ?Sized, T2 : ?Sized >( src1 : &T1, src2 : &T2 ) -> bool
  {
    mem_same_ptr( src1, src2 ) && mem_same_size( src1, src2 )
  }

  /* zzz : qqq : implement mem_same_region, comparing also data */

  ///
  /// Required to convert integets to floats.
  ///

  #[ macro_export ]
  macro_rules! num
  {

    () =>
    {
    };

    ( $num : expr ) =>
    {
      num_traits::cast::< _, T >( $num ).unwrap()
    };

    ( $( $num : expr ),+ ) =>
    {(
      $( num_traits::cast::< _, T >( $num ).unwrap() ),+
    )};

  }

  pub use num;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use super::internal::
  {
    mem_same_ptr,
    mem_same_size,
    mem_same_region,
    num,
  };
}
