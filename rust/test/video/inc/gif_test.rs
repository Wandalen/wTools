use super::*;

tests_impls!
{
  fn basic_rgb() -> Result< (), Box< dyn std::error::Error > >
  {
    let mut encoder = super::encoders::Gif::new( 100, 100, 30, None, &ColorType::Rgb, "../../../target/out_rgb.gif" )?;
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

    let path = std::path::PathBuf::from( "../../../target/out_rgb.gif" );
    a_id!( path.exists(), true );

    Ok( () )
  }

  //

  fn basic_rgba() -> Result< (), Box< dyn std::error::Error > >
  {
    let mut encoder = super::encoders::Gif::new( 100, 100, 30, None, &ColorType::Rgba, "../../../target/out_rgba.gif" )?;
    let mut buf = [ 255u8; 40_000 ];
    buf[ 0 ] = 0;
    buf[ 1 ] = 0;
    buf[ 2 ] = 0;
    encoder.encode( &buf )?;

    for i in 1..100
    {
      buf[ ( i - 1 ) * 4 + ( i - 1 ) * 400 ] = 255;
      buf[ ( i - 1 ) * 4 + 1 + ( i - 1 ) * 400 ] = 255;
      buf[ ( i - 1 ) * 4 + 2 + ( i - 1 ) * 400 ] = 255;

      buf[ i * 4 + i * 400 ] = 0;
      buf[ i * 4 + 1 + i * 400 ] = 0;
      buf[ i * 4 + 2 + i * 400 ] = 0;
      encoder.encode( &buf )?;
    }
    encoder.flush()?;

    let path = std::path::PathBuf::from( "../../../target/out_rgba.gif" );
    a_id!( path.exists(), true );

    Ok( () )
  }
}

//

tests_index!
{
  basic_rgb,
  basic_rgba,
}
