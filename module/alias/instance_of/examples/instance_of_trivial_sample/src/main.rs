//! qqq : write proper descriptionpub use instance_of::*;

fn main()
{

  dbg!( instance_of!( 13_i32 => Copy ) );
  // < instance_of!( 13_i32 => Copy ) : true
  dbg!( instance_of!( Box::new( 13_i32 ) => Copy ) );
  // < instance_of!( 13_i32 => Copy ) : false

}
