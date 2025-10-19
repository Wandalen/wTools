/// Error types for genfile operations
/// Errors that can occur during template processing and file generation.
///
/// Provides typed error variants for different failure modes, enabling
/// users to match and handle specific error cases.
///
/// # Examples
///
/// ```rust,ignore
/// use genfile_core::Error;
///
/// fn process() -> Result< (), Error >
/// {
///   // ... template processing
///   Err( Error::Render( "Invalid template syntax".into() ) )
/// }
/// ```
#[ derive( Debug ) ]
pub enum Error
{
  /// Template rendering failed.
  ///
  /// Occurs when the template engine encounters invalid syntax or
  /// fails to substitute variables.
  Render( String ),

  /// Missing mandatory parameters.
  ///
  /// Contains list of parameter names that are required but not provided.
  MissingParameters( Vec< String > ),

  /// File system operation failed.
  ///
  /// Wraps underlying I/O errors from filesystem operations.
  Fs( std ::io ::Error ),

  /// Invalid template.
  ///
  /// Template content is malformed or unsupported.
  InvalidTemplate( String ),
}

impl core ::fmt ::Display for Error
{
  fn fmt( &self, f: &mut core ::fmt ::Formatter< '_ > ) -> core ::fmt ::Result
  {
    match self
    {
      Error ::Render( msg ) => write!( f, "Template rendering failed: {msg}" ),
      Error ::MissingParameters( params ) =>
        write!( f, "Missing mandatory parameters: {params:?}" ),
      Error ::Fs( err ) => write!( f, "File system error: {err}" ),
      Error ::InvalidTemplate( msg ) => write!( f, "Invalid template: {msg}" ),
    }
  }
}

impl core ::error ::Error for Error
{
  fn source( &self ) -> Option< &( dyn core ::error ::Error + 'static ) >
  {
    match self
    {
      Error ::Fs( err ) => Some( err ),
      _ => None,
    }
  }
}

impl From< std ::io ::Error > for Error
{
  fn from( err: std ::io ::Error ) -> Self
  {
    Error ::Fs( err )
  }
}

impl From< handlebars ::RenderError > for Error
{
  fn from( err: handlebars ::RenderError ) -> Self
  {
    Error ::Render( err.to_string() )
  }
}

impl From< handlebars ::TemplateError > for Error
{
  fn from( err: handlebars ::TemplateError ) -> Self
  {
    Error ::Render( err.to_string() )
  }
}
