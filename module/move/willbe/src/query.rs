mod private
{
  use crate::*;

  use std::
  {
    str::FromStr,
    collections::HashMap
  };
  use wtools::error::{ for_app::{ Error }, Result };
  
  #[ derive( Debug, PartialEq, Eq ) ]
  /// Parser result enum
  pub enum Value 
  {
    /// string value
    String( String ),
    /// int value
    Int( i32 ),
    /// bool value
    Bool( bool ),
  }

  impl FromStr for Value 
  {
    type Err = Error;

    fn from_str( s: &str ) -> Result< Self, Self::Err > 
    {
      if let Ok( i ) = s.parse::< i32 >() 
      {
        Ok( Value::Int( i ) )
      } else if let Ok( b ) = s.parse::< bool >() 
      {
        Ok( Value::Bool( b ) )
      } else 
      {
        let s = s.trim_matches( '\'' );
        Ok( Value::String( s.to_string() ) )
      }
    }
  }

  impl From< &Value > for bool
  {
    fn from( value: &Value ) -> Self 
    {
      match value 
      {
        Value::Bool( value ) => *value,
        Value::String( string ) => string == "true",
        Value::Int( i ) => *i == 1,
      }
    }
  }

  /// The `parse` function parses an input string into a `HashMap` where the keys are `String` and the values are of type `Value`.
  ///
  /// # Arguments
  ///
  /// * `input_string`: A reference to a `str` that represents the input string to be parsed.
  ///
  /// # Returns
  ///
  /// This function returns a `Result` that contains a `HashMap<String, Value>` if the input string is successfully parsed, or error message if the input string cannot be parsed.
  ///
  /// # Edge Cases
  ///
  /// * If the input string is empty or contains only whitespace characters, the function returns an empty `HashMap`.
  /// ```rust
  /// use willbe::query::parse;
  /// use std::collections::HashMap;
  /// 
  /// let expected_map = HashMap::new();
  /// assert_eq!( parse( "" ).unwrap(), expected_map );
  /// ```
  /// * If the input string contains a single value enclosed in single quotes, the function returns a `HashMap` with a single entry where the key is `"path"` and the value is the input string.
  /// ```rust
  /// use willbe::query::{ parse, Value };
  /// use std::collections::HashMap;
  /// 
  /// let mut expected_map = HashMap::new();
  /// expected_map.insert( "0".to_string(), Value::String( "test/test".to_string() ) );
  /// assert_eq!( parse( "'test/test'" ).unwrap(), expected_map );
  /// ```
  /// * All values inside "'" are considered to be a string and can have any characters inside them, to escape "'" use "\'".
  /// ``` rust
  /// use willbe::query::{ parse, Value };
  /// use std::collections::HashMap;
  /// 
  /// let mut expected_map = HashMap::new();
  /// expected_map.insert( "key".to_string(), Value::String( r#"hello\'test\'test"#.into() ) );
  /// assert_eq!( parse( r#"key: 'hello\'test\'test'"# ).unwrap(), expected_map );
  /// 
  /// let mut expected_map = HashMap::new();
  /// expected_map.insert( "key".to_string(), Value::String( "test     ".into() ) );
  /// expected_map.insert( "key2".to_string(), Value::String( "test".into() ) );
  /// assert_eq!( parse( r#"key    :    'test     ', key2  :      test     "# ).unwrap(), expected_map ); 
  /// ```
  ///  
  
  pub fn parse( input_string: &str ) -> Result< HashMap< String, Value > >
  {
    let input_string = input_string.trim();
    let mut map = HashMap::new();
    if input_string.is_empty()
    {
      return Ok( map );
    }
    let mut start = 0;
    let mut in_quotes = false;
    let mut escaped = false;
    let mut has_named_values = false;
    
    let mut counter = 0;
    for ( i, c ) in input_string.char_indices()
    {
      match c
      {
        '\\' => if in_quotes { escaped = !escaped }
        ',' if !in_quotes =>
        {
          let item = &input_string[ start..i ];
          let parts = item.splitn( 2, ':' ).map( | s | s.trim() ).collect::< Vec< _ > >();
          if parts.len() == 2
          {
            if let Ok( value ) = parts[ 1 ].trim_matches( '\'' ).parse()
            {
              map.insert( parts[ 0 ].to_string(), value );
              has_named_values = true;
            }
          }
          else if parts.len() == 1
          {
            if let Ok( value ) = parts[ 0 ].trim_matches( '\'' ).parse::< Value >()
            {
              map.insert( counter.to_string(), value );
              counter+=1;
            }
          }
          start = i + 1;
        }
        '\'' => if !escaped { in_quotes = !in_quotes } else { escaped = false }
        _ => escaped = false,
      }
    }

    let item = &input_string[ start.. ];
    let parts = item.splitn( 2, ':' ).map( | s | s.trim() ).collect::< Vec< _ > >();
    if parts.len() == 2
    {
      if let Ok( value ) = parts[ 1 ].trim_matches( '\'' ).parse()
      {
        map.insert( parts[ 0 ].to_string(), value );
      }
    }
    else if parts.len() == 1
    {
      if let Ok( value ) = parts[ 0 ].trim_matches( '\'' ).parse::< Value >()
      {
        map.insert( counter.to_string(), value );
      }
    }

    Ok( map )
  }
}

crate::mod_interface!
{
  /// Bump version.
  protected use parse;
  protected use Value;
}
