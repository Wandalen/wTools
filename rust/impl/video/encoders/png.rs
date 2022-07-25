/// Private namespace.
pub( crate ) mod private
{
  use std::fmt::{ Debug, Formatter };
  use crate::common::prelude::*;
  use wtools::error::BasicError;
  use ::apng::{ Config, Encoder, Frame, PNGImage };
  use ::png::{ BitDepth, FilterType };


  /// Encoder for the buffer.
  // #[ derive( Former ) ]
  pub struct Png
  {
    /// Frame width.
    width : usize,
    /// Frame height.
    height : usize,
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
      .field( "width", &self.width )
      .field( "height", &self.height )
      .field( "frame_rate", &self.frame_rate )
      .field( "color_type", &self.color_type )
      .field( "output_filename", &self.output_filename )
      .finish()
     }
  }

  impl EncodeData for Png
  {
    /// Encode bytes buffer to output.
    fn encode( &mut self, data : impl AsRef< [ u8 ] > ) -> Result< (), Box<dyn std::error::Error > >
    {
      #[ allow( unreachable_patterns ) ]
      match self.color_type
      {
        ColorType::Rgb =>
        {
          let image = PNGImage
          {
            width : self.width as _,
            height : self.height as _,
            data : data.as_ref().to_vec(),
            bit_depth : BitDepth::Eight,
            color_type : ::png::ColorType::RGB,
          };
          self.images_buffer.push( image );
          Ok( () )
        },
        _ => unimplemented!( "not implemented" ),
      }
    }

    /// Finish encoding.
    fn flush( &mut self ) -> Result< (), Box<dyn std::error::Error > >
    {
      let mut out = std::io::BufWriter::new( std::fs::File::create( &self.output_filename )? );

      let config = Config
      {
        width : self.width as _,
        height : self.height as _,
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
        let encoded = encoder.write_frame( &image, frame.clone() );
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
    pub fn new( width : usize, height : usize, frame_rate : usize, repeat : Option< usize >, color_type : &ColorType, filename : impl AsRef< str > ) -> Result< Self, Box< dyn std::error::Error > >
    {
      let repeat = match repeat
      {
        Some( n ) => n as u32,
        None => 0 as u32,
      };

      let instance = Self
      {
        width,
        height,
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
