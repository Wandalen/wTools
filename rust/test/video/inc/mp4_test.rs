use super::*;

use std::fs::File;

use ac_ffmpeg::
{
  codec::{ video::VideoDecoder },
  format::
  {
    demuxer::{ Demuxer, DemuxerWithStreamInfo },
    io::IO,
  },
  Error,
};

fn open_input( path : &str ) -> Result< DemuxerWithStreamInfo< File >, Error >
{
  let input = File::open( path )
  .map_err( | err | Error::new( format!( "Unable to open input file {} : {}", path, err ) ) )?;

  let io = IO::from_seekable_read_stream( input );

  Demuxer::builder()
  .build( io )?
  .find_stream_info( None )
  .map_err( | ( _, err ) | err )
}

tests_impls!
{
  fn basic_rgb() -> Result< (), Box< dyn std::error::Error > >
  {
    {
      let mut encoder = super::encoders::Mp4::new( X2( 100, 100 ), 30, None, &ColorType::Rgb, "../../../target/out_rgb.mp4" )?;
      let mut buf = [ 255u8; 30_000 ];
      buf[ 0 ] = 0;
      buf[ 1 ] = 0;
      buf[ 2 ] = 0;
      encoder.encode( &buf )?;

      for i in 1..50
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
    }

    let path = std::path::PathBuf::from( "../../../target/out_rgb.mp4" );
    a_id!( path.exists(), true );

    let mut demuxer = open_input( &path.to_str().unwrap() ).unwrap();

    let ( stream_index, ( stream, _ ) ) = demuxer
    .streams()
    .iter()
    .map( | stream | ( stream, stream.codec_parameters() ) )
    .enumerate()
    .find( | ( _, ( _, params ) ) | params.is_video_codec() )
    .ok_or_else( | | Error::new( "No video stream" ) )?;

    let mut decoder = VideoDecoder::from_stream( stream )?.build()?;

    let mut frames = 0;
    while let Some( packet ) = demuxer.take()?
    {
      frames += 1
    }
    assert_eq!( 50, frames );
    Ok( () )
  }

  //

  fn basic_rgba() -> Result< (), Box< dyn std::error::Error > >
  {
    {
      let mut encoder = super::encoders::Mp4::new( X2( 100, 100 ), 30, None, &ColorType::Rgba, "../../../target/out_rgba.mp4" )?;
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
    }

    let path = std::path::PathBuf::from( "../../../target/out_rgba.mp4" );
    a_id!( path.exists(), true );

    let mut demuxer = open_input( &path.to_str().unwrap() ).unwrap();

    let ( stream_index, ( stream, _ ) ) = demuxer
    .streams()
    .iter()
    .map( | stream | ( stream, stream.codec_parameters() ) )
    .enumerate()
    .find( | ( _, ( _, params ) ) | params.is_video_codec() )
    .ok_or_else( | | Error::new( "No video stream" ) )?;

    let mut decoder = VideoDecoder::from_stream( stream )?.build()?;

    let mut frames = 0;
    while let Some( packet ) = demuxer.take()?
    {
      frames += 1
    }
    assert_eq!( 50, frames );

    Ok( () )
  }
}

//

tests_index!
{
  basic_rgb,
  basic_rgba,
}
