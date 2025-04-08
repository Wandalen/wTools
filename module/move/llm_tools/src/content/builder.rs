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
    Pdf,
    File,
    ContentAny,
  };

  use core::iter::IntoIterator;
  use serde_json;

  // --- Constructor Functions ---

  /// Creates a new Source with default values. Alias for `Source::new()`.
  #[ inline ]
  pub fn source< S : Data >() -> Source< S >
  {
    Source::< S >::new()
  }

  /// Creates a new Image wrapping the given source. Alias for `Image::new()`.
  #[ inline ]
  pub fn image< S : Data >( source : Source< S > ) -> Image< S >
  {
    Image::< S >::new( source )
  }

  /// Creates a new Sound wrapping the given source. Alias for `Sound::new()`.
  #[ inline ]
  pub fn sound< S : Data >( source : Source< S > ) -> Sound< S >
  {
    Sound::< S >::new( source )
  }

  /// Creates a new Video wrapping the given source. Alias for `Video::new()`.
  #[ inline ]
  pub fn video< S : Data >( source : Source< S > ) -> Video< S >
  {
    Video::< S >::new( source )
  }

  /// Creates a new Pdf wrapping the given source. Alias for `Pdf::new()`.
  #[ inline ]
  pub fn pdf< S : Data >( source : Source< S > ) -> Pdf< S >
  {
    Pdf::< S >::new( source )
  }

  /// Creates a new File wrapping the given source. Alias for `File::new()`.
  #[ inline ]
  pub fn file< S : Data >( source : Source< S > ) -> File< S >
  {
    File::< S >::new( source )
  }

  /// Creates a new ContentAny::Null variant. Alias for `ContentAny::null()`.
  #[ inline ]
  pub fn null< S : Data >() -> ContentAny< S >
  {
    ContentAny::< S >::null()
  }

  /// Creates a new ContentAny::Bool variant. Alias for `ContentAny::bool()`.
  #[ inline ]
  pub fn bool< S : Data >( val : bool ) -> ContentAny< S >
  {
    ContentAny::< S >::bool( val )
  }

  /// Creates a new ContentAny::String variant. Alias for `ContentAny::string()`.
  #[ inline ]
  pub fn string< S : Data >( val : String ) -> ContentAny< S >
  {
    ContentAny::< S >::string( val )
  }

  /// Creates a new empty ContentAny::Array variant. Alias for `ContentAny::array()`.
  #[ inline ]
  pub fn array< S : Data >() -> ContentAny< S >
  {
    ContentAny::< S >::array()
  }

  /// Creates a new ContentAny::Number variant. Alias for `ContentAny::number()`.
  #[ inline ]
  pub fn number< S : Data >( val : serde_json::Number ) -> ContentAny< S >
  {
    ContentAny::< S >::number( val )
  }

  // --- End Constructor Functions ---


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

    /// Consumes the Source and creates a Pdf containing it.
    #[ inline ]
    pub fn pdf( self ) -> Pdf< S > // Added pdf method
    {
      // Call the source method defined below in Pdf's impl block
      Pdf::default().source( self )
    }

    /// Consumes the Source and creates a File containing it.
    #[ inline ]
    pub fn file( self ) -> File< S > // Added file method
    {
      // Call the source method defined below in File's impl block
      File::default().source( self )
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

  // Added Pdf impl block
  impl< S > Pdf< S >
  where
    S : Data,
  {
    /// Creates a new Pdf wrapping the given source.
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

  // Added File impl block
  impl< S > File< S >
  where
    S : Data,
  {
    /// Creates a new File wrapping the given source.
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

    /// Creates a new ContentAny::Pdf variant from a Source<S> struct.
    #[ inline ]
    pub fn pdf( source : Source< S > ) -> ContentAny< S > // Added pdf constructor
    {
      ContentAny::Pdf( Pdf::new( source ) )
    }

    /// Creates a new ContentAny::File variant from a Source<S> struct.
    #[ inline ]
    pub fn file( source : Source< S > ) -> ContentAny< S > // Added file constructor
    {
      ContentAny::File( File::new( source ) )
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
  // Expose constructor functions moved from mod.rs
  orphan use private::
  {
    source,
    image,
    sound,
    video,
    pdf,
    file,
    null,
    bool,
    string,
    array,
    number,
  };
}