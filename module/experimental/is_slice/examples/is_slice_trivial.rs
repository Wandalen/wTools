//! Demonstrates basic usage of the `is_slice` macro to distinguish between slice references and array references at runtime.

use is_slice ::is_slice;

fn main() 
{
  dbg!(is_slice!(Box ::new(true)));
  // < is_slice!(Box ::new(true)) = false
  dbg!(is_slice!(&[ 1, 2, 3]));
  // < is_slice!(&[1, 2, 3]) = false
  dbg!(is_slice!(&[ 1, 2, 3][..]));
  // < is_slice!(&[1, 2, 3][..]) = true
}
