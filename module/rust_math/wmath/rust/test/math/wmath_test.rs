use wtest_basic::*;
use wmath::*;

//

  // pub type u32x2_winit = winit::dpi::PhysicalSize< u32 >;
  // pub type u32x2_cgmath = cgmath::Vector2< u32 >;

fn _basic()
{

  let _a = ix2_cgmath::new( 1 , 2 );
  let _b = ix2_nalgebra::new( 1, 2 );
  let _c = ix2_winit::new( 1 , 2 );

  println!("{}", std::mem::size_of::<ix2_cgmath<i8>>());
  println!("{}", std::mem::size_of::<ix2_nalgebra<i8>>());
  println!("{}", std::mem::size_of::<ix2_winit<i8>>());

  assert_eq!( true, true );
}

//

fn _winit()
{

  let _src = ix2_cgmath::new( 1 , 2 );
  assert_eq!( std::mem::size_of::<ix2_winit<i8>>(), 2 );

}

//

test_suite!
{
  basic,
  winit,
}
