use super::*;

tests_impls!
{
  fn basic()
  {
    use super::apng::{ Config, Encoder, Frame, PNGImage };
    use super::png::{ ColorType, BitDepth, FilterType };
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;

    let path = Path::new( "../../../target/out.png" );
    let mut out = BufWriter::new( File::create( path ).unwrap() );

    let config = Config
    {
      width : 100,
      height : 100,
      num_frames : 100,
      num_plays : 0,
      color : ColorType::RGB,
      depth : BitDepth::Eight,
      filter : FilterType::NoFilter,
    };
    let mut encoder = Encoder::new( &mut out, config ).unwrap();

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
      encoder.write_frame( &image, frame.clone() ).unwrap();
    }

    match encoder.finish_encode()
    {
      Err( err ) => assert!( false ),
      _ => {}
    }
  }
}

//

tests_index!
{
  basic,
}
