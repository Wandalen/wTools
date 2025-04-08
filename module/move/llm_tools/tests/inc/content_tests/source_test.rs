
use super::*;
use core::fmt;

/// Tests constructing a Source using `Default::default()`.
#[ test ]
fn construct_with_default()
{
  use the_module::content::Source; // Local use

  let source_default = Source::< Vec< u8 > >::default();
  assert_eq!( source_default.media_type, "" );
  assert_eq!( source_default.encoding, "" );
  assert_eq!( source_default.data, Vec::< u8 >::default() );
}

/// Tests constructing a Source using the `content::source()` shortcut function.
#[ test ]
fn construct_with_function_constructor()
{
  use the_module::content; // Local use

  let source = content::source::< Vec< u8 > >();
  // The function constructor just calls default, so check default values
  assert_eq!( source.media_type, "" );
  assert_eq!( source.encoding, "" );
  assert_eq!( source.data, Vec::< u8 >::default() );
}

/// Tests setting the media_type using the builder method.
#[ test ]
fn builder_media_type()
{
  use the_module::content; // Local use

  let source = content::source::< Vec< u8 > >()
    .media_type( "image/png".to_string() );

  assert_eq!( source.media_type, "image/png" );
  assert_eq!( source.encoding, "" ); // Other fields remain default
  assert_eq!( source.data, Vec::< u8 >::default() );
}

/// Tests setting the encoding using the builder method.
#[ test ]
fn builder_encoding()
{
  use the_module::content; // Local use

  let source = content::source::< Vec< u8 > >()
    .encoding( "base64".to_string() );

  assert_eq!( source.media_type, "" ); // Other fields remain default
  assert_eq!( source.encoding, "base64" );
  assert_eq!( source.data, Vec::< u8 >::default() );
}

/// Tests setting the data using the builder method.
#[ test ]
fn builder_data()
{
  use the_module::content; // Local use

  let data = vec![ 10, 20 ];
  let source = content::source::< Vec< u8 > >()
    .data( data.clone() );

  assert_eq!( source.media_type, "" ); // Other fields remain default
  assert_eq!( source.encoding, "" );
  assert_eq!( source.data, data );
}

/// Tests chaining multiple builder methods.
#[ test ]
fn builder_chained()
{
  use the_module::content; // Local use

  let data = vec![ 1, 2, 3 ];
  let source = content::source::< Vec< u8 > >()
    .media_type( "audio/mpeg".to_string() )
    .encoding( "binary".to_string() )
    .data( data.clone() );

  assert_eq!( source.media_type, "audio/mpeg" );
  assert_eq!( source.encoding, "binary" );
  assert_eq!( source.data, data );
}

/// Tests the equality and inequality comparisons for Source instances.
#[ test ]
fn equality()
{
  use the_module::content; // Local use

  let source1 = content::source::< Vec< u8 > >()
    .media_type( "a".into() ).encoding( "b".into() ).data( vec![ 1 ] );
  let source2 = content::source::< Vec< u8 > >()
    .media_type( "a".into() ).encoding( "b".into() ).data( vec![ 1 ] );
  let source3 = content::source::< Vec< u8 > >() // Different media_type
    .media_type( "x".into() ).encoding( "b".into() ).data( vec![ 1 ] );
  let source4 = content::source::< Vec< u8 > >() // Different encoding
    .media_type( "a".into() ).encoding( "y".into() ).data( vec![ 1 ] );
  let source5 = content::source::< Vec< u8 > >() // Different data
    .media_type( "a".into() ).encoding( "b".into() ).data( vec![ 9 ] );

  assert_eq!( source1, source2 ); // Identical
  assert_ne!( source1, source3 ); // Different media_type
  assert_ne!( source1, source4 ); // Different encoding
  assert_ne!( source1, source5 ); // Different data
}

/// Tests converting a Source into an Image using `.image()`.
#[ test ]
fn convert_to_image()
{
  use the_module::content; // Local use
  use the_module::content::Image; // Local use

  let data = vec![ 11, 22 ];
  let source = content::source::< Vec< u8 > >()
    .media_type( "img".into() ).encoding( "enc".into() ).data( data.clone() );
  let source_clone = source.clone(); // Clone for comparison

  let image : Image< Vec< u8 > > = source.image(); // Consumes source

  assert_eq!( image.source, source_clone );
  assert_eq!( image.source.media_type, "img" );
  assert_eq!( image.source.encoding, "enc" );
  assert_eq!( image.source.data, data );
}

/// Tests converting a Source into a Sound using `.sound()`.
#[ test ]
fn convert_to_sound()
{
  use the_module::content; // Local use
  use the_module::content::Sound; // Local use

  let data = vec![ 33, 44 ];
  let source = content::source::< Vec< u8 > >()
    .media_type( "snd".into() ).encoding( "enc2".into() ).data( data.clone() );
  let source_clone = source.clone(); // Clone for comparison

  let sound : Sound< Vec< u8 > > = source.sound(); // Consumes source

  assert_eq!( sound.source, source_clone );
  assert_eq!( sound.source.media_type, "snd" );
  assert_eq!( sound.source.encoding, "enc2" );
  assert_eq!( sound.source.data, data );
}

/// Tests converting a Source into a Video using `.video()`.
#[ test ]
fn convert_to_video()
{
  use the_module::content; // Local use
  use the_module::content::Video; // Local use

  let data = vec![ 55, 66 ];
  let source = content::source::< Vec< u8 > >()
    .media_type( "vid".into() ).encoding( "enc3".into() ).data( data.clone() );
  let source_clone = source.clone(); // Clone for comparison

  let video : Video< Vec< u8 > > = source.video(); // Consumes source

  assert_eq!( video.source, source_clone );
  assert_eq!( video.source.media_type, "vid" );
  assert_eq!( video.source.encoding, "enc3" );
  assert_eq!( video.source.data, data );
}

/// Tests converting a Source into a Pdf using `.pdf()`.
#[ test ]
fn convert_to_pdf()
{
  use the_module::content; // Local use
  use the_module::content::Pdf; // Local use

  let data = vec![ 77, 88 ];
  let source = content::source::< Vec< u8 > >()
    .media_type( "pdf".into() ).encoding( "enc4".into() ).data( data.clone() );
  let source_clone = source.clone(); // Clone for comparison

  let pdf : Pdf< Vec< u8 > > = source.pdf(); // Consumes source

  assert_eq!( pdf.source, source_clone );
  assert_eq!( pdf.source.media_type, "pdf" );
  assert_eq!( pdf.source.encoding, "enc4" );
  assert_eq!( pdf.source.data, data );
}

/// Tests converting a Source into a File using `.file()`.
#[ test ]
fn convert_to_file()
{
  use the_module::content; // Local use
  use the_module::content::File; // Local use

  let data = vec![ 99, 00 ];
  let source = content::source::< Vec< u8 > >()
    .media_type( "file".into() ).encoding( "enc5".into() ).data( data.clone() );
  let source_clone = source.clone(); // Clone for comparison

  let file : File< Vec< u8 > > = source.file(); // Consumes source

  assert_eq!( file.source, source_clone );
  assert_eq!( file.source.media_type, "file" );
  assert_eq!( file.source.encoding, "enc5" );
  assert_eq!( file.source.data, data );
}