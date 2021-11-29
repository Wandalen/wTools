
use wtools::str;

// //

// #[test]
// #[should_panic]
// fn split_trivial()
// {
//   let mut opts = str::split::default();
//   opts.delimeter( vec![ "" ] );
//   let got = str::split( &opts );
//   let exp : Vec<&str> = vec![];
//   assert_eq!( got, exp );
// }

// //

// #[test]
// fn split_fast_preserving_empty1_preserving_delimenter1()
// {
//   let mut opts = str::split_fast::default() ;
//   opts.delimeter( vec![ "" ] );
//   let got = str::split_fast( &opts );
//   let exp: Vec<&str> = vec![];
//   assert_eq!( got, exp );

//   let mut opts = str::split_fast::default() ;
//   opts.src( String::from( "abc" ) );
//   opts.delimeter( vec![ "" ] );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a", "", "b", "", "c" ] );

//   let mut opts = str::split_fast::default() ;
//   opts.src( String::from( "" ) );
//   opts.delimeter( vec![ "a" ] );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "a b" ) );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a", " ", "b" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "a   b" ) );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a", " ", "", " ", "", " ", "b" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "a   b" ) );
//   opts.delimeter( vec![ "c" ] );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a   b" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "a   b a   b" ) );
//   opts.delimeter( vec![ "a" ] );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "", "a", "   b ", "a", "   b" ] );

//   /* */

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( ".a" ) );
//   opts.delimeter( vec![ ".", "#" ] );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "", ".", "a" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "a." ) );
//   opts.delimeter( vec![ ".", "#" ] );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a", ".", "" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "Aa <<! <<- Bb" ) );
//   opts.delimeter( vec![ "->>>", "<<<-", "->>", "<<-", "!>>", "<<!", ">>", "<<", " " ] );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "Aa", " ", "", "<<!", "", " ", "", "<<-", "", " ", "Bb" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "Aa <<<- Bb" ) );
//   opts.delimeter( vec![ "->>>", "<<<-", "->>", "<<-", "!>>", "<<!", ">>", "<<" ] );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "Aa ", "<<<-", " Bb" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "." ) );
//   opts.delimeter( vec![ "." ] );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "", ".", "" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "..." ) );
//   opts.delimeter( vec![ "." ] );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "", ".", "", ".", "", ".", "" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "\"a b\" x \"\" c" ) );
//   opts.delimeter( vec![ "a b", " ", " c", "\"", "" ] );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "", "\"", "", "a b", "", "\"", "", " ", "x", "", "", " ", "", "\"", "", "\"", "", " ", "c" ] );
// }

// //

// #[test]
// fn split_fast_preserving_empty1_preserving_delimenter0()
// {
//   let mut opts = str::split_fast::default() ;
//   opts.delimeter( vec![ "" ] );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   let exp: Vec<&str> = vec![ "", "" ];
//   assert_eq!( got, exp );

//   let mut opts = str::split_fast::default() ;
//   opts.src( String::from( "abc" ) );
//   opts.delimeter( vec![ "" ] );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "", "a", "b", "c", "" ] );

//   let mut opts = str::split_fast::default() ;
//   opts.src( String::from( "" ) );
//   opts.delimeter( vec![ "a" ] );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "a b" ) );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a", "b" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "a   b" ) );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a", "", "", "b" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "a   b" ) );
//   opts.delimeter( vec![ "c" ] );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a   b" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "a   b a   b" ) );
//   opts.delimeter( vec![ "a" ] );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "", "   b ", "   b" ] );

//   /* */

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( ".a" ) );
//   opts.delimeter( vec![ ".", "#" ] );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "", "a" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "a." ) );
//   opts.delimeter( vec![ ".", "#" ] );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a", "" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "Aa <<! <<- Bb" ) );
//   opts.delimeter( vec![ "->>>", "<<<-", "->>", "<<-", "!>>", "<<!", ">>", "<<", " " ] );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "Aa", "", "", "", "", "Bb" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "Aa <<<- Bb" ) );
//   opts.delimeter( vec![ "->>>", "<<<-", "->>", "<<-", "!>>", "<<!", ">>", "<<" ] );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "Aa ", " Bb" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "." ) );
//   opts.delimeter( vec![ "." ] );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "", "" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "..." ) );
//   opts.delimeter( vec![ "." ] );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "", "", "", "" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "\"a b\" x \"\" c" ) );
//   opts.delimeter( vec![ "a b", " ", " c", "\"", "" ] );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "", "", "", "", "x", "", "", "", "", "c" ] );
// }

// //

// #[test]
// fn split_fast_preserving_empty0_preserving_delimenter1()
// {
//   let mut opts = str::split_fast::default() ;
//   opts.delimeter( vec![ "" ] );
//   opts.preserving_empty( false );
//   let got = str::split_fast( &opts );
//   let exp: Vec<&str> = vec![];
//   assert_eq!( got, exp );

//   let mut opts = str::split_fast::default() ;
//   opts.src( String::from( "abc" ) );
//   opts.delimeter( vec![ "" ] );
//   opts.preserving_empty( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a", "b", "c" ] );

//   let mut opts = str::split_fast::default() ;
//   opts.src( String::from( "" ) );
//   opts.delimeter( vec![ "a" ] );
//   opts.preserving_empty( false );
//   let got = str::split_fast( &opts );
//   let exp: Vec<&str> = vec![];
//   assert_eq!( got, exp );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "a b" ) );
//   opts.preserving_empty( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a", " ", "b" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "a   b" ) );
//   opts.preserving_empty( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a", " ", " ", " ", "b" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "a   b" ) );
//   opts.delimeter( vec![ "c" ] );
//   opts.preserving_empty( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a   b" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "a   b a   b" ) );
//   opts.delimeter( vec![ "a" ] );
//   opts.preserving_empty( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a", "   b ", "a", "   b" ] );

//   /* */

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( ".a" ) );
//   opts.delimeter( vec![ ".", "#" ] );
//   opts.preserving_empty( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ ".", "a" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "a." ) );
//   opts.delimeter( vec![ ".", "#" ] );
//   opts.preserving_empty( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a", "." ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "Aa <<! <<- Bb" ) );
//   opts.delimeter( vec![ "->>>", "<<<-", "->>", "<<-", "!>>", "<<!", ">>", "<<", " " ] );
//   opts.preserving_empty( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "Aa", " ", "<<!", " ", "<<-", " ", "Bb" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "Aa <<<- Bb" ) );
//   opts.delimeter( vec![ "->>>", "<<<-", "->>", "<<-", "!>>", "<<!", ">>", "<<" ] );
//   opts.preserving_empty( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "Aa ", "<<<-", " Bb" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "." ) );
//   opts.delimeter( vec![ "." ] );
//   opts.preserving_empty( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "." ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "..." ) );
//   opts.delimeter( vec![ "." ] );
//   opts.preserving_empty( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ ".", ".", "." ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "\"a b\" x \"\" c" ) );
//   opts.delimeter( vec![ "a b", " ", " c", "\"", "" ] );
//   opts.preserving_empty( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "\"", "a b", "\"", " ", "x", " ", "\"", "\"", " ", "c" ] );
// }

// //

// #[test]
// fn split_fast_preserving_empty0_preserving_delimenter0()
// {
//   let mut opts = str::split_fast::default() ;
//   opts.delimeter( vec![ "" ] );
//   opts.preserving_empty( false );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   let exp: Vec<&str> = vec![];
//   assert_eq!( got, exp );

//   let mut opts = str::split_fast::default() ;
//   opts.src( String::from( "abc" ) );
//   opts.delimeter( vec![ "" ] );
//   opts.preserving_empty( false );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a", "b", "c" ] );

//   let mut opts = str::split_fast::default() ;
//   opts.src( String::from( "" ) );
//   opts.delimeter( vec![ "a" ] );
//   opts.preserving_empty( false );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   let exp: Vec<&str> = vec![];
//   assert_eq!( got, exp );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "a b" ) );
//   opts.preserving_empty( false );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a", "b" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "a   b" ) );
//   opts.preserving_empty( false );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a", "b" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "a   b" ) );
//   opts.delimeter( vec![ "c" ] );
//   opts.preserving_empty( false );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a   b" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "a   b a   b" ) );
//   opts.delimeter( vec![ "a" ] );
//   opts.preserving_empty( false );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "   b ", "   b" ] );

//   /* */

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( ".a" ) );
//   opts.delimeter( vec![ ".", "#" ] );
//   opts.preserving_empty( false );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "a." ) );
//   opts.delimeter( vec![ ".", "#" ] );
//   opts.preserving_empty( false );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "a" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "Aa <<! <<- Bb" ) );
//   opts.delimeter( vec![ "->>>", "<<<-", "->>", "<<-", "!>>", "<<!", ">>", "<<", " " ] );
//   opts.preserving_empty( false );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "Aa", "Bb" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "Aa <<<- Bb" ) );
//   opts.delimeter( vec![ "->>>", "<<<-", "->>", "<<-", "!>>", "<<!", ">>", "<<" ] );
//   opts.preserving_empty( false );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "Aa ", " Bb" ] );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "." ) );
//   opts.delimeter( vec![ "." ] );
//   opts.preserving_empty( false );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   let exp: Vec<&str> = vec![];
//   assert_eq!( got, exp );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "..." ) );
//   opts.delimeter( vec![ "." ] );
//   opts.preserving_empty( false );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   let exp: Vec<&str> = vec![];
//   assert_eq!( got, exp );

//   let mut opts = str::split_fast::default();
//   opts.src( String::from( "\"a b\" x \"\" c" ) );
//   opts.delimeter( vec![ "a b", " ", " c", "\"", "" ] );
//   opts.preserving_empty( false );
//   opts.preserving_delimeters( false );
//   let got = str::split_fast( &opts );
//   assert_eq!( got, vec![ "x", "c" ] );
// }

// //

// /*
//   split_trivial

//   split_fast_preserving_empty1_preserving_delimenter1
//   split_fast_preserving_empty1_preserving_delimenter0
//   split_fast_preserving_empty0_preserving_delimenter1
//   split_fast_preserving_empty0_preserving_delimenter0
// */
