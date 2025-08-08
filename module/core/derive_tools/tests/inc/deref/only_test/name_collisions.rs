
use super::*;
use super::*;


use super::*;

#[ test ]
fn deref()
{
  let a = NameCollisions { a : 5, b : "boo".into() };
  let exp = &5;
  let got = &*a;
  assert_eq!(got, exp);
}
