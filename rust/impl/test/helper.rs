/// Internal namespace.
pub( crate ) mod private
{
  // use meta_tools::*;
//
//   /* xxx : qqq : move to mem_tools. discuss */
//
//   ///
//   /// Are two pointers are the same, not taking into accoint type.
//   ///
//   /// Unlike `std::ptr::eq()` does not require arguments to have the same type.
//   ///
//
//   pub fn mem_same_ptr< T1 : ?Sized, T2 : ?Sized >( src1 : &T1, src2 : &T2 ) -> bool
//   {
//     let mem1 = src1 as *const _ as *const ();
//     let mem2 = src2 as *const _ as *const ();
//     mem1 == mem2
//   }
//
//   ///
//   /// Are two pointers points on data of the same size.
//   ///
//
//   pub fn mem_same_size< T1 : ?Sized, T2 : ?Sized >( _src1 : &T1, _src2 : &T2 ) -> bool
//   {
//     core::mem::size_of_val( _src1 ) == core::mem::size_of_val( _src2 )
//   }
//
//   ///
//   /// Are two pointers points on the same region.
//   ///
//
//   pub fn mem_same_region< T1 : ?Sized, T2 : ?Sized >( src1 : &T1, src2 : &T2 ) -> bool
//   {
//     mem_same_ptr( src1, src2 ) && mem_same_size( src1, src2 )
//   }
//
//   /* zzz : qqq : implement mem_same_region, comparing also data */

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

//

meta_tools::mod_interface!
{
  prelude use num;
}
