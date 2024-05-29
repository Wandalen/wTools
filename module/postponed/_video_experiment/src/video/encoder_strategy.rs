/// Private namespace.
pub( crate ) mod private
{
  use std::fmt::{ Debug, Formatter };
  use crate::common::prelude::*;
  use crate::encoders::{ Gif, Png, Mp4 };
  use wtools::error::BasicError;
  #[ allow( unused_imports ) ]
  use wtools::prelude::former::Former;
  use wmath::X2;

  /// Encoder for the buffer.

  /* rrr : for Dmytro : add former macro when attributes and documentation comments handling will be implemented */
  // #[ derive( Former ) ]
  pub struct Encoder
  {
    /// Frame width and height.
    dims : wmath::X2< usize >,
    /// Frame rate.
    frame_rate : usize,
    /// Color encoding.
    color_type : ColorType,
    /// Repeat animation. For animated images formats.
    repeat : Option< usize >,

    /// Type of output format.
    encoder_type : EncoderType,
    /// Encoder for the output format.
    encoder : Box< dyn EncodeData >,

    /// Output filename.
    output_filename : std::path::PathBuf,
  }

  impl Debug for Encoder
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      f.debug_struct( "Encoder" )
      .field( "width", &self.dims.0 )
      .field( "height", &self.dims.0 )
      .field( "frame_rate", &self.frame_rate )
      .field( "color_type", &self.color_type )
      .field( "encoder_type", &self.encoder_type )
      .field( "output_filename", &self.output_filename )
      .finish()
     }
  }

  impl EncodeData for Encoder
  {
    /// Encode bytes buffer to output.
    fn encode( &mut self, data : &[ u8 ] ) -> Result< (), Box< dyn std::error::Error > >
    {
      self.encoder.encode( data )
    }
    /// Finish encoding.
    fn flush( &mut self ) -> Result< (), Box<dyn std::error::Error > >
    {
      self.encoder.flush()
    }
  }

  impl Encoder
  {
    /// Create an instance.
    pub fn new
    (
      encoder_type : EncoderType,
      dims : X2< usize >,
      frame_rate : usize,
      repeat : Option< usize >,
      color_type : ColorType,
      filename : impl AsRef< str >
    ) -> Result< Self, Box< dyn std::error::Error > >
    {
      let encoder = Encoder::encoder_make( &encoder_type, &dims, frame_rate, repeat, &color_type, filename.as_ref() )?;

      let instance = Self
      {
        dims,
        frame_rate,
        color_type,
        repeat,
        encoder_type,
        encoder,
        output_filename : std::path::PathBuf::from( filename.as_ref() ),
      };
      Ok( instance )
    }

    //

    fn encoder_make
    (
      encoder_type : &EncoderType,
      dims : &X2< usize >,
      frame_rate : usize,
      repeat : Option< usize >,
      color_type : &ColorType,
      filename : &str
    ) -> Result< Box< dyn EncodeData >, Box< dyn std::error::Error > >
    {
      if encoder_type == &EncoderType::Gif
      {
        let encoder = Gif::new( *dims, frame_rate, repeat, color_type, filename )?;
        return Ok( Box::new( encoder ) );
      }
      if encoder_type == &EncoderType::Png
      {
        let encoder = Png::new( *dims, frame_rate, repeat, color_type, filename )?;
        return Ok( Box::new( encoder ) );
      }
      if encoder_type == &EncoderType::Mp4
      {
        let encoder = Mp4::new( *dims, frame_rate, repeat, color_type, filename )?;
        return Ok( Box::new( encoder ) );
      }

      Err( Box::new( BasicError::new( format!( "unknown encoder type \"{:?}\"", encoder_type ) ) ) )
    }

    //

    /// Change type of encoder.
    pub fn type_change( &mut self, encoder_type : EncoderType ) -> Result< (), Box< dyn std::error::Error > >
    {
      let changed = match encoder_type
      {
        EncoderType::Gif => self.output_filename.set_extension( "gif" ),
        EncoderType::Png => self.output_filename.set_extension( "png" ),
        EncoderType::Mp4 => self.output_filename.set_extension( "mp4" ),
      };

      if !changed
      {
        return Err( Box::new( BasicError::new( "cannot update extension" ) ) );
      }

      let encoder = Encoder::encoder_make
      (
        &encoder_type,
        &self.dims,
        self.frame_rate,
        self.repeat,
        &self.color_type,
        self.output_filename.to_str().ok_or_else( | | BasicError::new( "cannot form filename" ) )?
      )?;
      self.encoder = encoder;
      Ok( () )
    }
  }

}

wtools::meta::mod_interface!
{
  prelude use Encoder;
}
