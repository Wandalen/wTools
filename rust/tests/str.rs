
use wtools::str;

#[test]
fn trivial() {

    let opts = str::split_fast { src : String::from( "abc" ), delimeter : None, preservingEmpty : None, preservingDelimeters : None };
    assert_eq!( opts.src, String::from( "abc" ) );
}
