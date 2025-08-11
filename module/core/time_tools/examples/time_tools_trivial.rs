//! qqq : write proper description
fn main() {
  #[ cfg( feature = "chrono" ) ]
  {
    use time_tools as the_module;

    /* get milliseconds from UNIX epoch */
    let now = the_module::now::now();
    println!("now {}", now);

    /* get nanoseconds from UNIX epoch */
    let now_ms = the_module::now::now();
    let now_ns = the_module::ns::now();
    assert_eq!(now_ms, now_ns / 1_000_000);

    /* get seconds from UNIX epoch */
    let now_ms = the_module::now::now();
    let now_seconds = the_module::s::now();
    assert_eq!(now_ms / 1000, now_seconds);
  }
}
