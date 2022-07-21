/// Private namespace.
pub( crate ) mod private
{
  use wtools::error::Result;
  use crate::common::prelude::*;
  use ::gif::{ Encoder, Frame, Repeat };

  /// Encoder for the buffer.

  // #[ derive( Debug ) ]
  pub struct Gif
  {
    /// Frame width.
    width : usize,
    /// Frame height.
    height : usize,
    /// Frame rate.
    frame_rate : usize,
    /// Color encoding.
    color_type : ColorType,
    /// Encoder for the gif.
    encoder : Encoder< std::fs::File >,
    /// Output filename.
    output_filename : std::path::PathBuf,
  }

  impl EncodeData for Gif
  {
    /// Encode bytes buffer to output.
    fn encode( &mut self, data : impl AsRef< [ u8 ] > ) -> Result< () >
    {
      match self.color_type
      {
        ColorType::Rgb =>
        {
          let buf = Frame::from_rgb( self.width as u16, self.height as u16, data.as_ref() );
          self.encoder.write_frame( &buf ).unwrap();
          Ok( () )
        },
        _ => unimplemented!( "not implemented" ),
      }

    }
    /// Finish encoding.
    fn flush( &self ) -> Result< () >
    {
      Ok( () )
    }
  }

  impl Gif
  {
    /// Create an instance.
    pub fn new( width : usize, height : usize, frame_rate : usize, repeat : Option< usize >, color_type : &ColorType, filename : impl AsRef< str > ) -> Result< Self >
    {
      let image = std::fs::File::create( filename.as_ref() ).unwrap();
      let mut encoder = Encoder::new( image, width as u16, height as u16, &[] ).unwrap();
      if let Some( n ) = repeat
      {
        match n
        {
          0 => encoder.set_repeat( Repeat::Infinite ).unwrap(),
          x => encoder.set_repeat( Repeat::Finite( x as u16 ) ).unwrap(),
        }
      }
      else
      {
        encoder.set_repeat( Repeat::Finite( 1 ) ).unwrap();
      }

      let instance = Self
      {
        width,
        height,
        frame_rate,
        color_type : color_type.clone(),
        encoder,
        output_filename : std::path::PathBuf::from( filename.as_ref() ),
      };
      Ok( instance )
    }
  }
}

wtools::meta::mod_interface!
{
  prelude use Gif;
}
