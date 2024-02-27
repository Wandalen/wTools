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

  #[ derive( Debug, PartialEq, Eq, Clone ) ]
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

  ///todo
  #[ derive( Debug, Clone ) ]
   pub enum ParseResult
  {
    ///todo
    Named( HashMap< String, Value >),
    ///todo
    Positioning( Vec< Value >)
  }

  impl ParseResult
  {
    ///todo
    pub fn into_vec( self ) -> Vec< Value >
    {
      match self
      {
        ParseResult::Named( map ) => map.values().cloned().collect(),
        ParseResult::Positioning( vec ) => vec,
      }
    }

    ///todo
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
    
  ///todo
  pub fn parse( input_string : &str ) -> Result< ParseResult >
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
  
  fn parse_to_vec( input: Vec< String > ) -> Result< Vec< Value > >
  {
    Ok( input.into_iter().filter_map( | w | Value::from_str( w.trim() ).ok() ).collect() )
  }
}

crate::mod_interface!
{
  /// Bump version.
  protected use parse;
  protected use Value;
  protected use ParseResult;
}
