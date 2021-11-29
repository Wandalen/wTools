pub use implements::implements;

fn main()
{

  println!( "implements!( 13_i32 => Copy ) : {}", implements!( 13_i32 => Copy ) );
  // < implements!( 13_i32 => Copy ) : true
  println!( "implements!( Box::new( 13_i32 ) => Copy ) : {}", implements!( Box::new( 13_i32 ) => Copy ) );
  // < implements!( 13_i32 => Copy ) : false

}
