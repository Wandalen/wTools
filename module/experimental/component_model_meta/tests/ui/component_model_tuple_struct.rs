// UI test: ComponentModel with tuple struct should fail compilation

use component_model_meta::ComponentModel;

#[ derive( Default, ComponentModel ) ]
struct Point( i32, i32 );

fn main()
{
  let _point = Point::default();
}
