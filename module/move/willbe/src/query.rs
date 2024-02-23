mod private
{
  use crate::*;

  use std::
  {
    str::FromStr,
    collections::HashMap
  };
  use error_tools::for_app::bail;
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
  
  pub fn parse( input_string : &str ) -> Result< HashMap< String, Value > >
  {
    todo!()
  }

  fn split_string( input : &str ) -> Vec< String >
  {
    let mut result = Vec::new();
    let mut start = 0;
    let mut in_quotes = false;
    for ( i, c ) in input.char_indices()
    {
      match c
      {
        '"' | '\'' => in_quotes = !in_quotes,
        ',' if !in_quotes =>
        {
          result.push( input[ start..i ].trim().to_string() );
          start = i + 1;
        }
        _ => {}
      }
    }
    result.push( input[ start.. ].trim().to_string() );
    result
  }

  fn parse_to_map(input: Vec<String> ) -> Result< HashMap< String, Value > > 
  {
    let mut map = HashMap::new();
    for line in input 
    {
      let mut in_quotes = false;
      let mut key = String::new();
      let mut value = String::new();
      let mut is_key = true;
      for c in line.chars() 
      {
        match c 
        {
          '"' | '\'' => 
          {
            in_quotes = !in_quotes;
            if is_key 
            {
              key.push( c );
            } 
            else 
            {
              value.push( c );
            }
          }
          ':' if !in_quotes => 
          {
            is_key = false;
          }
          _ => 
          {
            if is_key 
            {
              key.push( c );
            } 
            else 
            {
              value.push( c );
            }
          }
        }
      }
      if value.trim().is_empty() 
      {
        bail!( "Value is missing" )
      }
      map.insert( key.trim().to_string(), Value::from_str( value.trim() )? );
    }
    Ok( map )
  }
  
  fn parse_to_vec( input: Vec< String >) -> 
}

crate::mod_interface!
{
  /// Bump version.
  protected use parse;
  protected use Value;
}
