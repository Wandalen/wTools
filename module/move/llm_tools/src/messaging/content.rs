//! Content of a message which can be not only text, but also other media type.

mod private
{
  use crate::*;
  use core::fmt;

  pub trait Data
  where
    Self : AsBytes + fmt::Display,
  {
  }

  impl< T > Data for T
  where
    Self : AsBytes + fmt::Display,
  {
  }

  pub trait Content
  {
    fn content_type( &self ) -> ContentType;
    fn content_to_string( self ) -> String;
    fn content_to_bytes( self ) -> Vec< u8 >;
  }

  #[ derive( Debug ) ]
  pub enum ContentType
  {
    Text,
    Image,
    Sound,
    Video,
    Unwknown,
  }

  impl Content for String
  {
    fn content_type( &self ) -> ContentType
    {
      ContentType::Text
    }
    fn content_to_string( self ) -> String
    {
      self
    }
    fn content_to_bytes( self ) -> Vec< u8 >
    {
      self.to_bytes_vec()
    }
  }

  /// Image
  #[ derive( Debug ) ]
  pub struct Image< S >
  where
    S : Data,
  {
    pub source : Source< S >,
  }

  /// Source
  #[ derive( Debug ) ]
  pub struct Source< S : AsBytes >
  where
    S : Data,
  {
    pub media_type : String,
    pub encoding : String,
    pub data : S,
  }

  impl< S > Content for Image< S >
  where
    S : Data,
  {
    fn content_type( &self ) -> ContentType
    {
      ContentType::Image
    }
    fn content_to_string( self ) -> String
    {
      format!( "{}", self.source.data )
    }
    fn content_to_bytes( self ) -> Vec< u8 >
    {
      self.source.data.to_bytes_vec()
    }
  }

}

crate::mod_interface!
{
  orphan use private::
  {
    Data,
    Content,
    ContentType,
    Image,
  };
}
