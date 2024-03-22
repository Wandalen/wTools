use type_constructor::*;

fn main()
{

  types!
  {
    /// This is also attribute and macro understands it.
    #[ derive( Debug ) ]
    single MySingle : i32;
  }
  let x = MySingle( 13 );
  dbg!( x );

}
