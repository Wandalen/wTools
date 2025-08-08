
use super::*;
use super::*;


use super::*;

#[ test ]
fn deref()
{
  let a = GenericsLifetimes( &3 );
  let exp = &&3;
  let got = &*a;
  assert_eq!(got, exp);
}
