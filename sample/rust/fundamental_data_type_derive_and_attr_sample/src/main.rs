use fundamental_data_type::*;

fn main()
{

  single!
  {
    /// This is also attribute and macro understands it.
    #[ derive( Debug ) ]
    MySingle : i32;
  }
  let x = MySingle( 13 );
  dbg!( x );

}
