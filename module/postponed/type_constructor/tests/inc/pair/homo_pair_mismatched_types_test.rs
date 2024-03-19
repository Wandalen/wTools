use type_constructor::prelude::*;

fn main()
{
  types!( pair Bad : i32 );
  Bad( 1, "str" );
}
