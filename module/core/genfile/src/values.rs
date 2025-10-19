/// Value storage for template parameters
use std ::collections ::{ HashMap, BTreeMap };
use crate ::TemplateValue;

/// Stores parameter values for template rendering.
///
/// Generic container holding runtime values for template parameters. Supports
/// insertion, retrieval, serialization, and detection of missing values.
///
/// # Type Parameters
///
/// - `V`: The value type, must implement `TemplateValue` trait
///
/// # Examples
///
/// ```rust
/// use genfile_core::{ Values, Value };
///
/// let mut values = Values::new();
/// values.insert( "project_name", Value::String( "my_project".into() ) );
/// values.insert( "version", Value::Number( 1 ) );
///
/// // Convert to serializable format for template rendering (preserves types)
/// let serialized = values.to_serializable();
/// assert!( serialized.contains_key( "project_name" ) );
/// assert!( serialized.contains_key( "version" ) );
/// ```
#[ derive( Debug, Clone ) ]
#[ cfg_attr( feature = "enabled", derive( serde::Serialize, serde::Deserialize ) )]
pub struct Values< V = crate ::Value >
where
  V: TemplateValue
{
  /// Internal storage mapping parameter names to optional values
  inner: HashMap< String, Option< V > >,
}

impl< V > Values< V >
where
  V: TemplateValue
{
  /// Creates a new empty values collection.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ Values, Value };
  ///
  /// let values = Values::< Value >::new();
  /// assert_eq!( values.len(), 0 );
  /// ```
  #[must_use] 
  pub fn new() -> Self
  {
    Self
    {
      inner: HashMap ::new(),
    }
  }

  /// Inserts a value for a parameter, overwriting any existing value.
  ///
  /// # Parameters
  ///
  /// - `key`: Parameter name
  /// - `value`: Parameter value
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ Values, Value };
  ///
  /// let mut values = Values::new();
  /// values.insert( "name", Value::String( "test".into() ) );
  /// ```
  pub fn insert( &mut self, key: &str, value: V )
  {
    self.inner.insert( key.to_string(), Some( value ) );
  }

  /// Inserts a value only if the parameter has no value set.
  ///
  /// If the parameter already has a value (even if explicitly set to None),
  /// this method does nothing. This is useful for providing defaults without
  /// overwriting user-supplied values.
  ///
  /// # Parameters
  ///
  /// - `key`: Parameter name
  /// - `value`: Parameter value to insert if missing
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ Values, Value, TemplateValue };
  ///
  /// let mut values = Values::new();
  /// values.insert_if_empty( "region", Value::String( "us-east-1".into() ) );
  /// values.insert_if_empty( "region", Value::String( "eu-west-1".into() ) );
  ///
  /// // First value wins
  /// assert_eq!( values.get( "region" ).unwrap().to_template_string(), "us-east-1" );
  /// ```
  pub fn insert_if_empty( &mut self, key: &str, value: V )
  {
    // Only insert if key doesn't exist OR value is None
    let should_insert = match self.inner.get( key )
    {
      None | Some( None ) => true, // Key doesn't exist or value is None
      Some( Some( _ ) ) => false, // Key exists with Some value
    };

    if should_insert
    {
      self.inner.insert( key.to_string(), Some( value ) );
    }
  }

  /// Inserts an explicit None value for a parameter.
  ///
  /// Used to mark a parameter as present but unset, which is distinct from
  /// a parameter that was never mentioned.
  ///
  /// # Parameters
  ///
  /// - `key`: Parameter name
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ Values, Value };
  ///
  /// let mut values = Values::< Value >::new();
  /// values.insert_none( "optional_param" );
  ///
  /// assert!( !values.has_value( "optional_param" ) );
  /// assert!( values.needs_prompt( "optional_param" ) );
  /// ```
  pub fn insert_none( &mut self, key: &str )
  {
    self.inner.insert( key.to_string(), None );
  }

  /// Retrieves the value for a parameter.
  ///
  /// # Parameters
  ///
  /// - `key`: Parameter name
  ///
  /// # Returns
  ///
  /// `Some(&V)` if parameter has a value, `None` otherwise
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ Values, Value };
  ///
  /// let mut values = Values::new();
  /// values.insert( "test", Value::String( "value".into() ) );
  ///
  /// assert!( values.get( "test" ).is_some() );
  /// assert!( values.get( "missing" ).is_none() );
  /// ```
  #[must_use] 
  pub fn get( &self, key: &str ) -> Option< &V >
  {
    self.inner.get( key ).and_then( | opt | opt.as_ref() )
  }

  /// Checks if a parameter has a value set.
  ///
  /// Returns `true` only if the parameter exists AND has a non-None value.
  ///
  /// # Parameters
  ///
  /// - `key`: Parameter name
  ///
  /// # Returns
  ///
  /// `true` if value is present, `false` otherwise
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ Values, Value };
  ///
  /// let mut values = Values::new();
  /// values.insert( "set", Value::String( "yes".into() ) );
  /// values.insert_none( "not_set" );
  ///
  /// assert!( values.has_value( "set" ) );
  /// assert!( !values.has_value( "not_set" ) );
  /// ```
  #[must_use] 
  pub fn has_value( &self, key: &str ) -> bool
  {
    self.inner.get( key ).and_then( | opt | opt.as_ref() ).is_some()
  }

  /// Checks if a parameter needs interactive prompting.
  ///
  /// Returns `true` if the parameter exists but has no value (None).
  ///
  /// # Parameters
  ///
  /// - `key`: Parameter name
  ///
  /// # Returns
  ///
  /// `true` if prompt needed, `false` otherwise
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ Values, Value };
  ///
  /// let mut values: Values< Value > = Values::new();
  /// values.insert_none( "needs_input" );
  ///
  /// assert!( values.needs_prompt( "needs_input" ) );
  /// ```
  #[must_use]
  pub fn needs_prompt( &self, key: &str ) -> bool
  {
    matches!( self.inner.get( key ), Some( None ) )
  }

  /// Returns the number of parameters in the collection.
  ///
  /// Includes both parameters with values and parameters explicitly set to None.
  ///
  /// # Returns
  ///
  /// Count of parameters
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ Values, Value };
  ///
  /// let mut values = Values::new();
  /// values.insert( "a", Value::Number( 1 ) );
  /// values.insert( "b", Value::Number( 2 ) );
  ///
  /// assert_eq!( values.len(), 2 );
  /// ```
  #[must_use] 
  pub fn len( &self ) -> usize
  {
    self.inner.len()
  }

  /// Checks if the collection is empty.
  ///
  /// # Returns
  ///
  /// `true` if no parameters exist, `false` otherwise
  #[must_use] 
  pub fn is_empty( &self ) -> bool
  {
    self.inner.is_empty()
  }

  /// Converts all values to a serializable JSON map.
  ///
  /// Transforms the internal storage into a `BTreeMap<String, serde_json::Value>`
  /// suitable for template rendering. Preserves type information for proper
  /// template conditional logic.
  ///
  /// # Returns
  ///
  /// `BTreeMap` with all parameter names and their typed JSON representations
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ Values, Value };
  ///
  /// let mut values = Values::new();
  /// values.insert( "name", Value::String( "test".into() ) );
  /// values.insert( "count", Value::Number( 42 ) );
  /// values.insert( "enabled", Value::Bool( true ) );
  ///
  /// let serialized = values.to_serializable();
  /// // Types are preserved: strings as strings, numbers as numbers, bools as bools
  /// ```
  #[must_use] 
  pub fn to_serializable( &self ) -> BTreeMap< String, serde_json ::Value >
  where
    V: serde ::Serialize
  {
    self
      .inner
      .iter()
      .map( | ( key, value ) |
      {
        let json_value = match value
        {
          Some( v ) => serde_json ::to_value( v ).unwrap_or( serde_json ::Value ::Null ),
          None => serde_json ::Value ::String( "___UNSPECIFIED___".to_string() ),
        };
        ( key.clone(), json_value )
      })
      .collect()
  }
}

impl< V > Default for Values< V >
where
  V: TemplateValue
{
  fn default() -> Self
  {
    Self ::new()
  }
}
