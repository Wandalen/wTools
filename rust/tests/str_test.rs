
use wtools::str;

#[test]
fn trivial() {

    let opts = str::split_fast::default() ;
    assert_eq!( opts.src, String::from( "" ) );

    let opts = str::split_fast::default() ;
    let got = str::split_fast::split( &opts ) ;
    assert_eq!( got, vec![ "" ] );

    let mut opts = str::split_fast::default();
    opts.src( String::from( "a b" ) );
    let got = str::split_fast::split( &opts ) ;
    assert_eq!( got, vec![ "a", "b" ] );
}
