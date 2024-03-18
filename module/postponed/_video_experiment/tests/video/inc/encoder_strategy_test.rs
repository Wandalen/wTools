use super::*;

//

fn animation_write( encoder : &mut super::encoder_strategy::Encoder ) -> Result< (), Box< dyn std::error::Error > >
{
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

  encoder.flush()
}

//

fn animation_write_from_img_rgb( encoder : &mut super::encoder_strategy::Encoder ) -> Result< (), Box< dyn std::error::Error > >
{
  for i in 1..4
  {
    let path = std::path::PathBuf::from( format!( "./rust/test/video/_asset/img/rust_logo{}.png", i ) );
    let rgb_image = image::open( path )?.into_rgb8();
    let bytes = rgb_image.as_raw();
    encoder.encode( &bytes )?;
  }

  encoder.flush()
}

//

tests_impls!
{
  fn basic() -> Result< (), Box< dyn std::error::Error > >
  {
    let mut encoder_gif = super::encoder_strategy::Encoder::new( EncoderType::Gif, X2( 100, 100 ), 30, None, ColorType::Rgb, "../../../target/strategy.gif" )?;
    let mut encoder_png = super::encoder_strategy::Encoder::new( EncoderType::Png, X2( 100, 100 ), 30, None, ColorType::Rgb, "../../../target/strategy.png" )?;
    let mut encoder_mp4 = super::encoder_strategy::Encoder::new( EncoderType::Mp4, X2( 100, 100 ), 30, None, ColorType::Rgb, "../../../target/strategy.mp4" )?;
    animation_write( &mut encoder_gif )?;
    animation_write( &mut encoder_png )?;
    animation_write( &mut encoder_mp4 )?;

    let mut path = std::path::PathBuf::from( "../../../target/strategy.gif" );
    a_id!( path.exists(), true );
    path.set_extension( "png" );
    a_id!( path.exists(), true );
    path.set_extension( "mp4" );
    a_id!( path.exists(), true );

    Ok( () )
  }

  //

  fn basic_with_change() -> Result< (), Box< dyn std::error::Error > >
  {
    let mut encoder = super::encoder_strategy::Encoder::new( EncoderType::Gif, X2( 100, 100 ), 30, None, ColorType::Rgb, "../../../target/encoder_change.gif" )?;
    animation_write( &mut encoder )?;
    encoder.type_change( EncoderType::Mp4 )?;
    animation_write( &mut encoder )?;

    let mut path = std::path::PathBuf::from( "../../../target/encoder_change.gif" );
    a_id!( path.exists(), true );
    path.set_extension( "mp4" );
    a_id!( path.exists(), true );

    Ok( () )
  }

  //

  fn basic_with_images_rgb() -> Result< (), Box< dyn std::error::Error > >
  {
    let mut encoder_gif = super::encoder_strategy::Encoder::new( EncoderType::Gif, X2( 512, 512 ), 1, None, ColorType::Rgb, "../../../target/image.gif" )?;
    let mut encoder_png = super::encoder_strategy::Encoder::new( EncoderType::Png, X2( 512, 512 ), 1, None, ColorType::Rgb, "../../../target/image.png" )?;
    let mut encoder_mp4 = super::encoder_strategy::Encoder::new( EncoderType::Mp4, X2( 512, 512 ), 1, None, ColorType::Rgb, "../../../target/image.mp4" )?;
    animation_write_from_img_rgb( &mut encoder_gif )?;
    animation_write_from_img_rgb( &mut encoder_png )?;
    animation_write_from_img_rgb( &mut encoder_mp4 )?;

    let mut path = std::path::PathBuf::from( "../../../target/image.gif" );
    a_id!( path.exists(), true );
    path.set_extension( "png" );
    a_id!( path.exists(), true );
    path.set_extension( "mp4" );
    a_id!( path.exists(), true );

    Ok( () )
  }
}

//

tests_index!
{
  basic,
  basic_with_change,
  basic_with_images_rgb,
}
