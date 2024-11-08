/// Define a private namespace for all its items.
mod private
{
  #[ allow( unused_imports ) ]
  use crate::tool::*;

  use std::
  {
    str::FromStr,
  };
  use error::
  {
    untyped::{ Error, bail },
    // Result,
  };
  use collection::HashMap;

  #[ derive( Debug, PartialEq, Eq, Clone ) ]
  /// Parser value enum
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

    fn from_str( s : &str ) -> Result< Self, Self::Err >
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
    fn from( value : &Value ) -> Self
    {
      match value
      {
        Value::Bool( value ) => *value,
        Value::String( string ) => string == "true",
        Value::Int( i ) => *i == 1,
      }
    }
  }

  /// Represents the result of parsing.
  #[ derive( Debug, Clone ) ]
   pub enum ParseResult
  {
    /// Named parsing result.
    Named( HashMap< String, Value >),
    /// Positional parsing result.
    Positioning( Vec< Value >)
  }

  impl ParseResult
  {
    /// Converts the parsing result into a vector of values.
    /// ``` rust
    /// use std::collections::HashMap;
    /// use willbe::query::{ ParseResult, Value };
    ///
    /// let params = HashMap::from( [ ( "v1".to_string(), Value::Int( 1 ) ), ( "v2".to_string(), Value::Int( 2 ) ), ( "v3".to_string(), Value::Int( 3 ) ) ] );
    ///
    /// let result = ParseResult::Named( params ).into_vec();
    ///
    /// assert!( result.contains( &Value::Int( 1 ) ) );
    /// assert!( result.contains( &Value::Int( 2 ) ) );
    /// assert!( result.contains( &Value::Int( 3 ) ) );
    /// ```
    pub fn into_vec( self ) -> Vec< Value >
    {
      match self
      {
        ParseResult::Named( map ) => map.values().cloned().collect(),
        ParseResult::Positioning( vec ) => vec,
      }
    }

    /// Converts the parsing result into a hashmap, using a vector of names as keys.
    /// ```rust
    ///  use std::collections::HashMap;
    ///  use willbe::query::{ ParseResult, Value };
    ///
    ///  let params = vec![ Value::Int( 1 ), Value::Int( 2 ), Value::Int( 3 ) ];
    ///  let result = ParseResult::Positioning( params );
    ///
    ///  let named_map = result.clone().into_map( vec![ "var0".into(), "var1".into(),"var2".into() ] );
    ///  let unnamed_map = result.clone().into_map( vec![] );
    ///  let mixed_map = result.clone().into_map( vec![ "var0".into() ] );
    ///  let vec = result.into_vec();
    ///
    ///  assert_eq!( HashMap::from( [ ( "var0".to_string(), Value::Int( 1 ) ), ( "var1".to_string(),Value::Int( 2 ) ), ( "var2".to_string(),Value::Int( 3 ) ) ] ), named_map );
    ///  assert_eq!( HashMap::from( [ ( "1".to_string(), Value::Int( 1 ) ), ( "2".to_string(),Value::Int( 2 ) ), ( "3".to_string(),Value::Int( 3 ) ) ] ), unnamed_map );
    ///  assert_eq!( HashMap::from( [ ( "var0".to_string(), Value::Int( 1 ) ), ( "1".to_string(),Value::Int( 2 ) ), ( "2".to_string(),Value::Int( 3 ) ) ] ), mixed_map );
    /// ```
    pub fn into_map( self, names : Vec< String > ) -> HashMap< String, Value >
    {
      match self
      {
        ParseResult::Named( map ) => map,
        ParseResult::Positioning( vec ) =>
        {
          let mut map = HashMap::new();
          let mut counter = 0;
          for ( index, value ) in vec.into_iter().enumerate() {
            map.insert
            (
              names.get( index ).cloned().unwrap_or_else( || { counter+=1; counter.to_string() } ),
              value
            );
          }
          map
        }
      }
    }
  }

  /// Parses an input string and returns a parsing result.
  /// ```rust
  /// use willbe::query::{ parse, Value };
  /// use std::collections::HashMap;
  ///
  /// assert_eq!( parse( "()" ).unwrap().into_vec(), vec![] );
  ///
  /// let mut expected_map = HashMap::new();
  /// expected_map.insert( "1".to_string(), Value::String( "test/test".to_string() ) );
  /// assert_eq!( parse( "('test/test')" ).unwrap().into_map( vec![] ), expected_map );
  ///
  /// let mut expected_map = HashMap::new();
  /// expected_map.insert( "key".to_string(), Value::String( r#"hello\'test\'test"#.into() ) );
  /// assert_eq!( parse( r#"{ key : 'hello\'test\'test' }"# ).unwrap().into_map( vec![] ), expected_map );
  /// ```
  // qqq : use typed error
  pub fn parse( input_string : &str ) -> error::untyped::Result< ParseResult >
  {
    if input_string.len() < 2
    {
      bail!( "Input length should be two or more" )
    }
    if input_string.len() == 2
    {
      return Ok( ParseResult::Positioning( vec![] ) )
    }
    let start = input_string.chars().next().unwrap();
    let input_string = &input_string[1..input_string.len()-1];
    let params = split_string( input_string );
    let result = match start
    {
      '{' =>
      {
        ParseResult::Named( parse_to_map( params )? )
      },
      '(' =>
      {
        ParseResult::Positioning( parse_to_vec( params )? )
      },
      _ => bail!( "Invalid start character" )
    };

    Ok( result )
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

  // qqq : use typed error
  fn parse_to_map(input : Vec< String > ) -> error::untyped::Result< HashMap< String, Value > >
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

  // qqq : use typed error
  fn parse_to_vec( input : Vec< String > ) -> error::untyped::Result< Vec< Value > >
  {
    Ok( input.into_iter().filter_map( | w | Value::from_str( w.trim() ).ok() ).collect() )
  }
}

crate::mod_interface!
{
  own use parse;
  own use Value;
  own use ParseResult;
}
