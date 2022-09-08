use super::*;
use imagesize::size;
use png::decoder;

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

    let path = std::path::PathBuf::from( "../../../target/out_rgba.png" );
    a_id!( path.exists(), true );

    Ok( () )
  }

  //

  fn basic_imagesize() -> Result< (), Box< dyn std::error::Error > >
  {
    let mut encoder = super::encoders::Png::new( X2( 100, 100 ), 30, None, &ColorType::Rgba, "../../../target/out_rgba.png" )?;
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

    let path = std::path::PathBuf::from( "../../../target/out_rgba.png" );

    match size( path ) 
    {
      Ok( dim ) => 
      {
          a_id!( dim.width, 100 );
          a_id!( dim.height, 100 );
      }
      Err( why ) => println!( "Error getting size: {:?}", why )
    }

    Ok( () )
  }  

  //

  fn basic_pnginfo() -> Result< (), Box< dyn std::error::Error > >
  {
    let mut encoder = super::encoders::Png::new( X2( 100, 100 ), 30, None, &ColorType::Rgba, "../../../target/out_rgba.png" )?;
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

    let path = std::path::PathBuf::from( "../../../target/out_rgba.png" );
    let decoder = png::Decoder::new( File::open( path )? );

    let mut reader = decoder.read_info().expect( "Can not read the file target/out_rgba.png" );
    let animation_info = reader.info()?;    
    let mut buf = vec![ 0; reader.output_buffer_size() ];

    let info = reader.next_frame( &mut buf )?;

    a_id!( animation_info.weight, 100 );
    a_id!( animation_info.height, 100 );
    a_id!( animation_info.color_type, ColorType::Rgba );

    if let Some( animation_control ) = animation_info.animation_control
    {
      a_id!( animation_control.num_frames, 30 );
    }

    Ok( () )
  }

}

//

tests_index!
{
  basic_rgb,
  basic_rgba,
  basic_imagesize,
  basic_pnginfo,
}
