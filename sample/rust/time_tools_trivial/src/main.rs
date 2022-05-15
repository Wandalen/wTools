use time_tools::*;

fn main()
{
  /* get milliseconds from UNIX epoch */
  let now = time::now();
  println!( "now {}", now );

  /* get nanoseconds from UNIX epoch */
  let now = time::now();
  let now_ns = time::ns::now();
  assert_eq!( now, now_ns / 1000000 );

  /* get seconds from UNIX epoch */
  let now = time::now();
  let now_s = time::s::now();
  assert_eq!( now / 1000, now_s );
}
