use super::*;

tests_impls!
{
  fn basic_rgb() -> Result< (), Box< dyn std::error::Error > >
  {
    let mut encoder = super::encoders::Mp4::new( X2( 100, 100 ), 30, None, &ColorType::Rgb, "../../../target/out_rgb.mp4" )?;
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

    let path = std::path::PathBuf::from( "../../../target/out_rgb.mp4" );
    a_id!( path.exists(), true );

    Ok( () )
  }

  //

  fn basic_rgba() -> Result< (), Box< dyn std::error::Error > >
  {
    let mut encoder = super::encoders::Mp4::new( X2( 100, 100 ), 30, None, &ColorType::Rgba, "../../../target/out_rgba.mp4" )?;
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

    let path = std::path::PathBuf::from( "../../../target/out_rgba.mp4" );
    a_id!( path.exists(), true );

    Ok( () )
  }

  //

  fn basic_yuv() -> Result< (), Box< dyn std::error::Error > >
  {
    let mut encoder = super::encoders::Mp4::new( X2( 100, 100 ), 30, None, &ColorType::Yuv444, "../../../target/out_yuv.mp4" )?;
    let mut buf : Vec< u8 > = [ [ 255u8, 128u8, 128u8 ]; 10_000 ].into_iter().flatten().collect();
    buf[ 0 ] = 0;
    buf[ 1 ] = 0;
    buf[ 2 ] = 0;
    encoder.encode( &buf )?;

    for i in 1..100
    {
      buf[ ( i - 1 ) * 3 + ( i - 1 ) * 300 ] = 255;
      buf[ ( i - 1 ) * 3 + 1 + ( i - 1 ) * 300 ] = 128;
      buf[ ( i - 1 ) * 3 + 2 + ( i - 1 ) * 300 ] = 128;

      buf[ i * 3 + i * 300 ] = 0;
      buf[ i * 3 + 1 + i * 300 ] = 0;
      buf[ i * 3 + 2 + i * 300 ] = 0;
      encoder.encode( &buf )?;
    }
    encoder.flush()?;

    let path = std::path::PathBuf::from( "../../../target/out_yuv.mp4" );
    a_id!( path.exists(), true );

    Ok( () )
  }

  //

  fn basic_yuv420p() -> Result< (), Box< dyn std::error::Error > >
  {
    let mut encoder = super::encoders::Mp4::new( X2( 100, 100 ), 30, None, &ColorType::Yuv420p, "../../../target/out_yuv420p.mp4" )?;
    let mut buf = [ 255u8; 15_000 ];
    buf[ 100 * 100.. ].fill( 128 );
    buf[ 0 ] = 0;
    encoder.encode( &buf )?;

    for i in 1..100
    {
      buf[ ( i - 1 ) + ( i - 1 ) * 100 ] = 255;

      buf[ i + i * 100 ] = 0;
      encoder.encode( &buf )?;
    }
    encoder.flush()?;

    let path = std::path::PathBuf::from( "../../../target/out_yuv420p.mp4" );
    a_id!( path.exists(), true );

    Ok( () )
  }
}

//

tests_index!
{
  basic_rgb,
  basic_rgba,
  basic_yuv,
  basic_yuv420p,
}
