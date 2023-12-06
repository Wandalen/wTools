//! example

fn main()
{
  #[ cfg( feature = "chrono" ) ]
  {
    use time_tools as TheModule;

    /* get milliseconds from UNIX epoch */
    let now = TheModule::now();
    println!( "now {}", now );

    /* get nanoseconds from UNIX epoch */
    let now = TheModule::now();
    let now_ns = TheModule::ns::now();
    assert_eq!( now, now_ns / 1000000 );

    /* get seconds from UNIX epoch */
    let now = TheModule::now();
    let now_s = TheModule::s::now();
    assert_eq!( now / 1000, now_s );
  }
}
