//! Content of a message which can be not only text, but also other media type.

mod private
{
  use crate::*;
  use core::fmt;
  use serde::Serialize;
  use serde_json;

  pub trait Data
  where
    Self : AsBytes + fmt::Display + Default,
  {
  }

  impl< T > Data for T
  where
    Self : AsBytes + fmt::Display + Default,
  {
  }

  pub trait Content
  {
    fn content_type( &self ) -> ContentType;
    fn content_to_bytes( self ) -> Vec< u8 >;
    fn content_to_json( self ) -> serde_json::Value where Self : Sized;
  }

  #[ derive( Debug, PartialEq, Eq ) ]
  pub enum ContentType
  {
    Text,
    Image,
    Sound,
    Video,
    List,
    Json,
    Unwknown,
  }

  impl Content for String
  {
    fn content_type( &self ) -> ContentType
    {
      ContentType::Text
    }
    fn content_to_bytes( self ) -> Vec< u8 >
    {
      self.into_bytes()
    }
    fn content_to_json( self ) -> serde_json::Value
    {
      serde_json::Value::String( self )
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
    fn content_to_bytes( self ) -> Vec< u8 >
    {
      self.source.data.to_bytes_vec()
    }
    fn content_to_json( self ) -> serde_json::Value
    {
      // Represent image as a JSON object indicating its type and media type.
      // Data itself is not included as it's binary.
      serde_json::json!
      ({
        "type": "image",
        "media_type": self.source.media_type,
        "encoding": self.source.encoding,
        // Data is omitted or represented as a placeholder if needed
        // "data_placeholder": format!( "<{} bytes of image data>", self.source.data.as_bytes().len() )
      })
    }
  }

  impl< C > Content for Vec< C >
  where
    C : Content,
  {
    fn content_type( &self ) -> ContentType
    {
      // If the list is not empty, return List type. Otherwise, Unknown.
      if !self.is_empty()
      {
        ContentType::List
      }
      else
      {
        ContentType::Unwknown
      }
    }

    fn content_to_bytes( self ) -> Vec< u8 >
    {
      self.into_iter().flat_map( | c | c.content_to_bytes() ).collect()
    }
    fn content_to_json( self ) -> serde_json::Value
    {
      serde_json::Value::Array( self.into_iter().map( | c | c.content_to_json() ).collect() )
    }
  }

  /// Wrapper for types that implement Serialize to treat them as Content.
  #[ derive( Debug ) ]
  pub struct JsonContent< T : Serialize >( pub T );

  impl< T > Content for JsonContent< T >
  where
    T : Serialize,
  {
    fn content_type( &self ) -> ContentType
    {
      ContentType::Json
    }

    fn content_to_bytes( self ) -> Vec< u8 >
    {
      serde_json::to_vec( &self.0 ).unwrap_or_else( | e | format!( "{{\"error\":\"Serialization failed: {}\"}}", e ).into_bytes() )
    }
    fn content_to_json( self ) -> serde_json::Value
    {
      serde_json::to_value( self.0 ).unwrap_or_else( | e | serde_json::json!( { "error": format!( "Serialization failed: {}", e ) } ) )
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
    JsonContent,
  };
}