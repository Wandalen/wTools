use type_constructor::prelude::*;

fn main()
{
  types!( many Bad : < T > );
  Bad::from( ( 1, 2 ) );
}
