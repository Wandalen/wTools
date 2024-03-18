use super::*;

// zzz : remove
// pub use wmath::X2;
// pub use wmath::X2BasicInterface;

//

tests_impls!
{

  // #[ignore]
  fn without()
  {
    use the_module::math::X2;
    use the_module::prelude::*;

    let file_name = "./test.png";
    let dims = X2::make( 32, 32 );
    let mut imgbuf = image::ImageBuffer::new( dims.0, dims.1 );

    for x in 0 ..= 30
    {
      let y = 0;
      *imgbuf.get_pixel_mut( x, y ) = image::Rgb( [ 255_u8, 0_u8, 255_u8 ] );
    }

    for x in 1 ..= 31
    {
      let y = 31;
      *imgbuf.get_pixel_mut( x, y ) = image::Rgb( [ 255_u8, 0_u8, 255_u8 ] );
    }

    for y in 0 ..= 30
    {
      let x = 31;
      *imgbuf.get_pixel_mut( x, y ) = image::Rgb( [ 255_u8, 0_u8, 255_u8 ] );
    }

    for y in 1 ..= 31
    {
      let x = 0;
      *imgbuf.get_pixel_mut( x, y ) = image::Rgb( [ 255_u8, 0_u8, 255_u8 ] );
    }

    imgbuf.save( file_name ).unwrap();
    // open::that( file_name ).unwrap();

  }

  //

  // #[ignore]
//   fn basic()
//   {
//     use the_module::math::X2;
//     use the_module::prelude::*;

//     // let c = the_module::context::make();
//     let mut c = the_module::context();
//     // let c = the_module::context().new();

//     // c.canvas.size( from!( 32, 32 ) );
//     let c = c
//     // .stroke().color( [ 1.0, 0.0, 1.0 ] ).end()
//     .stroke().width( 2.0 ).color( [ 1.0, 0.0, 1.0 ] ).context()
//     // c.draw().begin();
//     // c.draw().name( "drawing1" );
//     .draw().rect().context()
//     // c.draw().rect().region( from!( 0.0, 0.0 ), from!( 1.0, 1.0 ) ).context();
//     // c.draw().end();
//     // c.draw().now();
//     ;

// //     // c.canvas().storing_to_file_path( file_name );
// //     // c.canvas().showing_file( true );
// //     c.canvas().store_to_file();

//     println!( "{:?}", c );

//   }

}

//

tests_index!
{
  without,
  // basic,
}
