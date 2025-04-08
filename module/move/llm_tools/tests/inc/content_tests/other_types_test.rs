// Assuming this code resides in a file like `tests/content_media_types_integration_test.rs`
// Replace `the_module` with the actual name of your crate.

use super::*;
use core::fmt;

// Using Vec<u8> directly as the Data type for simplicity
type CA = the_module::content::ContentAny< Vec< u8 > >;

// Helper to create a default Source<Vec<u8>>
fn default_media_source( media_type : &str, encoding : &str ) -> the_module::content::Source< Vec< u8 > >
{
  the_module::content::Source::< Vec< u8 > >
  {
    media_type : media_type.to_string(),
    encoding : encoding.to_string(),
    data : vec![ 1, 2, 3 ],
  }
}

/// Tests basic construction and PartialEq for Image, Sound, Video, Pdf.
#[ cfg( test ) ]
mod construction_and_equality
{
  use super::*;
  use the_module::content::{ Image, Sound, Video, Pdf, Source }; // Local use

  /// Tests ::new(), Default::default(), and PartialEq for Image.
  #[ test ]
  fn image()
  {
    let source = default_media_source( "image/png", "bin" );
    let source_clone_for_assert = source.clone(); // Clone before potential partial move
    let img1 = Image::new( source.clone() );
    let img2 = Image::new( source.clone() );
    let img_default = Image::< Vec< u8 > >::default();
    // Create a new source with different data instead of partial move
    let diff_source = Source { data: vec![9], ..source }; // source is partially moved here
    let img_diff_source = Image::new( diff_source );

    assert_eq!( img1.source, source_clone_for_assert ); // Compare with the clone
    assert_eq!( img1, img2 );
    assert_ne!( img1, img_default );
    assert_ne!( img1, img_diff_source );
    assert_eq!( img_default.source, Source::default() );
  }

  /// Tests ::new(), Default::default(), and PartialEq for Sound.
  #[ test ]
  fn sound()
  {
    let source = default_media_source( "audio/mp3", "bin" );
    let source_clone_for_assert = source.clone(); // Clone before potential partial move
    let snd1 = Sound::new( source.clone() );
    let snd2 = Sound::new( source.clone() );
    let snd_default = Sound::< Vec< u8 > >::default();
    // Create a new source with different data instead of partial move
    let diff_source = Source { data: vec![9], ..source }; // source is partially moved here
    let snd_diff_source = Sound::new( diff_source );

    assert_eq!( snd1.source, source_clone_for_assert ); // Compare with the clone
    assert_eq!( snd1, snd2 );
    assert_ne!( snd1, snd_default );
    assert_ne!( snd1, snd_diff_source );
    assert_eq!( snd_default.source, Source::default() );
  }

  /// Tests ::new(), Default::default(), and PartialEq for Video.
  #[ test ]
  fn video()
  {
    let source = default_media_source( "video/mp4", "bin" );
    let source_clone_for_assert = source.clone(); // Clone before potential partial move
    let vid1 = Video::new( source.clone() );
    let vid2 = Video::new( source.clone() );
    let vid_default = Video::< Vec< u8 > >::default();
    // Create a new source with different data instead of partial move
    let diff_source = Source { data: vec![9], ..source }; // source is partially moved here
    let vid_diff_source = Video::new( diff_source );

    assert_eq!( vid1.source, source_clone_for_assert ); // Compare with the clone
    assert_eq!( vid1, vid2 );
    assert_ne!( vid1, vid_default );
    assert_ne!( vid1, vid_diff_source );
    assert_eq!( vid_default.source, Source::default() );
  }

  /// Tests ::new(), Default::default(), and PartialEq for Pdf.
  #[ test ]
  fn pdf()
  {
    let source = default_media_source( "application/pdf", "bin" );
    let source_clone_for_assert = source.clone(); // Clone before potential partial move
    let pdf1 = Pdf::new( source.clone() );
    let pdf2 = Pdf::new( source.clone() );
    let pdf_default = Pdf::< Vec< u8 > >::default();
    // Create a new source with different data instead of partial move
    let diff_source = Source { data: vec![9], ..source }; // source is partially moved here
    let pdf_diff_source = Pdf::new( diff_source );

    assert_eq!( pdf1.source, source_clone_for_assert ); // Compare with the clone
    assert_eq!( pdf1, pdf2 );
    assert_ne!( pdf1, pdf_default );
    assert_ne!( pdf1, pdf_diff_source );
    assert_eq!( pdf_default.source, Source::default() );
  }

  /// Tests the `.source()` builder method.
  #[ test ]
  fn source_builder_method()
  {
    use the_module::content::Image; // Test with one type, pattern is the same

    let source = default_media_source( "image/jpeg", "base64" );
    let img = Image::default().source( source.clone() );
    assert_eq!( img.source, source );
  }
}

/// Tests the ContentLike implementation for Image, Sound, Video, Pdf.
#[ cfg( test ) ]
mod content_like_impls
{
  use super::*;
  use the_module::content::{ ContentLike, ContentType, ContentAny }; // Local use
  use the_module::content::{ Image, Sound, Video, Pdf }; // Local use
  use the_module::content; // Local use for builder functions
  use serde_json; // Local use

  /// Verifies `content_type()` for each media type.
  #[ test ]
  fn content_type()
  {
    let img = Image::< Vec< u8 > >::default();
    let snd = Sound::< Vec< u8 > >::default();
    let vid = Video::< Vec< u8 > >::default();
    let pdf = Pdf::< Vec< u8 > >::default();

    assert_eq!( <Image<Vec<u8>> as ContentLike<Vec<u8>>>::content_type( &img ), ContentType::Image );
    assert_eq!( <Sound<Vec<u8>> as ContentLike<Vec<u8>>>::content_type( &snd ), ContentType::Sound );
    assert_eq!( <Video<Vec<u8>> as ContentLike<Vec<u8>>>::content_type( &vid ), ContentType::Video );
    assert_eq!( <Pdf<Vec<u8>> as ContentLike<Vec<u8>>>::content_type( &pdf ), ContentType::Pdf );
  }

  /// Verifies `content_to_bytes()` returns the source data.
  #[ test ]
  fn content_to_bytes()
  {
    let source = default_media_source( "any", "any" );
    let expected_bytes = source.data.clone();

    let img = Image::new( source.clone() );
    let snd = Sound::new( source.clone() );
    let vid = Video::new( source.clone() );
    let pdf = Pdf::new( source.clone() );

    assert_eq!( <Image<Vec<u8>> as ContentLike<Vec<u8>>>::content_to_bytes( img ), expected_bytes );
    assert_eq!( <Sound<Vec<u8>> as ContentLike<Vec<u8>>>::content_to_bytes( snd ), expected_bytes );
    assert_eq!( <Video<Vec<u8>> as ContentLike<Vec<u8>>>::content_to_bytes( vid ), expected_bytes );
    assert_eq!( <Pdf<Vec<u8>> as ContentLike<Vec<u8>>>::content_to_bytes( pdf ), expected_bytes );
  }

  /// Verifies `content_to_json()` produces the correct structure.
  #[ test ]
  fn content_to_json()
  {
    let source = default_media_source( "test/type", "test-enc" );
    let base_json = serde_json::json!
    ({
      "media_type": "test/type",
      "encoding": "test-enc"
    });

    let img = Image::new( source.clone() );
    let mut expected_img = base_json.clone(); expected_img["type"] = "image".into();
    assert_eq!( <Image<Vec<u8>> as ContentLike<Vec<u8>>>::content_to_json( img ), expected_img );

    let snd = Sound::new( source.clone() );
    let mut expected_snd = base_json.clone(); expected_snd["type"] = "sound".into();
    assert_eq!( <Sound<Vec<u8>> as ContentLike<Vec<u8>>>::content_to_json( snd ), expected_snd );

    let vid = Video::new( source.clone() );
    let mut expected_vid = base_json.clone(); expected_vid["type"] = "video".into();
    assert_eq!( <Video<Vec<u8>> as ContentLike<Vec<u8>>>::content_to_json( vid ), expected_vid );

    let pdf = Pdf::new( source.clone() );
    let mut expected_pdf = base_json.clone(); expected_pdf["type"] = "pdf".into();
    assert_eq!( <Pdf<Vec<u8>> as ContentLike<Vec<u8>>>::content_to_json( pdf ), expected_pdf );
  }

  /// Verifies `into_any()` converts correctly.
  #[ test ]
  fn into_any()
  {
    // Test with Image, pattern is the same for others
    let source = default_media_source( "img", "bin" );
    let img = Image::new( source.clone() );
    let img_clone = img.clone(); // For comparison

    let any_content : CA = <Image<Vec<u8>> as ContentLike<Vec<u8>>>::into_any( img );

    match any_content
    {
      ContentAny::Image( inner_img ) => assert_eq!( inner_img, img_clone ),
      _ => panic!( "into_any() did not produce ContentAny::Image" ),
    }
    // Add similar checks for Sound, Video, Pdf if desired, but the logic is identical
  }

  /// Verifies `push_to()` works for media types.
  #[ test ]
  fn push_to_success()
  {
    // Test with Sound, pattern is the same for others
    let snd = Sound::new( default_media_source( "snd", "bin" ) );
    let snd_clone = snd.clone(); // For comparison
    let mut target_array = content::array::< Vec< u8 > >();

    target_array = <Sound<Vec<u8>> as ContentLike<Vec<u8>>>::push_to( snd, target_array );

    match target_array
    {
      ContentAny::Array( arr ) =>
      {
        assert_eq!( arr.len(), 1 );
        match &arr[ 0 ]
        {
          ContentAny::Sound( inner_snd ) => assert_eq!( inner_snd, &snd_clone ),
          _ => panic!( "Element in array is not a Sound variant" ),
        }
      }
      _ => panic!( "push_to did not result in an array" ),
    }
  }

  /// Verifies `push_to()` panics when target is not an array.
  #[ test ]
  #[ should_panic ]
  fn push_to_panic()
  {
    // Test with Video, pattern is the same for others
    let vid = Video::new( default_media_source( "vid", "bin" ) );
    let target_not_array = content::null::< Vec< u8 > >();
    let _ = <Video<Vec<u8>> as ContentLike<Vec<u8>>>::push_to( vid, target_not_array );
  }

  // Test data conversion within into_any (using a mock type)
  #[ derive( Clone, PartialEq, Default, Debug ) ]
  struct MockData( String );
  impl the_module::IntoBytes for MockData { fn into_bytes( self ) -> Vec< u8 > { self.0.into_bytes() } }
  impl From< MockData > for Vec< u8 > { fn from( data : MockData ) -> Self { data.0.into_bytes() } }
  // Removed explicit `impl Data for MockData` - relies on blanket impl


  /// Verifies `into_any()` correctly converts data types via `Into`.
  #[ test ]
  fn into_any_data_conversion()
  {
    let mock_source = the_module::content::Source
    {
      media_type: "mock/type".into(),
      encoding: "mock-enc".into(),
      data: MockData( "mock_data".into() ),
    };
    let pdf_mock = Pdf::new( mock_source );

    // Convert Pdf<MockData> into ContentAny<Vec<u8>>
    let any_content_vec : CA = <Pdf<MockData> as ContentLike<Vec<u8>>>::into_any( pdf_mock );

    match any_content_vec
    {
      ContentAny::Pdf( inner_pdf ) =>
      {
        assert_eq!( inner_pdf.source.media_type, "mock/type" );
        assert_eq!( inner_pdf.source.encoding, "mock-enc" );
        // Check that the data was converted from MockData to Vec<u8>
        assert_eq!( inner_pdf.source.data, b"mock_data".to_vec() );
      }
      _ => panic!( "into_any() with data conversion failed" ),
    }
  }
}
