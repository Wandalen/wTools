use super::*;


tests_impls!
{
  fn basic_rgb() -> Result< (), Box< dyn std::error::Error > >
  {
    let mut encoder = super::encoders::Png::new( X2( 100, 100 ), 30, None, &ColorType::Rgb, "../../../target/out_rgb.png" )?;
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

    let path = std::path::PathBuf::from( "../../../target/out_rgb.png" );
    a_id!( path.exists(), true );

    let decoder = png::Decoder::new( std::fs::File::open( &path )? );

    let mut reader = decoder.read_info().expect( "Can not read the file target/out_rgb.png" );
    let animation_info = reader.0;
    let mut bytes = vec![ 0; reader.1.output_buffer_size() ];

    let info = reader.1.next_frame( &mut bytes )?;

    a_id!( animation_info.width, 100 );
    a_id!( animation_info.height, 100 );
    a_id!( animation_info.color_type, png::ColorType::RGB );

    // first frame
    a_id!( [ 0, 0, 0 ], bytes.as_slice()[ ..3 ] );
    assert_eq!( [ 255; 30_000 - 3 ], bytes.as_slice()[ 3.. ] );

    // all frames valid
    for _ in 1..100
    {
      assert!( reader.1.next_frame( &mut bytes ).is_ok() );
    }

    // last frame
    assert_eq!( buf, bytes.as_slice() );
    Ok( () )
  }

  //

  fn basic_rgba() -> Result< (), Box< dyn std::error::Error > >
  {
    let mut encoder = super::encoders::Png::new( X2( 100, 100 ), 30, None, &ColorType::Rgba, "../../../target/out_rgba.png" )?;
    let mut buf = [ 255u8; 40_000 ];
    buf[ 0 ] = 0;
    buf[ 1 ] = 0;
    buf[ 2 ] = 0;
    encoder.encode( &buf )?;

    for i in 1..50
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

    let path = std::path::PathBuf::from( "../../../target/out_rgba.png" );

    a_id!( path.exists(), true );

    let decoder = png::Decoder::new( std::fs::File::open( &path )? );

    let mut reader = decoder.read_info().expect( "Can not read the file target/out_rgba.png" );
    let animation_info = reader.0;
    let mut bytes = vec![ 0; reader.1.output_buffer_size() ];

    let info = reader.1.next_frame( &mut bytes )?;

    a_id!( animation_info.width, 100 );
    a_id!( animation_info.height, 100 );
    a_id!( animation_info.color_type, png::ColorType::RGBA );

    // first frame
    a_id!( [ 0, 0, 0 ], bytes.as_slice()[ ..3 ] );
    assert_eq!( [ 255u8; 40_000 - 3 ], bytes.as_slice()[ 3.. ] );

    // all frames valid
    for _ in 1..50
    {
      assert!( reader.1.next_frame( &mut bytes ).is_ok() );
    }

    // last frame
    assert_eq!( buf, bytes.as_slice() );

    Ok( () )
  }

  //

  fn basic_yuv() -> Result< (), Box< dyn std::error::Error > >
  {
    let mut encoder = super::encoders::Png::new( X2( 100, 100 ), 30, None, &ColorType::Yuv444, "../../../target/out_yuv.png" )?;
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

    let path = std::path::PathBuf::from( "../../../target/out_yuv.png" );
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
