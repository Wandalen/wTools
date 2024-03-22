//! qqq : write proper description
fn main()
{
  #[ cfg( feature = "chrono" ) ]
  {
    use time_tools as the_module;

    /* get milliseconds from UNIX epoch */
    let now = the_module::now();
    println!( "now {}", now );

    /* get nanoseconds from UNIX epoch */
    let now = the_module::now();
    let now_ns = the_module::ns::now();
    assert_eq!( now, now_ns / 1000000 );

    /* get seconds from UNIX epoch */
    let now = the_module::now();
    let now_s = the_module::s::now();
    assert_eq!( now / 1000, now_s );
  }
}
