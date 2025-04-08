
//! Content of a message which can be not only text, but also other media type.

mod private
{

  use core::
  {
    fmt,
    convert::TryFrom,
    iter::IntoIterator,
  };
  use serde_json;
  use derive_tools::From;

  /// Represents errors during content conversion, particularly from JSON.
  #[ derive( Debug, Clone, PartialEq, Eq ) ]
  pub enum Error
  {
    /// Indicates that a JSON type cannot be represented as the target ContentAny variant.
    UnsupportedType( String ),
    /// Error during conversion from a `serde_json::Value`.
    JsonConversionError( String ),
  }

  impl fmt::Display for Error
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      match self
      {
        Error::UnsupportedType( t ) =>
          write!( f, "Cannot convert JSON type '{}' to ContentAny", t ),
        Error::JsonConversionError( e ) =>
          write!( f, "Error converting JSON to ContentAny: {}", e ),
      }
    }
  }

  impl std::error::Error for Error {}

  /// Marker trait for data types usable within media content like `Source`.
  /// Requires common traits for data handling and representation.
  pub trait Data
  where
    // Use crate::IntoBytes explicitly if not brought into scope otherwise
    Self : crate::IntoBytes + fmt::Debug + Default + Clone + PartialEq,
  {
  }

  impl< T > Data for T
  where
    // Use crate::IntoBytes explicitly
    Self : crate::IntoBytes + fmt::Debug + Default + Clone + PartialEq,
  {
  }

  /// Classifies the high-level type of content.
  #[ derive( Debug, PartialEq, Eq, Clone ) ]
  pub enum ContentType
  {
    /// Represents a null or empty value.
    Null,
    /// Represents a boolean value.
    Bool,
    /// Represents a numeric value.
    Number,
    /// Represents a string value.
    String,
    /// Represents image content.
    Image,
    /// Represents sound content.
    Sound,
    /// Represents video content.
    Video,
    /// Represents PDF content.
    Pdf, // Added Pdf
    /// Represents a generic file content.
    File, // Added File
    /// Represents an array of content.
    Array,
    /// Represents an unknown or unspecified content type.
    Unwknown, // Typo kept as is from original code
  }

  /// Holds media data along with its type and encoding information. Generic over the data storage type `S`.
  #[ derive( Debug, Clone, PartialEq ) ]
  pub struct Source< S >
  where
    S : Data,
  {
    /// The specific media type (e.g., "image/png", "audio/mpeg").
    pub media_type : String,
    /// The encoding of the data (e.g., "base64").
    pub encoding : String,
    /// The actual media data.
    pub data : S,
  }

  impl< S > Default for Source< S >
  where
    S : Data,
  {
    fn default() -> Self
    {
      Self
      {
        media_type : String::default(),
        encoding : String::default(),
        data : S::default(),
      }
    }
  }

  /// Represents image content, wrapping a `Source`.
  #[ derive( Debug, Clone, PartialEq, Default ) ]
  pub struct Image< S >
  where
    S : Data,
  {
    /// The underlying source data for the image.
    pub source : Source< S >,
  }

  /// Represents sound content, wrapping a `Source`.
  #[ derive( Debug, Clone, PartialEq, Default ) ]
  pub struct Sound< S >
  where
    S : Data,
  {
    /// The underlying source data for the sound.
    pub source : Source< S >,
  }


  /// Represents video content, wrapping a `Source`.
  #[ derive( Debug, Clone, PartialEq, Default ) ]
  pub struct Video< S >
  where
    S : Data,
  {
    /// The underlying source data for the video.
    pub source : Source< S >,
  }

  /// Represents PDF content, wrapping a `Source`.
  #[ derive( Debug, Clone, PartialEq, Default ) ] // Added Pdf struct
  pub struct Pdf< S >
  where
    S : Data,
  {
    /// The underlying source data for the PDF.
    pub source : Source< S >,
  }

  /// Represents generic file content, wrapping a `Source`.
  #[ derive( Debug, Clone, PartialEq, Default ) ] // Added File struct
  pub struct File< S >
  where
    S : Data,
  {
    /// The underlying source data for the file.
    pub source : Source< S >,
  }

  /// Represents any possible content type, generic over the data storage type `S` for media.
  #[ derive( Debug, Clone, PartialEq, From ) ]
  pub enum ContentAny< S >
  where
    S : Data,
  {
    /// Represents a null or empty value.
    Null,
    /// Represents a boolean value.
    Bool( bool ),
    /// Represents a string value.
    String( String ),
    /// Represents an array of other `ContentAny` values.
    Array( Vec< ContentAny< S > > ),
    /// Represents a numeric value using `serde_json::Number`.
    Number( serde_json::Number ),
    /// Represents image content.
    Image( Image< S > ),
    /// Represents sound content.
    Sound( Sound< S > ),
    /// Represents video content.
    Video( Video< S > ),
    /// Represents PDF content.
    Pdf( Pdf< S > ), // Added Pdf variant
    /// Represents generic file content.
    File( File< S > ), // Added File variant
  }

  /// Trait for types that can be treated as or converted into `ContentAny<S>`.
  /// Provides methods for introspection and conversion. Generic over the target data type `S`.
  pub trait ContentLike< S : Data >
  {
    /// Returns the high-level classification of the content.
    fn content_type( &self ) -> ContentType;

    /// Consumes the content and returns its raw byte representation.
    fn content_to_bytes( self ) -> Vec< u8 >
    where
      Self : Sized;

    /// Consumes the content and converts it into a `serde_json::Value`. Media data might be omitted.
    fn content_to_json( self ) -> serde_json::Value
    where
      Self : Sized;

    /// Consumes the implementing type and converts it into a `ContentAny<S>` variant.
    fn into_any( self ) -> ContentAny< S >
    where
      Self : Sized;

    /// Consumes self, converts to `ContentAny<S>`, and pushes to `target_content` if it's an Array. Panics otherwise.
    #[ inline ]
    fn push_to( self, target_content : ContentAny< S > ) -> ContentAny< S >
    where
      Self : Sized,
      S : Data + 'static, // Added constraint needed for push
    {
      let element_content = self.into_any();
      // Call the push method defined in the builder module's impl block
      ContentAny::push( target_content, element_content )
    }
  }

  impl< S > ContentLike< S > for ContentAny< S >
  where
    S : Data + 'static,
  {
    fn content_type( &self ) -> ContentType
    {
      match self
      {
        ContentAny::Null => < () as ContentLike< S > >::content_type( &() ),
        ContentAny::Bool( b ) => < bool as ContentLike< S > >::content_type( b ),
        ContentAny::Number( n ) => < serde_json::Number as ContentLike< S > >::content_type( n ),
        ContentAny::String( s ) => < String as ContentLike< S > >::content_type( s ),
        ContentAny::Array( arr ) => < Vec< ContentAny< S > > as ContentLike< S > >::content_type( arr ),
        ContentAny::Image( it ) => < Image< S > as ContentLike< S > >::content_type( it ),
        ContentAny::Sound( it ) => < Sound< S > as ContentLike< S > >::content_type( it ),
        ContentAny::Video( it ) => < Video< S > as ContentLike< S > >::content_type( it ),
        ContentAny::Pdf( it ) => < Pdf< S > as ContentLike< S > >::content_type( it ), // Added Pdf arm
        ContentAny::File( it ) => < File< S > as ContentLike< S > >::content_type( it ), // Added File arm
      }
    }

    fn content_to_bytes( self ) -> Vec< u8 >
    {
      match self
      {
        ContentAny::Null => < () as ContentLike< S > >::content_to_bytes( () ),
        ContentAny::Bool( b ) => < bool as ContentLike< S > >::content_to_bytes( b ),
        ContentAny::Number( n ) => < serde_json::Number as ContentLike< S > >::content_to_bytes( n ),
        ContentAny::String( s ) => < String as ContentLike< S > >::content_to_bytes( s ),
        ContentAny::Array( list ) => < Vec< ContentAny< S > > as ContentLike< S > >::content_to_bytes( list ),
        ContentAny::Image( image ) => < Image< S > as ContentLike< S > >::content_to_bytes( image ),
        ContentAny::Sound( sound ) => < Sound< S > as ContentLike< S > >::content_to_bytes( sound ),
        ContentAny::Video( video ) => < Video< S > as ContentLike< S > >::content_to_bytes( video ),
        ContentAny::Pdf( pdf ) => < Pdf< S > as ContentLike< S > >::content_to_bytes( pdf ), // Added Pdf arm
        ContentAny::File( file ) => < File< S > as ContentLike< S > >::content_to_bytes( file ), // Added File arm
      }
    }

    fn content_to_json( self ) -> serde_json::Value
    {
      serde_json::Value::from( self )
    }

    #[ inline ]
    fn into_any( self ) -> ContentAny< S >
    where
      Self : Sized
    {
       self
    }
  }

  impl< T, S > ContentLike< S > for Image< T >
  where
    T : Data + Into< S > + 'static,
    S : Data + 'static,
  {
    fn content_type( &self ) -> ContentType
    {
      ContentType::Image
    }

    fn content_to_bytes( self ) -> Vec< u8 >
    where
      Self : Sized
    {
      // Use crate::IntoBytes explicitly
      self.source.data.into_bytes()
    }

    fn content_to_json( self ) -> serde_json::Value
    where
      Self : Sized
    {
      let content_any_s : ContentAny< S > = self.into_any();
      content_any_s.content_to_json()
    }

    /// Converts Image<T> into ContentAny<S>::Image, converting data from T to S.
    #[ inline ]
    fn into_any( self ) -> ContentAny< S >
    where
      Self : Sized,
    {
      let source_s = Source::< S >
      {
        media_type : self.source.media_type,
        encoding : self.source.encoding,
        data : self.source.data.into(),
      };
      // Call the `new` associated function from the builder module
      Image::new( source_s ).into()
    }
  }

  impl< T, S > ContentLike< S > for Sound< T >
  where
    T : Data + Into< S > + 'static,
    S : Data + 'static,
  {
    fn content_type( &self ) -> ContentType
    {
      ContentType::Sound
    }

    fn content_to_bytes( self ) -> Vec< u8 >
    where
      Self : Sized
    {
      // Use crate::IntoBytes explicitly
      self.source.data.into_bytes()
    }

    fn content_to_json( self ) -> serde_json::Value
    where
      Self : Sized
    {
      let content_any_s : ContentAny< S > = self.into_any();
      content_any_s.content_to_json()
    }

    /// Converts Sound<T> into ContentAny<S>::Sound, converting data from T to S.
    #[ inline ]
    fn into_any( self ) -> ContentAny< S >
    where
      Self : Sized,
    {
      let source_s = Source::< S >
      {
        media_type : self.source.media_type,
        encoding : self.source.encoding,
        data : self.source.data.into(),
      };
      // Call the `new` associated function from the builder module
      Sound::new( source_s ).into()
    }
  }

  impl< T, S > ContentLike< S > for Video< T >
  where
    T : Data + Into< S > + 'static,
    S : Data + 'static,
  {
    fn content_type( &self ) -> ContentType
    {
      ContentType::Video
    }

    fn content_to_bytes( self ) -> Vec< u8 >
    where
      Self : Sized
    {
      // Use crate::IntoBytes explicitly
      self.source.data.into_bytes()
    }

    fn content_to_json( self ) -> serde_json::Value
    where
      Self : Sized
    {
      let content_any_s : ContentAny< S > = self.into_any();
      content_any_s.content_to_json()
    }

    /// Converts Video<T> into ContentAny<S>::Video, converting data from T to S.
    #[ inline ]
    fn into_any( self ) -> ContentAny< S >
    where
      Self : Sized,
    {
      let source_s = Source::< S >
      {
        media_type : self.source.media_type,
        encoding : self.source.encoding,
        data : self.source.data.into(),
      };
      // Call the `new` associated function from the builder module
      Video::new( source_s ).into()
    }
  }

  // Added ContentLike impl for Pdf
  impl< T, S > ContentLike< S > for Pdf< T >
  where
    T : Data + Into< S > + 'static,
    S : Data + 'static,
  {
    fn content_type( &self ) -> ContentType
    {
      ContentType::Pdf
    }

    fn content_to_bytes( self ) -> Vec< u8 >
    where
      Self : Sized
    {
      // Use crate::IntoBytes explicitly
      self.source.data.into_bytes()
    }

    fn content_to_json( self ) -> serde_json::Value
    where
      Self : Sized
    {
      let content_any_s : ContentAny< S > = self.into_any();
      content_any_s.content_to_json()
    }

    /// Converts Pdf<T> into ContentAny<S>::Pdf, converting data from T to S.
    #[ inline ]
    fn into_any( self ) -> ContentAny< S >
    where
      Self : Sized,
    {
      let source_s = Source::< S >
      {
        media_type : self.source.media_type,
        encoding : self.source.encoding,
        data : self.source.data.into(),
      };
      // Call the `new` associated function from the builder module
      Pdf::new( source_s ).into()
    }
  }

  // Added ContentLike impl for File
  impl< T, S > ContentLike< S > for File< T >
  where
    T : Data + Into< S > + 'static,
    S : Data + 'static,
  {
    fn content_type( &self ) -> ContentType
    {
      ContentType::File
    }

    fn content_to_bytes( self ) -> Vec< u8 >
    where
      Self : Sized
    {
      // Use crate::IntoBytes explicitly
      self.source.data.into_bytes()
    }

    fn content_to_json( self ) -> serde_json::Value
    where
      Self : Sized
    {
      let content_any_s : ContentAny< S > = self.into_any();
      content_any_s.content_to_json()
    }

    /// Converts File<T> into ContentAny<S>::File, converting data from T to S.
    #[ inline ]
    fn into_any( self ) -> ContentAny< S >
    where
      Self : Sized,
    {
      let source_s = Source::< S >
      {
        media_type : self.source.media_type,
        encoding : self.source.encoding,
        data : self.source.data.into(),
      };
      // Call the `new` associated function from the builder module
      File::new( source_s ).into()
    }
  }

  impl< S > ContentLike< S > for Vec< ContentAny< S > >
  where
    S : Data + 'static,
  {
    fn content_type( &self ) -> ContentType
    {
      ContentType::Array
    }

    fn content_to_bytes( self ) -> Vec< u8 >
    where
      Self : Sized
    {
      self.into_iter()
      .flat_map( | c | c.content_to_bytes() )
      .collect()
    }

    fn content_to_json( self ) -> serde_json::Value
    where
      Self : Sized
    {
      serde_json::Value::Array
      (
        self.into_iter()
        .map( | c | c.content_to_json() )
        .collect()
      )
    }

    /// Converts Vec<ContentAny<S>> into ContentAny<S>::Array.
    #[ inline ]
    fn into_any( self ) -> ContentAny< S >
    where
      Self : Sized
    {
      ContentAny::Array( self )
    }
  }

  impl< S > ContentLike< S > for String
  where
    S : Data + 'static,
  {
    fn content_type( &self ) -> ContentType
    {
      ContentType::String
    }

    fn content_to_bytes( self ) -> Vec< u8 >
    where
      Self : Sized
    {
      self.into_bytes()
    }

    fn content_to_json( self ) -> serde_json::Value
    where
      Self : Sized
    {
      serde_json::Value::String( self )
    }

    /// Converts String into ContentAny<S>::String.
    #[ inline ]
    fn into_any( self ) -> ContentAny< S >
    where
      Self : Sized
    {
      // Call the `string` associated function from the builder module
      ContentAny::string( self )
    }
  }

  impl< S > ContentLike< S > for bool
  where
    S : Data + 'static,
  {
    fn content_type( &self ) -> ContentType
    {
      ContentType::Bool
    }

    fn content_to_bytes( self ) -> Vec< u8 >
    where
      Self : Sized
    {
      serde_json::to_vec( &serde_json::Value::Bool( self ) ).unwrap_or_default()
    }

    fn content_to_json( self ) -> serde_json::Value
    where
      Self : Sized
    {
      serde_json::Value::Bool( self )
    }

    /// Converts bool into ContentAny<S>::Bool.
    #[ inline ]
    fn into_any( self ) -> ContentAny< S >
    where
      Self : Sized
    {
      // Call the `bool` associated function from the builder module
      ContentAny::bool( self )
    }
  }

  impl< S > ContentLike< S > for serde_json::Number
  where
    S : Data + 'static,
  {
    fn content_type( &self ) -> ContentType
    {
      ContentType::Number
    }

    fn content_to_bytes( self ) -> Vec< u8 >
    where
      Self : Sized
    {
      let value = serde_json::Value::Number( self );
      serde_json::to_vec( &value ).unwrap_or_default()
    }

    fn content_to_json( self ) -> serde_json::Value
    where
      Self : Sized
    {
      serde_json::Value::Number( self )
    }

    /// Converts serde_json::Number into ContentAny<S>::Number.
    #[ inline ]
    fn into_any( self ) -> ContentAny< S >
    where
      Self : Sized
    {
      // Call the `number` associated function from the builder module
      ContentAny::number( self )
    }
  }

  impl< S > ContentLike< S > for ()
  where
    S : Data + 'static,
  {
    fn content_type( &self ) -> ContentType
    {
      ContentType::Null
    }

    fn content_to_bytes( self ) -> Vec< u8 >
    where
      Self : Sized
    {
      serde_json::to_vec( &serde_json::Value::Null ).unwrap_or_default()
    }

    fn content_to_json( self ) -> serde_json::Value
    where
      Self : Sized
    {
      serde_json::Value::Null
    }

    /// Converts () into ContentAny<S>::Null.
    #[ inline ]
    fn into_any( self ) -> ContentAny< S >
    where
      Self : Sized
    {
      // Call the `null` associated function from the builder module
      ContentAny::null()
    }
  }

  /// Converts `ContentAny<S>` to a `serde_json::Value`, omitting media data.
  impl< S > From< ContentAny< S > > for serde_json::Value
  where
    S : Data + 'static,
  {
    fn from( value : ContentAny< S > ) -> Self
    {
      match value
      {
        ContentAny::Null => serde_json::Value::Null,
        ContentAny::Bool( b ) => serde_json::Value::Bool( b ),
        ContentAny::Number( n ) => serde_json::Value::Number( n ),
        ContentAny::String( s ) => serde_json::Value::String( s ),
        ContentAny::Image( image ) => serde_json::json!
        ({
          "type" : "image",
          "media_type" : image.source.media_type,
          "encoding" : image.source.encoding,
        }),
        ContentAny::Sound( sound ) => serde_json::json!
        ({
          "type" : "sound",
          "media_type" : sound.source.media_type,
          "encoding" : sound.source.encoding,
        }),
        ContentAny::Video( video ) => serde_json::json!
        ({
          "type" : "video",
          "media_type" : video.source.media_type,
          "encoding" : video.source.encoding,
        }),
        ContentAny::Pdf( pdf ) => serde_json::json! // Added Pdf arm
        ({
          "type" : "pdf",
          "media_type" : pdf.source.media_type,
          "encoding" : pdf.source.encoding,
        }),
        ContentAny::File( file ) => serde_json::json! // Added File arm
        ({
          "type" : "file",
          "media_type" : file.source.media_type,
          "encoding" : file.source.encoding,
        }),
        ContentAny::Array( list ) =>
        {
          let json_list = list.into_iter().map( serde_json::Value::from ).collect();
          serde_json::Value::Array( json_list )
        }
      }
    }
  }

  /// Attempts to convert a `serde_json::Value` into `ContentAny<Vec<u8>>`.
  /// Fails for JSON objects as media data reconstruction is not supported.
  impl TryFrom< serde_json::Value > for ContentAny< Vec< u8 > >
  {
    type Error = Error;

    fn try_from( value : serde_json::Value ) -> Result< Self, Self::Error >
    {
      match value
      {
        serde_json::Value::Null => Ok( ContentAny::null() ),
        serde_json::Value::Bool( b ) => Ok( ContentAny::bool( b ) ),
        serde_json::Value::Number( n ) => Ok( ContentAny::number( n ) ),
        serde_json::Value::String( s ) => Ok( ContentAny::string( s ) ),
        serde_json::Value::Array( arr ) =>
        {
          let content_array = arr
          .into_iter()
          .map( ContentAny::< Vec< u8 > >::try_from )
          .collect::< Result< Vec< ContentAny< Vec< u8 > > >, Self::Error > >()?;
          Ok( ContentAny::Array( content_array ) ) // Array construction doesn't use builder method directly here
        },
        serde_json::Value::Object( obj ) =>
        {
          let _ = obj; // Avoid unused variable warning
          // Media types (Image, Sound, Video, Pdf, File) are represented as objects in JSON,
          // but converting *from* JSON object back to a specific media type with data is not supported here.
          Err( Error::UnsupportedType( "Object (Media types require specific structure for JSON deserialization)".to_string() ) )
        },
      }
    }
  }

  /// Attempts to convert a `serde_json::Value` into `ContentAny<String>`.
  /// Fails for JSON objects as media data reconstruction is not supported.
  impl TryFrom< serde_json::Value > for ContentAny< String >
  {
    type Error = Error;

    fn try_from( value : serde_json::Value ) -> Result< Self, Self::Error >
    {
      match value
      {
        serde_json::Value::Null => Ok( ContentAny::null() ),
        serde_json::Value::Bool( b ) => Ok( ContentAny::bool( b ) ),
        serde_json::Value::Number( n ) => Ok( ContentAny::number( n ) ),
        serde_json::Value::String( s ) => Ok( ContentAny::string( s ) ),
        serde_json::Value::Array( arr ) =>
        {
          let content_array = arr
          .into_iter()
          .map( ContentAny::< String >::try_from )
          .collect::< Result< Vec< ContentAny< String > >, Self::Error > >()?;
          Ok( ContentAny::Array( content_array ) ) // Array construction doesn't use builder method directly here
        },
        serde_json::Value::Object( obj ) =>
        {
          let _ = obj; // Avoid unused variable warning
          // Media types (Image, Sound, Video, Pdf, File) are represented as objects in JSON,
          // but converting *from* JSON object back to a specific media type with data is not supported here.
          Err( Error::UnsupportedType( "Object (Media types cannot be reliably represented as String from JSON)".to_string() ) )
        },
      }
    }
  }

} // mod private

mod builder;

crate::mod_interface!
{
  // Expose original types
  own use private::
  {
    Error,
    Data,
    ContentLike,
    ContentType,
    Source,
    Image,
    Sound,
    Video,
    Pdf,
    File,
    ContentAny,
  };

  // Expose the builder module itself. Users will need to use paths like
  // `content::builder::source()` or `content::builder::string()`
  // or import items from `content::builder`.
  // The builder methods on the types themselves (like Source::media_type)
  // are already available because the types are exposed.
  reuse builder;

}