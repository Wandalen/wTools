
//! Module containing builder pattern implementations and constructors.

mod private
{

  // Import necessary items from the parent module
  // Use `super::*` to bring parent items into scope easily
  use crate::*;

  use content::
  {
    Data,
    Source,
    Image,
    Sound,
    Video,
    ContentAny,
  };

  use core::iter::IntoIterator;
  use serde_json;

  // // Define empty structs to hang the impl blocks on, mirroring the parent types
  // // These are only used internally to structure the code
  // pub struct Source;
  // pub struct Image;
  // pub struct Sound;
  // pub struct Video;
  // pub struct ContentAny;

  impl< S > Source< S >
  where
    S : Data,
  {
    /// Creates a new Source with default values.
    pub fn new() -> Self
    {
      Default::default()
    }

    /// Sets the media_type field.
    #[ inline ]
    pub fn media_type( mut self, media_type : String ) -> Self
    {
      self.media_type = media_type;
      self
    }

    /// Sets the encoding field.
    #[ inline ]
    pub fn encoding( mut self, encoding : String ) -> Self
    {
      self.encoding = encoding;
      self
    }

    /// Sets the data field.
    #[ inline ]
    pub fn data( mut self, data : S ) -> Self
    {
      self.data = data;
      self
    }

    /// Consumes the Source and creates an Image containing it.
    #[ inline ]
    pub fn image( self ) -> Image< S >
    {
      // Call the source method defined below in Image's impl block
      Image::default().source( self )
    }

    /// Consumes the Source and creates a Sound containing it.
    #[ inline ]
    pub fn sound( self ) -> Sound< S >
    {
      // Call the source method defined below in Sound's impl block
      Sound::default().source( self )
    }

    /// Consumes the Source and creates a Video containing it.
    #[ inline ]
    pub fn video( self ) -> Video< S >
    {
      // Call the source method defined below in Video's impl block
      Video::default().source( self )
    }
  }

  impl< S > Image< S >
  where
    S : Data,
  {
    /// Creates a new Image wrapping the given source.
    pub fn new( source : Source< S > ) -> Self
    {
      Self
      {
        source
      }
    }

    /// Sets the source field.
    #[ inline ]
    pub fn source( mut self, source : Source< S > ) -> Self
    {
      self.source = source;
      self
    }
  }

  impl< S > Sound< S >
  where
    S : Data,
  {
    /// Creates a new Sound wrapping the given source.
    pub fn new( source : Source< S > ) -> Self
    {
      Self
      {
        source
      }
    }

    /// Sets the source field.
    #[ inline ]
    pub fn source( mut self, source : Source< S > ) -> Self
    {
      self.source = source;
      self
    }
  }

  impl< S > Video< S >
  where
    S : Data,
  {
    /// Creates a new Video wrapping the given source.
    pub fn new( source : Source< S > ) -> Self
    {
      Self
      {
        source
      }
    }

    /// Sets the source field.
    #[ inline ]
    pub fn source( mut self, source : Source< S > ) -> Self
    {
      self.source = source;
      self
    }
  }

  // Use the empty struct to hang the associated functions and methods
  impl< S > ContentAny< S >
  where
    S : Data,
  {
    /// Creates a new ContentAny::Null variant.
    #[ inline ]
    pub fn null() -> ContentAny< S >
    {
      ContentAny::Null
    }

    /// Creates a new ContentAny::Bool variant.
    #[ inline ]
    pub fn bool( val : bool ) -> ContentAny< S >
    {
      ContentAny::Bool( val )
    }

    /// Creates a new ContentAny::String variant.
    #[ inline ]
    pub fn string( val : String ) -> ContentAny< S >
    {
      ContentAny::String( val )
    }

    /// Creates a new empty ContentAny::Array variant.
    #[ inline ]
    pub fn array() -> ContentAny< S >
    {
      ContentAny::Array( Vec::new() )
    }

    /// Creates a new ContentAny::Number variant.
    #[ inline ]
    pub fn number( val : serde_json::Number ) -> ContentAny< S >
    {
      ContentAny::Number( val )
    }

    /// Creates a new ContentAny::Image variant from a Source<S> struct.
    #[ inline ]
    pub fn image( source : Source< S > ) -> ContentAny< S >
    {
      ContentAny::Image( Image::new( source ) )
    }

    /// Creates a new ContentAny::Sound variant from a Source<S> struct.
    #[ inline ]
    pub fn sound( source : Source< S > ) -> ContentAny< S >
    {
      ContentAny::Sound( Sound::new( source ) )
    }

    /// Creates a new ContentAny::Video variant from a Source<S> struct.
    #[ inline ]
    pub fn video( source : Source< S > ) -> ContentAny< S >
    {
      ContentAny::Video( Video::new( source ) )
    }

    /// Pushes an element to the content if it is an Array variant. Panics otherwise.
    // This is now an associated function taking `content` as the first argument
    #[ inline ]
    pub fn push( mut content : ContentAny< S >, element : ContentAny< S > ) -> ContentAny< S >
    {
      match &mut content
      {
        ContentAny::Array( arr ) =>
          arr.push( element ),
        _ =>
          panic!( "Cannot push to ContentAny that is not an Array variant" ),
      }
      content
    }

    /// Extends the content with elements if it is an Array variant. Panics otherwise.
    // This is now an associated function taking `content` as the first argument
    #[ inline ]
    pub fn extend( mut content : ContentAny< S >, elements : impl IntoIterator< Item = ContentAny< S > > ) -> ContentAny< S >
    {
      match &mut content
      {
        ContentAny::Array( arr ) =>
          arr.extend( elements ),
        _ =>
          panic!( "Cannot extend ContentAny that is not an Array variant" ),
      }
      content
    }
  }

} // mod private

crate::mod_interface!
{
  // own use private::
  // {
  //   // SourceBuilder,
  //   // ImageBuilder,
  //   // SoundBuilder,
  //   // VideoBuilder,
  //   // ContentAnyBuilder,
  // };
}
