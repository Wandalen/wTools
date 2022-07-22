use super::*;
use wtools::error::BasicError;

tests_impls!
{
  fn basic() -> Result< (), Box< dyn std::error::Error > >
  {
    use super::apng::{ Config, Encoder, Frame, PNGImage };
    use super::png::{ ColorType, BitDepth, FilterType };
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;

    let path = Path::new( "../../../target/out.png" );
    let mut out = BufWriter::new( File::create( path )? );

    let config = Config
    {
      width : 100,
      height : 100,
      num_frames : 10,
      num_plays : 0,
      color : ColorType::RGB,
      depth : BitDepth::Eight,
      filter : FilterType::NoFilter,
    };
    let encoder_res = Encoder::new( &mut out, config );
    if encoder_res.is_err()
    {
      return Err( Box::new( BasicError::new( "cannot build encoder" ) ) );
    }
    let mut encoder = encoder_res.unwrap();

    let frame = Frame
    {
      delay_num : Some( 1 ),
      delay_den : Some( 100 ),
      ..Default::default()
    };

    for i in 0..100
    {
      let mut data = vec![ 255u8; 30_000 ];
      data[ i * 3 + i * 300 ] = 0;
      data[ i * 3 + 1 + i * 300 ] = 0;
      data[ i * 3 + 2 + i * 300 ] = 0;
      let image = PNGImage
      {
        width : 100,
        height : 100,
        data,
        bit_depth : BitDepth::Eight,
        color_type : ColorType::RGB,
      };
      let encoded = encoder.write_frame( &image, frame.clone() );
      if encoded.is_err()
      {
        return Err( Box::new( BasicError::new( "cannot write frame" ) ) );
      }
    }

    let finished =  encoder.finish_encode();
    if finished.is_err()
    {
      return Err( Box::new( BasicError::new( "cannot write image" ) ) );
    }
    Ok( () )
  }

  //

  fn basic_with_encoder() -> Result< (), Box< dyn std::error::Error > >
  {
    let mut encoder = super::encoders::Png::new( 100, 100, 30, None, &ColorType::Rgb, "../../../target/out_encoder.png" )?;
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
