extern crate wtools as wt;

fn main()
{

  // let opts = wt::str::split_fast::default()
  // .src( String::from( "abc & def | ghi" ) )
  // .delimeter( vec![ "&", "|" ] )
  // // .preserving_empty( false )
  // // .preserving_delimeters( false )
  // .form()
  // ;
  // let got = wt::str::split_fast( &opts );
  // assert_eq!( got, vec![ "abc", "def", "ghi" ] );

  println!( "implements!( 13_i32 => Copy ) : {}", wt::implements!( 13_i32 => Copy ) );
  println!( "implements!( Box::new( 13_i32 ) => Copy ) : {}", wt::implements!( Box::new( 13_i32 ) => Copy ) );

}
