/// Private namespace.
mod private
{
  use std::fmt::{ Debug, Formatter };
  use crate::common::prelude::*;
  use crate::yuv;
  use wmath::X2;
  use ::gif::{ Encoder, Frame, Repeat };

  /// Encoder for the buffer.
  // #[ derive( Former ) ]
  pub struct Gif
  {
    /// Frame width and height.
    dims : X2< usize >,
    /// Frame rate.
    frame_rate : usize,
    /// Delay for frame.
    frame_delay : u16,
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
      .field( "width", &self.dims.0 )
      .field( "height", &self.dims.1 )
      .field( "frame_rate", &self.frame_rate )
      .field( "color_type", &self.color_type )
      .field( "output_filename", &self.output_filename )
      .finish()
     }
  }

  impl EncodeData for Gif
  {
    /// Encode bytes buffer to output.
    fn encode( &mut self, data : &[ u8 ] ) -> Result< (), Box<dyn std::error::Error > >
    {
      let mut buf = match self.color_type
      {
        ColorType::Rgb =>
        {
          Frame::from_rgb( self.dims.0 as u16, self.dims.1 as u16, data )
        },
        ColorType::Rgba =>
        {
          let mut cloned_data = data.to_vec();
          /* routine accepts mutable slice */
          Frame::from_rgba( self.dims.0 as u16, self.dims.1 as u16, cloned_data.as_mut_slice() )
        },
        ColorType::Yuv444 =>
        {
          let rgb = yuv::yuv444_to_rgb( data );
          Frame::from_rgb( self.dims.0 as u16, self.dims.1 as u16, &rgb )
        },
        ColorType::Yuv422 =>
        {
          let rgb = yuv::yuv422_to_rgb( data );
          Frame::from_rgb( self.dims.0 as u16, self.dims.1 as u16, &rgb )
        },
        ColorType::Yuv420p =>
        {
          let rgb = yuv::yuv420p_to_rgb( data, self.dims.0, self.dims.1 );
          Frame::from_rgb( self.dims.0 as u16, self.dims.1 as u16, &rgb )
        },
        ColorType::Yvu420p =>
        {
          let rgb = yuv::yvu420p_to_rgb( data, self.dims.0, self.dims.1 );
          Frame::from_rgb( self.dims.0 as u16, self.dims.1 as u16, &rgb )
        },
        ColorType::Yuv422p =>
        {
          let rgb = yuv::yuv422p_to_rgb( data, self.dims.0, self.dims.1 );
          Frame::from_rgb( self.dims.0 as u16, self.dims.1 as u16, &rgb )
        },
        ColorType::Grayscale =>
        {
          let rgb = yuv::grayscale_to_rgb( data );
          Frame::from_rgb( self.dims.0 as u16, self.dims.1 as u16, &rgb )
        },
      };
      buf.delay = self.frame_delay;

      self.encoder.write_frame( &buf )?;
      Ok( () )
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
    pub fn new
    (
      dims : X2< usize >,
      frame_rate : usize,
      repeat : Option< usize >,
      color_type : &ColorType,
      filename : impl AsRef< str >
    ) -> Result< Self, Box< dyn std::error::Error > >
    {
      let image = std::fs::File::create( filename.as_ref() )?;
      let mut encoder = Encoder::new( image, dims.0 as u16, dims.1 as u16, &[] )?;
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
        encoder.set_repeat( Repeat::Finite( 0 ) )?;
      }

      let gif_time_step = 10; // library allow write images with time step equal to 10 ms
      let frame_delay = ( 1000 / gif_time_step / frame_rate ) as u16;

      let instance = Self
      {
        dims,
        frame_rate,
        frame_delay,
        color_type : color_type.clone(),
        encoder,
        output_filename : std::path::PathBuf::from( filename.as_ref() ),
      };
      Ok( instance )
    }
  }
}

//

wtools::meta::mod_interface!
{
  prelude use Gif;
}
