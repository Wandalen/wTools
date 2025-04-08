use super::*;
use core::fmt;
use std::convert::
{
  TryFrom,
  TryInto
};

use the_module::content::
{
  self, // Import the module itself for builder functions
  ContentAny,
  ContentLike,
  ContentType,
  File,
  Image,
  Pdf,
  Sound,
  Source,
  Video,
};

// Helper function to create a default Source<Vec<u8>> for media types
fn default_media_source() -> Source< Vec< u8 > >
{
  Source::< Vec< u8 > >
  {
    media_type : "application/octet-stream".to_string(),
    encoding : "binary".to_string(),
    data : vec![ 1, 2, 3 ],
  }
}

// Helper function to create a default Image<Vec<u8>>
fn default_image() -> Image< Vec< u8 > >
{
  Image
  {
    source : default_media_source()
  }
}

// Helper function to create a default Sound<Vec<u8>>
fn default_sound() -> Sound< Vec< u8 > >
{
  Sound
  {
    source : default_media_source()
  }
}

// Helper function to create a default Video<Vec<u8>>
fn default_video() -> Video< Vec< u8 > >
{
  Video
  {
    source : default_media_source()
  }
}

// Helper function to create a default Pdf<Vec<u8>>
fn default_pdf() -> Pdf< Vec< u8 > >
{
  Pdf
  {
    source : default_media_source()
  }
}

// Helper function to create a default File<Vec<u8>>
fn default_file() -> File< Vec< u8 > >
{
  File
  {
    source : default_media_source()
  }
}


/// Tests construction of each ContentAny variant using builder functions.
#[ test ]
fn construct_variants()
{
  // Local use for builder functions already imported at top level
  // Local use for types already imported at top level
  use serde_json; // Local use

  // Null
  let null_content = content::null::< Vec< u8 > >();
  assert_eq!( null_content, ContentAny::Null );

  // Bool
  let bool_content_true = content::bool::< Vec< u8 > >( true );
  let bool_content_false = content::bool::< Vec< u8 > >( false );
  assert_eq!( bool_content_true, ContentAny::Bool( true ) );
  assert_eq!( bool_content_false, ContentAny::Bool( false ) );

  // String
  let string_content = content::string::< Vec< u8 > >( "test string".to_string() );
  assert_eq!( string_content, ContentAny::String( "test string".to_string() ) );

  // Array
  let array_content = content::array::< Vec< u8 > >();
  assert_eq!( array_content, ContentAny::Array( vec![] ) );

  // Number
  let number = serde_json::Number::from( 123 );
  let number_content = content::number::< Vec< u8 > >( number.clone() );
  assert_eq!( number_content, ContentAny::Number( number ) );

  // Image (using the struct's ::new method via builder)
  let source_img = default_media_source();
  let image_struct = content::image( source_img.clone() ); // Uses Image::new internally
  let image_content = ContentAny::Image( image_struct ); // Wrap it
  match &image_content
  {
    ContentAny::Image( img ) =>
      assert_eq!( img.source, source_img ),
    _ =>
      panic!( "Expected Image variant" ),
  }


  // Sound (using the struct's ::new method via builder)
  let source_snd = default_media_source();
  let sound_struct = content::sound( source_snd.clone() ); // Uses Sound::new internally
  let sound_content = ContentAny::Sound( sound_struct ); // Wrap it
  match &sound_content
  {
    ContentAny::Sound( snd ) =>
      assert_eq!( snd.source, source_snd ),
    _ =>
      panic!( "Expected Sound variant" ),
  }

  // Video (using the struct's ::new method via builder)
  let source_vid = default_media_source();
  let video_struct = content::video( source_vid.clone() ); // Uses Video::new internally
  let video_content = ContentAny::Video( video_struct ); // Wrap it
  match &video_content
  {
    ContentAny::Video( vid ) =>
      assert_eq!( vid.source, source_vid ),
    _ =>
      panic!( "Expected Video variant" ),
  }

  // Pdf (using the struct's ::new method via builder)
  let source_pdf = default_media_source();
  let pdf_struct = content::pdf( source_pdf.clone() ); // Uses Pdf::new internally
  let pdf_content = ContentAny::Pdf( pdf_struct ); // Wrap it
  match &pdf_content
  {
    ContentAny::Pdf( pdf ) =>
      assert_eq!( pdf.source, source_pdf ),
    _ =>
      panic!( "Expected Pdf variant" ),
  }

  // File (using the struct's ::new method via builder)
  let source_file = default_media_source();
  let file_struct = content::file( source_file.clone() ); // Uses File::new internally
  let file_content = ContentAny::File( file_struct ); // Wrap it
  match &file_content
  {
    ContentAny::File( f ) =>
      assert_eq!( f.source, source_file ),
    _ =>
      panic!( "Expected File variant" ),
  }

  // Test construction via ContentAny associated functions directly
  let direct_image = ContentAny::image( default_media_source() );
  match &direct_image
  {
    ContentAny::Image( img ) =>
      assert_eq!( img.source, default_media_source() ),
    _ =>
      panic!( "Expected Image variant from direct constructor" ),
  }
  let direct_sound = ContentAny::sound( default_media_source() );
  match &direct_sound
  {
    ContentAny::Sound( snd ) =>
      assert_eq!( snd.source, default_media_source() ),
    _ =>
      panic!( "Expected Sound variant from direct constructor" ),
  }
  let direct_video = ContentAny::video( default_media_source() );
  match &direct_video
  {
    ContentAny::Video( vid ) =>
      assert_eq!( vid.source, default_media_source() ),
    _ =>
      panic!( "Expected Video variant from direct constructor" ),
  }
  let direct_pdf = ContentAny::pdf( default_media_source() );
  match &direct_pdf
  {
    ContentAny::Pdf( pdf ) =>
      assert_eq!( pdf.source, default_media_source() ),
    _ =>
      panic!( "Expected Pdf variant from direct constructor" ),
  }
  let direct_file = ContentAny::file( default_media_source() );
  match &direct_file
  {
    ContentAny::File( f ) =>
      assert_eq!( f.source, default_media_source() ),
    _ =>
      panic!( "Expected File variant from direct constructor" ),
  }
}

/// Tests the `content_type()` method from the `ContentLike` trait for each variant.
#[ test ]
fn content_like_content_type()
{
  // Local use for types already imported at top level
  // Local use for builder functions already imported at top level
  use serde_json; // Local use

  type CA = ContentAny< Vec< u8 > >; // Alias for brevity

  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_type( &content::null() ), ContentType::Null );
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_type( &content::bool( true ) ), ContentType::Bool );
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_type( &content::string( "".into() ) ), ContentType::String );
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_type( &content::number( serde_json::Number::from( 0 ) ) ), ContentType::Number );
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_type( &content::array() ), ContentType::Array );
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_type( &ContentAny::Image( default_image() ) ), ContentType::Image );
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_type( &ContentAny::Sound( default_sound() ) ), ContentType::Sound );
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_type( &ContentAny::Video( default_video() ) ), ContentType::Video );
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_type( &ContentAny::Pdf( default_pdf() ) ), ContentType::Pdf );
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_type( &ContentAny::File( default_file() ) ), ContentType::File );
}

/// Tests the `content_to_bytes()` method from the `ContentLike` trait for each variant.
#[ test ]
fn content_like_content_to_bytes()
{
  // Local use for types already imported at top level
  // Local use for builder functions already imported at top level
  use serde_json; // Local use

  type CA = ContentAny< Vec< u8 > >; // Alias for brevity

  // Null -> JSON "null" bytes
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_bytes( content::null() ), b"null" );
  // Bool -> JSON bool bytes
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_bytes( content::bool( true ) ), b"true" );
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_bytes( content::bool( false ) ), b"false" );
  // String -> Raw string bytes
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_bytes( content::string( "abc".into() ) ), b"abc" );
  // Number -> JSON number bytes
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_bytes( content::number( serde_json::Number::from( 123 ) ) ), b"123" );
  // Array -> Concatenated bytes of elements
  let mut arr = content::array::< Vec< u8 > >();
  arr = ContentAny::push( arr, content::string( "a".into() ) );
  arr = ContentAny::push( arr, content::string( "bc".into() ) );
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_bytes( arr ), b"abc" );
  // Media types -> Raw data bytes
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_bytes( ContentAny::Image( default_image() ) ), vec![ 1, 2, 3 ] );
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_bytes( ContentAny::Sound( default_sound() ) ), vec![ 1, 2, 3 ] );
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_bytes( ContentAny::Video( default_video() ) ), vec![ 1, 2, 3 ] );
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_bytes( ContentAny::Pdf( default_pdf() ) ), vec![ 1, 2, 3 ] );
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_bytes( ContentAny::File( default_file() ) ), vec![ 1, 2, 3 ] );
}

/// Tests the `content_to_json()` method from the `ContentLike` trait (relies on From impl).
#[ test ]
fn content_like_content_to_json()
{
  // Local use for types already imported at top level
  // Local use for builder functions already imported at top level
  use serde_json; // Local use

  type CA = ContentAny< Vec< u8 > >; // Alias for brevity

  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_json( content::null() ), serde_json::Value::Null );
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_json( content::bool( true ) ), serde_json::Value::Bool( true ) );
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_json( content::string( "s".into() ) ), serde_json::Value::String( "s".into() ) );
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_json( content::number( serde_json::Number::from( 1 ) ) ), serde_json::Value::Number( 1.into() ) );

  let mut arr = content::array::< Vec< u8 > >();
  arr = ContentAny::push( arr, content::bool( true ) );
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_json( arr ), serde_json::json!( [ true ] ) );

  let expected_media_json = serde_json::json!
  ({
    // Type depends on the specific media variant tested below
    "media_type" : "application/octet-stream",
    "encoding" : "binary"
  });

  let mut img_json = expected_media_json.clone();
  img_json[ "type" ] = "image".into();
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_json( ContentAny::Image( default_image() ) ), img_json );

  let mut snd_json = expected_media_json.clone();
  snd_json[ "type" ] = "sound".into();
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_json( ContentAny::Sound( default_sound() ) ), snd_json );

  let mut vid_json = expected_media_json.clone();
  vid_json[ "type" ] = "video".into();
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_json( ContentAny::Video( default_video() ) ), vid_json );

  let mut pdf_json = expected_media_json.clone();
  pdf_json[ "type" ] = "pdf".into();
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_json( ContentAny::Pdf( default_pdf() ) ), pdf_json );

  let mut file_json = expected_media_json.clone();
  file_json[ "type" ] = "file".into();
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::content_to_json( ContentAny::File( default_file() ) ), file_json );
}

/// Tests the `into_any()` method from the `ContentLike` trait.
#[ test ]
fn content_like_into_any()
{
  // Local use for types already imported at top level
  // Local use for builder functions already imported at top level

  type CA = ContentAny< Vec< u8 > >; // Alias for brevity

  let val = content::string::< Vec< u8 > >( "test".into() );
  let val_clone = val.clone();
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::into_any( val ), val_clone );

  let val_media = ContentAny::File( default_file() );
  let val_media_clone = val_media.clone();
  assert_eq!( < CA as ContentLike< Vec< u8 > > >::into_any( val_media ), val_media_clone );
}

/// Tests the `push_to()` method from the `ContentLike` trait.
#[ test ]
fn content_like_push_to()
{
  // Local use for types already imported at top level
  // Local use for builder functions already imported at top level

  let mut target_array = content::array::< Vec< u8 > >();
  let _element = content::bool::< Vec< u8 > >( true );

  // Push element using its push_to method
  target_array = < bool as ContentLike< Vec< u8 > > >::push_to( true, target_array );

  match target_array
  {
    ContentAny::Array( arr ) =>
    {
      assert_eq!( arr.len(), 1 );
      assert_eq!( arr[ 0 ], ContentAny::Bool( true ) );
    }
    _ =>
      panic!( "push_to did not result in an array" ),
  }
}

/// Tests that `push_to()` panics when the target is not an array.
#[ test ]
#[ should_panic ]
fn content_like_push_to_panic()
{
  // Local use for types already imported at top level
  // Local use for builder functions already imported at top level

  let target_not_array = content::string::< Vec< u8 > >( "target".into() );
  let _ = < bool as ContentLike< Vec< u8 > > >::push_to( true, target_not_array );
}

/// Tests the `From<Media>` implementations for `ContentAny`.
#[ test ]
fn from_media_impls()
{
  // Local use for types already imported at top level

  let img = default_image();
  let img_clone = img.clone();
  let ca_img = ContentAny::from( img );
  assert_eq!( ca_img, ContentAny::Image( img_clone ) );

  let snd = default_sound();
  let snd_clone = snd.clone();
  let ca_snd = ContentAny::from( snd );
  assert_eq!( ca_snd, ContentAny::Sound( snd_clone ) );

  let vid = default_video();
  let vid_clone = vid.clone();
  let ca_vid = ContentAny::from( vid );
  assert_eq!( ca_vid, ContentAny::Video( vid_clone ) );

  let pdf = default_pdf();
  let pdf_clone = pdf.clone();
  let ca_pdf = ContentAny::from( pdf );
  assert_eq!( ca_pdf, ContentAny::Pdf( pdf_clone ) );

  let file = default_file();
  let file_clone = file.clone();
  let ca_file = ContentAny::from( file );
  assert_eq!( ca_file, ContentAny::File( file_clone ) );
}

/// Tests the `From<ContentAny>` implementation for `serde_json::Value`.
#[ test ]
fn from_content_any_for_json()
{
  // Local use for types already imported at top level
  // Local use for builder functions already imported at top level
  use serde_json; // Local use

  assert_eq!( serde_json::Value::from( content::null::< Vec< u8 > >() ), serde_json::Value::Null );
  assert_eq!( serde_json::Value::from( content::bool::< Vec< u8 > >( false ) ), serde_json::Value::Bool( false ) );
  // ... other non-media types tested in content_like_content_to_json ...

  let expected_media_json = serde_json::json!
  ({
    "media_type" : "application/octet-stream",
    "encoding" : "binary"
  });

  let mut img_json = expected_media_json.clone();
  img_json[ "type" ] = "image".into();
  assert_eq!( serde_json::Value::from( ContentAny::Image( default_image() ) ), img_json );
  // ... other media types tested in content_like_content_to_json ...
}

/// Tests `TryFrom<serde_json::Value>` for `ContentAny<Vec<u8>>`.
#[ test ]
fn try_from_json_for_content_any_vec_u8()
{
  // Local use for types already imported at top level
  // Local use for builder functions already imported at top level
  use serde_json; // Local use

  // Success cases
  assert_eq!( ContentAny::< Vec< u8 > >::try_from( serde_json::Value::Null ).unwrap(), content::null() );
  assert_eq!( ContentAny::< Vec< u8 > >::try_from( serde_json::Value::Bool( true ) ).unwrap(), content::bool( true ) );
  assert_eq!( ContentAny::< Vec< u8 > >::try_from( serde_json::Value::String( "hi".into() ) ).unwrap(), content::string( "hi".into() ) );
  let num = serde_json::Number::from( 42 );
  assert_eq!( ContentAny::< Vec< u8 > >::try_from( serde_json::Value::Number( num.clone() ) ).unwrap(), content::number( num ) );
  let json_arr = serde_json::json!( [ true, "a" ] );
  let expected_ca_arr = ContentAny::Array( vec![ ContentAny::Bool( true ), ContentAny::String( "a".into() ) ] );
  assert_eq!( ContentAny::< Vec< u8 > >::try_from( json_arr ).unwrap(), expected_ca_arr );

  // Failure case (Object representing media)
  let json_obj = serde_json::json!( { "type": "file", "media_type": "..." } );
  assert!( ContentAny::< Vec< u8 > >::try_from( json_obj ).is_err() );
}

/// Tests `TryFrom<serde_json::Value>` for `ContentAny<String>`.
#[ test ]
fn try_from_json_for_content_any_string()
{
  // Local use for types already imported at top level
  // Local use for builder functions already imported at top level
  use serde_json; // Local use

  // Success cases (same as Vec<u8> for non-media)
  assert_eq!( ContentAny::< String >::try_from( serde_json::Value::Null ).unwrap(), content::null() );
  assert_eq!( ContentAny::< String >::try_from( serde_json::Value::Bool( true ) ).unwrap(), content::bool( true ) );
  assert_eq!( ContentAny::< String >::try_from( serde_json::Value::String( "hi".into() ) ).unwrap(), content::string( "hi".into() ) );
  let num = serde_json::Number::from( 42 );
  assert_eq!( ContentAny::< String >::try_from( serde_json::Value::Number( num.clone() ) ).unwrap(), content::number( num ) );
  let json_arr = serde_json::json!( [ false, "b" ] );
  let expected_ca_arr = ContentAny::Array( vec![ ContentAny::Bool( false ), ContentAny::String( "b".into() ) ] );
  assert_eq!( ContentAny::< String >::try_from( json_arr ).unwrap(), expected_ca_arr );

  // Failure case (Object representing media)
  let json_obj = serde_json::json!( { "type": "image", "media_type": "..." } );
  assert!( ContentAny::< String >::try_from( json_obj ).is_err() );
}

/// Tests `ContentAny::push()` associated function.
#[ test ]
fn array_push_associated_fn()
{
  // Local use for types already imported at top level
  // Local use for builder functions already imported at top level

  let mut arr = content::array::< Vec< u8 > >();
  let element1 = content::bool::< Vec< u8 > >( true );
  let element2 = content::string::< Vec< u8 > >( "next".into() );

  arr = ContentAny::push( arr, element1 );
  arr = ContentAny::push( arr, element2 );

  match arr
  {
    ContentAny::Array( vec ) =>
    {
      assert_eq!( vec.len(), 2 );
      assert_eq!( vec[ 0 ], ContentAny::Bool( true ) );
      assert_eq!( vec[ 1 ], ContentAny::String( "next".into() ) );
    }
    _ =>
      panic!( "Expected Array variant after push" ),
  }
}

/// Tests that `ContentAny::push()` panics on non-array variants.
#[ test ]
#[ should_panic ]
fn array_push_associated_fn_panic()
{
  // Local use for types already imported at top level
  // Local use for builder functions already imported at top level

  let not_arr = content::null::< Vec< u8 > >();
  let element = content::bool::< Vec< u8 > >( true );
  let _ = ContentAny::push( not_arr, element );
}

/// Tests `ContentAny::extend()` associated function.
#[ test ]
fn array_extend_associated_fn()
{
  // Local use for types already imported at top level
  // Local use for builder functions already imported at top level

  let mut arr = content::array::< Vec< u8 > >();
  arr = ContentAny::push( arr, content::bool( false ) ); // Start with one element

  let elements_to_extend = vec!
  [
    content::string::< Vec< u8 > >( "add".into() ),
    content::null::< Vec< u8 > >(),
  ];

  arr = ContentAny::extend( arr, elements_to_extend );

  match arr
  {
    ContentAny::Array( vec ) =>
    {
      assert_eq!( vec.len(), 3 );
      assert_eq!( vec[ 0 ], ContentAny::Bool( false ) );
      assert_eq!( vec[ 1 ], ContentAny::String( "add".into() ) );
      assert_eq!( vec[ 2 ], ContentAny::Null );
    }
    _ =>
      panic!( "Expected Array variant after extend" ),
  }
}

/// Tests that `ContentAny::extend()` panics on non-array variants.
#[ test ]
#[ should_panic ]
fn array_extend_associated_fn_panic()
{
  // Local use for types already imported at top level
  // Local use for builder functions already imported at top level

  let not_arr = content::string::< Vec< u8 > >( "base".into() );
  let elements = vec![ content::bool::< Vec< u8 > >( true ) ];
  let _ = ContentAny::extend( not_arr, elements );
}

/// Tests equality between different ContentAny variants.
#[ test ]
fn equality_tests()
{
  // Local use for types already imported at top level
  // Local use for builder functions already imported at top level
  use serde_json; // Local use

  // Identical variants
  assert_eq!( content::null::< Vec< u8 > >(), content::null::< Vec< u8 > >() );
  assert_eq!( content::bool::< Vec< u8 > >( true ), content::bool::< Vec< u8 > >( true ) );
  assert_eq!( content::string::< Vec< u8 > >( "a".into() ), content::string::< Vec< u8 > >( "a".into() ) );
  assert_eq!( content::number::< Vec< u8 > >( 1.into() ), content::number::< Vec< u8 > >( 1.into() ) );
  assert_eq!( content::array::< Vec< u8 > >(), content::array::< Vec< u8 > >() );
  assert_eq!( ContentAny::Image( default_image() ), ContentAny::Image( default_image() ) );
  // ... other media types ...
  assert_eq!( ContentAny::File( default_file() ), ContentAny::File( default_file() ) );

  // Different values within same variant
  assert_ne!( content::bool::< Vec< u8 > >( true ), content::bool::< Vec< u8 > >( false ) );
  assert_ne!( content::string::< Vec< u8 > >( "a".into() ), content::string::< Vec< u8 > >( "b".into() ) );
  assert_ne!( content::number::< Vec< u8 > >( 1.into() ), content::number::< Vec< u8 > >( 2.into() ) );
  let mut arr1 = content::array::< Vec< u8 > >();
  arr1 = ContentAny::push( arr1, content::bool( true ) );
  let mut arr2 = content::array::< Vec< u8 > >();
  arr2 = ContentAny::push( arr2, content::bool( false ) );
  assert_ne!( arr1, arr2 );
  // Media types with different sources (assuming Source PartialEq works)
  let mut src2 = default_media_source();
  src2.media_type = "other".into();
  assert_ne!( ContentAny::File( default_file() ), ContentAny::File( File { source : src2 } ) );

  // Different variants
  assert_ne!( content::null::< Vec< u8 > >(), content::bool::< Vec< u8 > >( false ) );
  assert_ne!( content::string::< Vec< u8 > >( "".into() ), content::array::< Vec< u8 > >() );
  assert_ne!( content::number::< Vec< u8 > >( 0.into() ), ContentAny::File( default_file() ) );
}