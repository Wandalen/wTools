/// Template value types and trait
/// Generic template value that can be serialized to strings for template substitution.
///
/// Implement this trait for custom value types to use them with genfile templates.
///
/// # Examples
///
/// ```rust
/// use genfile_core::TemplateValue;
///
/// #[ derive( Clone ) ]
/// struct CustomValue( String );
///
/// impl TemplateValue for CustomValue
/// {
///   fn to_template_string( &self ) -> String
///   {
///     format!( "custom:{}", self.0 )
///   }
///
///   fn from_string( s: String ) -> Self
///   {
///     CustomValue( s )
///   }
///
///   fn is_empty( &self ) -> bool
///   {
///     self.0.is_empty()
///   }
/// }
/// ```
pub trait TemplateValue: Clone + Send + Sync
{
  /// Converts the value to a string suitable for template substitution.
  ///
  /// This method determines how the value appears in rendered templates.
  fn to_template_string( &self ) -> String;

  /// Creates a value from a string.
  ///
  /// Default implementation for simple string-based values.
  fn from_string( s: String ) -> Self;

  /// Checks if the value is considered empty.
  ///
  /// Used to determine if a value has been set or needs prompting.
  fn is_empty( &self ) -> bool;
}

/// Default value type supporting common data types for template rendering.
///
/// Provides built-in support for strings, numbers, booleans, and lists without
/// requiring custom implementations.
///
/// # Variants
///
/// - `String`: UTF-8 text value
/// - `Number`: Signed 64-bit integer
/// - `Bool`: Boolean true/false
/// - `List`: Collection of strings (rendered as comma-separated)
///
/// # Examples
///
/// ```rust
/// use genfile_core::{ Value, TemplateValue };
///
/// let name = Value::String( "genfile".into() );
/// assert_eq!( name.to_template_string(), "genfile" );
///
/// let count = Value::Number( 42 );
/// assert_eq!( count.to_template_string(), "42" );
///
/// let enabled = Value::Bool( true );
/// assert_eq!( enabled.to_template_string(), "true" );
///
/// let items = Value::List( vec![ "a".into(), "b".into() ] );
/// assert_eq!( items.to_template_string(), "a, b" );
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(any(feature = "archive", feature = "template"), derive(serde::Deserialize))]
#[cfg_attr(any(feature = "archive", feature = "template"), serde(untagged))]
pub enum Value
{
  /// Boolean value
  Bool( bool ),
  /// Integer number value
  Number( i64 ),
  /// List of strings (rendered as comma-separated)
  List( Vec< String > ),
  /// String value
  String( String ),
}

#[cfg(any(feature = "archive", feature = "template"))]
impl serde ::Serialize for Value
{
  fn serialize< S >( &self, serializer: S ) -> Result< S ::Ok, S ::Error >
  where
    S: serde ::Serializer,
  {
    match self
    {
      Value ::String( s ) => serializer.serialize_str( s ),
      Value ::Number( n ) => serializer.serialize_i64( *n ),
      Value ::Bool( b ) => serializer.serialize_bool( *b ),
      // Serialize lists as comma-separated strings for simple interpolation
      Value ::List( items ) => serializer.serialize_str( &items.join( ", " ) ),
    }
  }
}

impl TemplateValue for Value
{
  fn to_template_string( &self ) -> String
  {
    match self
    {
      Value ::String( s ) => s.clone(),
      Value ::Number( n ) => n.to_string(),
      Value ::Bool( b ) => b.to_string(),
      Value ::List( items ) => items.join( ", " ),
    }
  }

  fn from_string( s: String ) -> Self
  {
    Value ::String( s )
  }

  fn is_empty( &self ) -> bool
  {
    match self
    {
      Value ::String( s ) => s.is_empty(),
      Value ::Number( _ ) | Value ::Bool( _ ) => false,
      Value ::List( items ) => items.is_empty(),
    }
  }
}
