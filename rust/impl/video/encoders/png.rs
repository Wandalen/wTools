/// Private namespace.
pub( crate ) mod private
{
  use std::fmt::{ Debug, Formatter };
  use crate::common::prelude::*;
  use crate::yuv;
  use wtools::error::BasicError;
  use wmath::X2;
  use ::apng::{ Config, Encoder, Frame, PNGImage };
  use ::png::{ BitDepth, FilterType };


  /// Encoder for the buffer.
  // #[ derive( Former ) ]
  pub struct Png
  {
    /// Frame width and height.
    dims : X2< usize >,
    /// Frame rate.
    frame_rate : usize,
    /// Color encoding.
    color_type : ColorType,
    /// Buffer for images.
    images_buffer : Vec< PNGImage >,
    /// Number of repeats.
    repeat : u32,
    /// Output filename.
    output_filename : std::path::PathBuf,
  }

  impl Debug for Png
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      f.debug_struct( "Png" )
      .field( "width", &self.dims.0 )
      .field( "height", &self.dims.1 )
      .field( "frame_rate", &self.frame_rate )
      .field( "color_type", &self.color_type )
      .field( "output_filename", &self.output_filename )
      .finish()
     }
  }

  impl EncodeData for Png
  {
    /// Encode bytes buffer to output.
    fn encode( &mut self, data : &[ u8 ] ) -> Result< (), Box<dyn std::error::Error > >
    {
      let image = match self.color_type
      {
        ColorType::Rgb =>
        {
          PNGImage
          {
            width : self.dims.0 as _,
            height : self.dims.1 as _,
            data : data.to_vec(),
            bit_depth : BitDepth::Eight,
            color_type : ::png::ColorType::RGB,
          }
        },
        ColorType::Rgba =>
        {
          PNGImage
          {
            width : self.dims.0 as _,
            height : self.dims.1 as _,
            data : data.to_vec(),
            bit_depth : BitDepth::Eight,
            color_type : ::png::ColorType::RGBA,
          }
        },
        ColorType::Yuv444 =>
        {
          PNGImage
          {
            width : self.dims.0 as _,
            height : self.dims.1 as _,
            data : yuv::yuv444_to_rgb( data ),
            bit_depth : BitDepth::Eight,
            color_type : ::png::ColorType::RGB,
          }
        },
        ColorType::Yuv422 =>
        {
          PNGImage
          {
            width : self.dims.0 as _,
            height : self.dims.1 as _,
            data : yuv::yuv422_to_rgb( data ),
            bit_depth : BitDepth::Eight,
            color_type : ::png::ColorType::RGB,
          }
        },
        ColorType::Yuv420p =>
        {
          PNGImage
          {
            width : self.dims.0 as _,
            height : self.dims.1 as _,
            data : yuv::yuv420p_to_rgb( data, self.dims.0, self.dims.1 ),
            bit_depth : BitDepth::Eight,
            color_type : ::png::ColorType::RGB,
          }
        },
        ColorType::Yvu420p =>
        {
          PNGImage
          {
            width : self.dims.0 as _,
            height : self.dims.1 as _,
            data : yuv::yvu420p_to_rgb( data, self.dims.0, self.dims.1 ),
            bit_depth : BitDepth::Eight,
            color_type : ::png::ColorType::RGB,
          }
        },
        ColorType::Yuv422p =>
        {
          PNGImage
          {
            width : self.dims.0 as _,
            height : self.dims.1 as _,
            data : yuv::yuv422p_to_rgb( data, self.dims.0, self.dims.1 ),
            bit_depth : BitDepth::Eight,
            color_type : ::png::ColorType::RGB,
          }
        },
        ColorType::Grayscale =>
        {
          PNGImage
          {
            width : self.dims.0 as _,
            height : self.dims.1 as _,
            data : yuv::grayscale_to_rgb( data ),
            bit_depth : BitDepth::Eight,
            color_type : ::png::ColorType::RGB,
          }
        },
      };

      self.images_buffer.push( image );
      Ok( () )
    }

    /// Finish encoding.
    fn flush( &mut self ) -> Result< (), Box<dyn std::error::Error > >
    {
      let mut out = std::io::BufWriter::new( std::fs::File::create( &self.output_filename )? );

      let config = Config
      {
        width : self.dims.0 as _,
        height : self.dims.1 as _,
        num_frames : self.images_buffer.len() as _,
        num_plays : self.repeat,
        color : self.images_buffer[ 0 ].color_type,
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
        delay_den : Some( self.frame_rate as _ ),
        ..Default::default()
      };

      for image in &self.images_buffer
      {
        let encoded = encoder.write_frame( image, frame.clone() );
        if encoded.is_err()
        {
          return Err( Box::new( BasicError::new( "cannot write frame" ) ) );
        }
      }
      let finished = encoder.finish_encode();
      if finished.is_err()
      {
        return Err( Box::new( BasicError::new( "cannot write image" ) ) );
      }

      Ok( () )
    }
  }

  impl Png
  {
    /// Create an instance.
    pub fn new
    (
      dims : X2< usize >,
      frame_rate : usize,
      repeat : Option< usize >,
      color_type : &ColorType,
      filename : impl AsRef< str >
    ) -> Result< Self, Box< dyn std::error::Error > >
    {
      let repeat = match repeat
      {
        Some( 0 ) => u32::MAX,
        Some( n ) => n as u32,
        None => 1_u32,
      };

      let instance = Self
      {
        dims,
        frame_rate,
        color_type : color_type.clone(),
        images_buffer : vec![],
        repeat,
        output_filename : std::path::PathBuf::from( filename.as_ref() ),
      };
      Ok( instance )
    }
  }
}

//

wtools::meta::mod_interface!
{
  prelude use Png;
}
