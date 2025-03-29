#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn md_offset_basic()
{
  use the_module::md_math::MdOffset;

  let md_size = [ 10, 100, 1000 ];
  let md_index = [ 2, 3, 4 ];
  let got = md_size.md_offset( md_index );
  let exp = 2 + 3 * 10 + 4 * 10 * 100;
  assert_eq!( got, exp );

  // 2 * 100'000

}
