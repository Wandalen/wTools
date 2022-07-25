use super::*;

tests_impls!
{
  fn basic() -> Result< (), Box< dyn std::error::Error > >
  {
    let mut encoder_gif = super::encoder_strategy::Encoder::new( EncoderType::Gif, 100, 100, 30, None, ColorType::Rgb, "../../../target/strategy_gif.gif" )?;
    let mut encoder_png = super::encoder_strategy::Encoder::new( EncoderType::Png, 100, 100, 30, None, ColorType::Rgb, "../../../target/strategy_png.png" )?;
    let mut encoder_mp4 = super::encoder_strategy::Encoder::new( EncoderType::Mp4, 100, 100, 30, None, ColorType::Rgb, "../../../target/strategy_mp4.mp4" )?;
    let mut buf = [ 255u8; 30_000 ];
    buf[ 0 ] = 0;
    buf[ 1 ] = 0;
    buf[ 2 ] = 0;

    encoder_gif.encode( &buf )?;
    encoder_png.encode( &buf )?;
    encoder_mp4.encode( &buf )?;

    for i in 1..100
    {
      buf[ ( i - 1 ) * 3 + ( i - 1 ) * 300 ] = 255;
      buf[ ( i - 1 ) * 3 + 1 + ( i - 1 ) * 300 ] = 255;
      buf[ ( i - 1 ) * 3 + 2 + ( i - 1 ) * 300 ] = 255;

      buf[ i * 3 + i * 300 ] = 0;
      buf[ i * 3 + 1 + i * 300 ] = 0;
      buf[ i * 3 + 2 + i * 300 ] = 0;

      encoder_gif.encode( &buf )?;
      encoder_png.encode( &buf )?;
      encoder_mp4.encode( &buf )?;
    }

    encoder_gif.flush()?;
    encoder_png.flush()?;
    encoder_mp4.flush()?;
    Ok( () )
  }
}

//

tests_index!
{
  basic,
}
