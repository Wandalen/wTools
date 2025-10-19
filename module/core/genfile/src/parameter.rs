/// Parameter management for templates
/// Parameter descriptor defining a single template parameter.
///
/// Stores metadata about a parameter including its name, whether it's mandatory,
/// optional default value, and description.
///
/// # Examples
///
/// ```rust,ignore
/// use genfile_core::ParameterDescriptor;
///
/// // Direct construction
/// let param = ParameterDescriptor
/// {
///   parameter: "project_name".into(),
///   is_mandatory: true,
///   default_value: None,
///   description: Some( "Name of the project".into() ),
/// };
///
/// // Builder pattern
/// let param = ParameterDescriptor::former()
///   .parameter( "project_name" )
///   .is_mandatory( true )
///   .description( "Name of the project" )
///   .form();
/// ```
#[ derive( Debug, Clone, serde::Serialize, serde::Deserialize ) ]
pub struct ParameterDescriptor
{
  /// Parameter name
  pub parameter: String,
  /// Whether this parameter is mandatory
  pub is_mandatory: bool,
  /// Optional default value
  pub default_value: Option< String >,
  /// Optional description
  pub description: Option< String >,
}

/// Collection of parameter descriptors for a template.
///
/// Provides methods for validation and extracting mandatory parameters.
///
/// # Examples
///
/// ```rust,ignore
/// use genfile_core:: { Parameters, ParameterDescriptor };
///
/// // Direct construction
/// let params = Parameters
/// {
///   descriptors: vec!
///   [
///     ParameterDescriptor
///     {
///       parameter: "name".into(),
///       is_mandatory: true,
///       default_value: None,
///       description: None,
///     },
///   ],
/// };
///
/// // Builder pattern
/// let params = Parameters::former()
///   .descriptors( vec!
///   [
///     ParameterDescriptor::former()
///       .parameter( "name" )
///       .is_mandatory( true )
///       .form(),
///   ])
///   .form();
///
/// let mandatory = params.list_mandatory();
/// assert_eq!( mandatory.len(), 1 );
/// ```
#[ derive( Debug, Clone, Default, serde::Serialize, serde::Deserialize ) ]
pub struct Parameters
{
  /// List of parameter descriptors
  pub descriptors: Vec< ParameterDescriptor >,
}

impl Parameters
{
  /// Returns a list of all mandatory parameter names.
  ///
  /// # Returns
  ///
  /// Vector of parameter names that have `is_mandatory = true`.
  ///
  /// # Examples
  ///
  /// ```rust,ignore
  /// use genfile_core:: { Parameters, ParameterDescriptor };
  ///
  /// let params = Parameters
  /// {
  ///   descriptors: vec!
  ///   [
  ///     ParameterDescriptor
  ///     {
  ///       parameter: "name".into(),
  ///       is_mandatory: true,
  ///       default_value: None,
  ///       description: None,
  ///     },
  ///   ],
  /// };
  ///
  /// let mandatory = params.list_mandatory();
  /// assert_eq!( mandatory, vec![ "name" ] );
  /// ```
  #[must_use] 
  pub fn list_mandatory( &self ) -> Vec< &str >
  {
    self
      .descriptors
      .iter()
      .filter( | d | d.is_mandatory )
      .map( | d | d.parameter.as_str() )
      .collect()
  }
}
