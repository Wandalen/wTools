
use wtools::str;

#[test]
fn trivial() {

    // let mut opts = str::split_fast::default() ;
    // opts.delimeter( vec![ "" ] );
    // let got = str::split_fast( &opts );
    // let exp: Vec<&str> = vec![];
    // assert_eq!( got, exp );

    // let mut opts = str::split_fast::default() ;
    // opts.src( String::from( "abc" ) );
    // opts.delimeter( vec![ "" ] );
    // let got = str::split_fast( &opts );
    // assert_eq!( got, vec![ "", "a", "b", "c", "" ] );
    //
    // let mut opts = str::split_fast::default() ;
    // opts.src( String::from( "" ) );
    // opts.delimeter( vec![ "a" ] );
    // let got = str::split_fast( &opts );
    // assert_eq!( got, vec![ "" ] );

    let mut opts = str::split_fast::default();
    opts.src( String::from( "a b" ) );
    let got = str::split_fast( &opts );
    assert_eq!( got, vec![ "a", " ", "b" ] );

    let mut opts = str::split_fast::default();
    opts.src( String::from( "a   b" ) );
    let got = str::split_fast( &opts );
    assert_eq!( got, vec![ "a", " ", "", " ", "", " ", "b" ] );

    let mut opts = str::split_fast::default();
    opts.src( String::from( "a   b" ) );
    opts.delimeter( vec![ "c" ] );
    let got = str::split_fast( &opts );
    assert_eq!( got, vec![ "a   b" ] );

    let mut opts = str::split_fast::default();
    opts.src( String::from( "a   b a   b" ) );
    opts.delimeter( vec![ "a" ] );
    let got = str::split_fast( &opts );
    assert_eq!( got, vec![ "", "a", "   b ", "a", "   b" ] );
}
