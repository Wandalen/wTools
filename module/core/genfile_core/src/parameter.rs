/// Parameter management for templates
/// Parameter descriptor defining a single template parameter.
///
/// Stores metadata about a parameter including its name, whether it's mandatory,
/// optional default value, and description.
///
/// # Examples
///
/// ```rust
/// use genfile_core::ParameterDescriptor;
///
/// let param = ParameterDescriptor
/// {
///   parameter: "project_name".into(),
///   is_mandatory: true,
///   default_value: None,
///   description: Some( "Name of the project".into() ),
/// };
/// assert_eq!( param.parameter, "project_name" );
/// assert!( param.is_mandatory );
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(any(feature = "json", feature = "yaml"), derive(serde::Serialize, serde::Deserialize))]
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
/// ```rust
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
/// assert_eq!( params.descriptors.len(), 1 );
/// ```
#[derive(Debug, Clone, Default)]
#[cfg_attr(any(feature = "json", feature = "yaml"), derive(serde::Serialize, serde::Deserialize))]
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
  /// ```rust
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
