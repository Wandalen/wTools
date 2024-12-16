
use super::*;
use openai_api_rs::v1::file::FileData;

// Assuming the `format_tools` module and `field!` macro are defined elsewhere

/// A wrapper for `FileData` to make pretty print.
#[ derive( Debug ) ]
pub struct FileDataWrap( pub FileData );

/// Manually implemented `Clone`, as `FileData` does not implement it.
impl Clone for FileDataWrap
{
  fn clone( &self ) -> Self
  {
    FileDataWrap( FileData
    {
      id : self.0.id.clone(),
      object : self.0.object.clone(),
      bytes : self.0.bytes,
      created_at : self.0.created_at,
      filename : self.0.filename.clone(),
      purpose : self.0.purpose.clone(),
    } )
  }
}

impl TableWithFields for FileDataWrap {}
impl Fields< &'_ str, Option< Cow< '_, str > > >
for FileDataWrap
{
  type Key<'k> = &'k str;
  type Val< 'v > = Option< Cow< 'v, str > >;

  fn fields( &self ) -> impl format_tools::IteratorTrait< Item = ( &'_ str, Option< Cow< '_, str > > ) >
  {
    use format_tools::ref_or_display_or_debug_multiline::field;
    let mut dst = Vec::new();

    // Use the field! macro for direct field references
    dst.push( field!( &self.0.id ) );
    dst.push( field!( &self.0.object ) );
    dst.push( ( "bytes", Some( Cow::Owned( self.0.bytes.to_string() ) ) ) );
    dst.push( ( "created_at", Some( Cow::Owned( self.0.created_at.to_string() ) ) ) );
    dst.push( field!( &self.0.filename ) );
    dst.push( field!( &self.0.purpose ) );

    dst.into_iter()
  }
}
