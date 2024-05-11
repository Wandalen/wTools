//! qqq : write proper description
use mem_tools as mem;

fn main()
{

  // Are two pointers are the same, not taking into accoint type.
  // Unlike `std::ptr::eq()` does not require arguments to have the same type.
  let src1 = ( 1, );
  let src2 = ( 1, );
  assert!( !mem::same_ptr( &src1, &src2 ) );

  // Are two pointers points on data of the same size.
  let src1 = "abc";
  let src2 = "cba";
  assert!( mem::same_size( src1, src2 ) );

  // Are two pointers points on the same region, ie same size and same pointer.
  // Does not require arguments to have the same type.
  let src1 = "abc";
  let src2 = "abc";
  assert!( mem::same_region( src1, src2 ) );

}
