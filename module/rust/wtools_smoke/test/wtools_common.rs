
#[ allow( unused_imports ) ]
use super::wtools;

pub fn wtools_time()
{
  let now = wtools::time::now();
  assert!( now < 0 );
}
