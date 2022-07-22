use super::*;

tests_impls!
{
  fn basic() -> Result< (), Box< dyn std::error::Error > >
  {
    use super::gif::{ Frame, Repeat, Encoder };
    use std::fs::File;

    // let color_map = &[ 0xFF, 0xFF, 0xFF, 0, 0, 0 ];
    let ( width, height ) = ( 100, 100 );

    let mut image = File::create( "../../../target/out.gif" )?;
    let mut encoder = Encoder::new( &mut image, width, height, /* color_map */ &[] )?;
    encoder.set_repeat( Repeat::Infinite )?;

    let mut frame = [ 255u8; 30_000 ];
    frame[ 0 ] = 0;
    frame[ 1 ] = 0;
    frame[ 2 ] = 0;
    let buf = Frame::from_rgb( width, height, &frame );
    encoder.write_frame( &buf )?;

    for i in 1..100
    {
      frame[ ( i - 1 ) * 3 + ( i - 1 ) * 300 ] = 255;
      frame[ ( i - 1 ) * 3 + 1 + ( i - 1 ) * 300 ] = 255;
      frame[ ( i - 1 ) * 3 + 2 + ( i - 1 ) * 300 ] = 255;

      frame[ i * 3 + i * 300 ] = 0;
      frame[ i * 3 + 1 + i * 300 ] = 0;
      frame[ i * 3 + 2 + i * 300 ] = 0;
      let buf = Frame::from_rgb( width, height, &frame );
      encoder.write_frame( &buf )?;
    }

    Ok( () )
  }

  //

  fn basic_with_encoder() -> Result< (), Box< dyn std::error::Error > >
  {
    let mut encoder = super::encoders::Gif::new( 100, 100, 30, None, &ColorType::Rgb, "../../../target/out_encoder.gif" )?;
    let mut buf = [ 255u8; 30_000 ];
    buf[ 0 ] = 0;
    buf[ 1 ] = 0;
    buf[ 2 ] = 0;
    encoder.encode( &buf )?;

    for i in 1..100
    {
      buf[ ( i - 1 ) * 3 + ( i - 1 ) * 300 ] = 255;
      buf[ ( i - 1 ) * 3 + 1 + ( i - 1 ) * 300 ] = 255;
      buf[ ( i - 1 ) * 3 + 2 + ( i - 1 ) * 300 ] = 255;

      buf[ i * 3 + i * 300 ] = 0;
      buf[ i * 3 + 1 + i * 300 ] = 0;
      buf[ i * 3 + 2 + i * 300 ] = 0;
      encoder.encode( &buf )?;
    }
    encoder.flush()?;
    Ok( () )
  }
}

//

tests_index!
{
  basic,
  basic_with_encoder,
}
