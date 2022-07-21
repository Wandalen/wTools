use super::*;

tests_impls! {
  fn basic_minimp4_muxer()
  {
    use std::io::{ Cursor, Read, Seek, SeekFrom };
    use super::minimp4::Mp4Muxer;
    use super::openh264::encoder::{ Encoder, EncoderConfig };

    let config = EncoderConfig::new( 100, 100 );
    let mut encoder = Encoder::with_config( config ).unwrap();

    let mut write_frame_to_buf = | buf: &mut Vec< u8 >, frame: &[ u8 ] |
    {
      let mut yuv = openh264::formats::RBGYUVConverter::new( 100, 100 );
      yuv.convert( frame );

      let bitstream = encoder.encode( &yuv ).unwrap();
      bitstream.write_vec( buf );
    };

    let mut buf = Vec::new();

    let mut frame = [ 255u8; 30_000 ];
    frame[ 0 ] = 0;
    frame[ 1 ] = 0;
    frame[ 2 ] = 0;
    write_frame_to_buf( &mut buf, frame.as_slice() );

    for i in 1..100
    {
      frame[ ( i - 1 ) * 3 + ( i - 1 ) * 300 ] = 255;
      frame[ ( i - 1 ) * 3 + 1 + ( i - 1 ) * 300 ] = 255;
      frame[ ( i - 1 ) * 3 + 2 + ( i - 1 ) * 300 ] = 255;

      frame[ i * 3 + i * 300 ] = 0;
      frame[ i * 3 + 1 + i * 300 ] = 0;
      frame[ i * 3 + 2 + i * 300 ] = 0;
      write_frame_to_buf( &mut buf, frame.as_slice() );
    }

    let mut video_buffer = Cursor::new( Vec::new() );
    let mut mp4muxer = Mp4Muxer::new( &mut video_buffer );
    mp4muxer.init_video( 100, 100, false, "Moving circle." );
    mp4muxer.write_video( &buf );
    mp4muxer.close();

    video_buffer.seek( SeekFrom::Start( 0 ) ).unwrap();
    let mut video_bytes = Vec::new();
    video_buffer.read_to_end( &mut video_bytes ).unwrap();

    std::fs::write( "../../../target/out_minimp4.mp4", &video_bytes ).unwrap();
  }

  //

  fn basic_ac_ffmpeg_muxer()
  {
    use std::fs::File;

    use super::ac_ffmpeg::
    {
      packet::PacketMut,
      codec::{ CodecParameters, VideoCodecParameters },
      format::
      {
        io::IO,
        muxer::{ Muxer, OutputFormat },
      },
      time::{ TimeBase, Timestamp },
      Error,
    };
    use openh264::encoder::{ Encoder, EncoderConfig };

    fn open_output( path : &str, codec_parameters : &CodecParameters ) -> Result< Muxer< File >, Error >
    {
      let output_format = OutputFormat::guess_from_file_name( path )
      .ok_or_else( || Error::new( format!( "unable to guess output format for file: {}", path ) ) )?;

      let output = File::create( path )
      .map_err( | err | Error::new( format!( "unable to create output file {}: {}", path, err ) ) )?;

      let io = IO::from_seekable_write_stream( output );

      let mut muxer_builder = Muxer::builder();

      muxer_builder.add_stream( codec_parameters )?;

      muxer_builder.build( io, output_format )
    }

    /* */

    let codec_parameters = CodecParameters::from
    (
      VideoCodecParameters::builder( "libx264" ).unwrap()
      .width( 100 )
      .height( 100 )
      .build()
    );

    let config = EncoderConfig::new( 100, 100 );
    let mut encoder = Encoder::with_config( config ).unwrap();

    let mut write_frame_to_buf = | buf: &mut Vec< u8 >, frame: &[ u8 ] |
    {
      let mut yuv = openh264::formats::RBGYUVConverter::new( 100, 100 );
      yuv.convert( frame );

      let bitstream = encoder.encode( &yuv ).unwrap();
      bitstream.write_vec( buf );
    };

    let mut muxer = open_output( "../../../target/out_ac_ffmpeg.mp4", &codec_parameters ).unwrap();
    let mut buf = vec![];

    let time_base = TimeBase::new( 1, 60 );
    let mut frame_idx = 0;
    let mut frame_timestamp = Timestamp::new( frame_idx, time_base );

    let mut frame = [ 255u8; 30_000 ];
    frame[ 0 ] = 0;
    frame[ 1 ] = 0;
    frame[ 2 ] = 0;
    write_frame_to_buf( &mut buf, frame.as_slice() );

    let packet = PacketMut::from( &buf )
    .with_pts( frame_timestamp )
    .with_dts( frame_timestamp )
    .freeze();
    muxer.push( packet ).unwrap();
    buf.clear();

    for i in 1..100
    {
      frame_idx += 1;
      frame_timestamp = Timestamp::new( frame_idx, time_base );

      frame[ ( i - 1 ) * 3 + ( i - 1 ) * 300 ] = 255;
      frame[ ( i - 1 ) * 3 + 1 + ( i - 1 ) * 300 ] = 255;
      frame[ ( i - 1 ) * 3 + 2 + ( i - 1 ) * 300 ] = 255;

      frame[ i * 3 + i * 300 ] = 0;
      frame[ i * 3 + 1 + i * 300 ] = 0;
      frame[ i * 3 + 2 + i * 300 ] = 0;
      write_frame_to_buf( &mut buf, frame.as_slice() );

      let packet = PacketMut::from( &buf )
      .with_pts( frame_timestamp )
      .with_dts( frame_timestamp )
      .freeze();
      muxer.push( packet ).unwrap();
      buf.clear();
    }

    muxer.flush().unwrap();
  }
}

//

tests_index!
{
  basic_minimp4_muxer,
  basic_ac_ffmpeg_muxer,
}
