/// Private namespace.
pub( crate ) mod private
{
  use std::fmt::{ Debug, Formatter };
  use crate::common::prelude::*;
  use ::gif::{ Encoder, Frame, Repeat };

  /// Encoder for the buffer.
  // #[ derive( Former ) ]
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

  impl Debug for Gif
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      f.debug_struct( "Gif" )
      .field( "width", &self.width )
      .field( "height", &self.height )
      .field( "frame_rate", &self.frame_rate )
      .field( "color_type", &self.color_type )
      .field( "output_filename", &self.output_filename )
      .finish()
     }
  }

  impl EncodeData for Gif
  {
    /// Encode bytes buffer to output.
    fn encode( &mut self, data : impl AsRef< [ u8 ] > ) -> Result< (), Box<dyn std::error::Error > >
    {
      #[ allow( unreachable_patterns ) ]
      match self.color_type
      {
        ColorType::Rgb =>
        {
          let mut buf = Frame::from_rgb( self.width as u16, self.height as u16, data.as_ref() );
          let gif_time_step = 10; // library allow write images with time step equal to 10 ms
          buf.delay = ( 1000 / gif_time_step / self.frame_rate ) as u16;

          self.encoder.write_frame( &buf )?;
          Ok( () )
        },
        _ => unimplemented!( "not implemented" ),
      }

    }
    /// Finish encoding.
    fn flush( &mut self ) -> Result< (), Box<dyn std::error::Error > >
    {
      Ok( () )
    }
  }

  impl Gif
  {
    /// Create an instance.
    pub fn new( width : usize, height : usize, frame_rate : usize, repeat : Option< usize >, color_type : &ColorType, filename : impl AsRef< str > ) -> Result< Self, Box< dyn std::error::Error > >
    {
      let image = std::fs::File::create( filename.as_ref() )?;
      let mut encoder = Encoder::new( image, width as u16, height as u16, &[] )?;
      if let Some( n ) = repeat
      {
        match n
        {
          0 => encoder.set_repeat( Repeat::Infinite )?,
          x => encoder.set_repeat( Repeat::Finite( x as u16 ) )?,
        }
      }
      else
      {
        encoder.set_repeat( Repeat::Finite( 1 ) )?;
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
