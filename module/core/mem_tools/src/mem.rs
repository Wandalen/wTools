/// Define a private namespace for all its items.
mod private 
{
  // use crate ::own :: *;

  ///
  /// Are two pointers points on the same data.
  ///
  /// Does not require arguments to have the same type.
  #[ allow( unsafe_code ) ]
  pub fn same_data< T1: ?Sized, T2: ?Sized >(src1: &T1, src2: &T2) -> bool
  {
  extern "C" {
   fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32;
 }

  let mem1 = core ::ptr ::from_ref :: < T1 >(src1).cast :: < u8 >();
  let mem2 = core ::ptr ::from_ref :: < T2 >(src2).cast :: < u8 >();

  if !same_size(src1, src2) 
  {
   return false;
 }

  // Safety :
  // The `unsafe` block is required because we're calling a foreign function (`memcmp`)
  // and manually managing memory addresses.
  // `mem1` and `mem2` are obtained from valid references `src1` and `src2` using `core ::ptr ::from_ref`
  // and then cast to `*const u8`. This ensures they are valid, non-null, and properly aligned
  // pointers to the start of the data.
  // The size `n` is obtained from `core ::mem ::size_of_val(src1)`, which is the correct
  // size of the data pointed to by `src1`.
  // The `same_size` check (which compares `core ::mem ::size_of_val(src1)` and `core ::mem ::size_of_val(src2)`)
  // ensures that both memory regions have the same length. This guarantees that `memcmp`
  // will not read out of bounds for `src2` when comparing `n` bytes, as both `mem1` and `mem2`
  // are guaranteed to point to at least `n` bytes of valid memory.
  // SAFETY: The pointers `mem1` and `mem2` are valid for the size of the data, and both regions
  // have been verified to have the same size, ensuring no out-of-bounds access.
  unsafe { memcmp(mem1, mem2, core ::mem ::size_of_val(src1)) == 0 }
 }

  /* zzz: qqq: implement mem ::same_data, comparing data. discuss */

  ///
  /// Are two pointers are the same, not taking into accoint type.
  ///
  /// Unlike `std ::ptr ::eq()` does not require arguments to have the same type.
  pub fn same_ptr< T1: ?Sized, T2: ?Sized >(src1: &T1, src2: &T2) -> bool
  {
  let mem1 = core ::ptr ::from_ref :: < T1 >(src1).cast :: < () >();
  let mem2 = core ::ptr ::from_ref :: < T2 >(src2).cast :: < () >();
  mem1 == mem2
 }

  ///
  /// Are two pointers points on data of the same size.
  pub fn same_size< T1: ?Sized, T2: ?Sized >(src1: &T1, src2: &T2) -> bool
  {
  core ::mem ::size_of_val(src1) == core ::mem ::size_of_val(src2)
 }

  ///
  /// Are two pointers points on the same region, ie same size and same pointer.
  ///
  /// Does not require arguments to have the same type.
  pub fn same_region< T1: ?Sized, T2: ?Sized >(src1: &T1, src2: &T2) -> bool
  {
  same_ptr(src1, src2) && same_size(src1, src2)
 }
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own :: *;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own 
{
  use super :: *;
  #[ doc( inline ) ]
  pub use super :: { orphan :: * };
}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan 
{
  use super :: *;
  #[ doc( inline ) ]
  pub use super :: { exposed :: *, private ::same_data, private ::same_ptr, private ::same_size, private ::same_region };
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed 
{
  use super :: *;
  // Expose itself.
  pub use super ::super ::mem;

  #[ doc( inline ) ]
  pub use prelude :: *;
}

/// Prelude to use essentials: `use my_module ::prelude :: *`.
#[ allow( unused_imports ) ]
pub mod prelude 
{
  use super :: *;
}
