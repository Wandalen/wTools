use super::*;

tests_impls!
{
  fn basic()
  {
    use super::gif::{ Frame, Repeat, Encoder };
    use std::fs::File;

    // let color_map = &[ 0xFF, 0xFF, 0xFF, 0, 0, 0 ];
    let ( width, height ) = ( 100, 100 );

    let mut image = File::create( "../../../target/out.gif" ).unwrap();
    let mut encoder = Encoder::new( &mut image, width, height, /* color_map */ &[] ).unwrap();
    encoder.set_repeat( Repeat::Infinite ).unwrap();

    let mut frame = [ 255u8; 30_000 ];
    frame[ 0 ] = 0;
    frame[ 1 ] = 0;
    frame[ 2 ] = 0;
    let buf = Frame::from_rgb( width, height, &frame );
    encoder.write_frame( &buf ).unwrap();

    for i in 1..100
    {
      frame[ ( i - 1 ) * 3 + ( i - 1 ) * 300 ] = 255;
      frame[ ( i - 1 ) * 3 + 1 + ( i - 1 ) * 300 ] = 255;
      frame[ ( i - 1 ) * 3 + 2 + ( i - 1 ) * 300 ] = 255;

      frame[ i * 3 + i * 300 ] = 0;
      frame[ i * 3 + 1 + i * 300 ] = 0;
      frame[ i * 3 + 2 + i * 300 ] = 0;
      let buf = Frame::from_rgb( width, height, &frame );
      encoder.write_frame( &buf ).unwrap();
    }

    assert!( true );
  }
}

//

tests_index!
{
  basic,
}
