use super::*;

fn rgba_to_rgb( rgba_buf : Vec< u8 > ) -> Vec< u8 >
{
  let mut result = vec![ 0; rgba_buf.len() * 3 / 4 ];
  let mut i = 0;
  for chunk in rgba_buf.chunks( 4 )
  {
      result[ i..i+3 ].copy_from_slice( &chunk[ 0..3 ] );
      i += 3;
  }
  result
}

tests_impls!
{
  fn basic_rgb() -> Result< (), Box< dyn std::error::Error > >
  {
    let mut encoder = super::encoders::Gif::new( X2( 100, 100 ), 30, None, &ColorType::Rgb, "../../../target/out_rgb.gif" )?;
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

    let mut decoder = gif::DecodeOptions::new();
    // must be gif::ColorOuput::RGB but it has not the variant
    decoder.set_color_output( gif::ColorOutput::RGBA );
    let mut reader = decoder.read_info( std::fs::File::open( &path )? ).expect( "Can not read the file target/out_rgb.gif" );

    reader.next_frame_info()?;
    let mut bytes = vec![ 0; reader.buffer_size() ];
    reader.read_into_buffer( &mut bytes )?;
    bytes = rgba_to_rgb( bytes );

    a_id!( reader.width(), 100 );
    a_id!( reader.height(), 100 );

    // first frame
    a_id!( [ 0, 0, 0 ], bytes[ ..3 ] );
    assert_eq!( [ 255u8; 30_000 - 3 ], bytes[ 3.. ] );

    // all frames valid
    for _ in 1..100
    {
      assert!( reader.next_frame_info().is_ok() );
    }

    // last frame
    let mut bytes = vec![ 0; reader.buffer_size() ];
    reader.read_into_buffer( &mut bytes )?;
    bytes = rgba_to_rgb( bytes );
    assert_eq!( buf, bytes.as_slice() );
    Ok( () )
  }

  //

  fn basic_rgba() -> Result< (), Box< dyn std::error::Error > >
  {
    let mut encoder = super::encoders::Gif::new( X2( 100, 100 ), 30, None, &ColorType::Rgba, "../../../target/out_rgba.gif" )?;
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

    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output( gif::ColorOutput::RGBA );
    let mut reader = decoder.read_info( std::fs::File::open( &path )? ).expect( "Can not read the file target/out_rgba.gif" );

    reader.next_frame_info()?;
    let mut bytes = vec![ 0; reader.buffer_size() ];
    reader.read_into_buffer( &mut bytes )?;

    a_id!( reader.width(), 100 );
    a_id!( reader.height(), 100 );

    // first frame
    a_id!( [ 0, 0, 0 ], bytes[ ..3 ] );
    assert_eq!( [ 255u8; 40_000 - 3 ], bytes[ 3.. ] );

    // all frames valid
    for _ in 1..100
    {
      assert!( reader.next_frame_info().is_ok() );
    }

    // last frame
    reader.read_into_buffer( &mut bytes )?;
    assert_eq!( buf, bytes.as_slice() );
    Ok( () )
  }

  //

  fn basic_yuv() -> Result< (), Box< dyn std::error::Error > >
  {
    let mut encoder = super::encoders::Gif::new( X2( 100, 100 ), 30, None, &ColorType::Yuv444, "../../../target/out_yuv.gif" )?;
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

    let path = std::path::PathBuf::from( "../../../target/out_yuv.gif" );
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
}
