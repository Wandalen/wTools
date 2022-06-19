use super::*;

//

tests_impls!
{

  fn basic()
  {
    use TheModule::math::X2;
    use TheModule::prelude::*;

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
    open::that( file_name ).unwrap();

  }

}

//

tests_index!
{
  basic,
}
