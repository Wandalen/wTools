mod private
{
  use std::collections::HashMap;
  use std::str::FromStr;

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
    type Err = error_tools::for_app::Error;

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

  /// parse string to HashMap< String, Value >
  pub fn parse( input_string: &str ) -> HashMap< String, Value > 
  {
    let input_string = input_string.trim();
    let mut map = HashMap::new();
    if input_string.is_empty() 
    {
      return map;
    }
    let mut start = 0;
    let mut in_quotes = false;
    let mut escaped = false;

    for ( i, c ) in input_string.chars().enumerate() 
    {
      match c 
      {
        '\\' => 
        {
          if in_quotes 
          {
            escaped = !escaped;
          }
        }
        ',' if !in_quotes => 
        {
          let item = &input_string[ start..i ];
          let parts = item.splitn( 2, ':' ).map( | s | s.trim() ).collect::< Vec< &str > >();
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
              map.insert( "path".to_string(), value );
            }
          }
          start = i + 1;
        }
        '\'' => 
        {
          if !escaped 
          {
            in_quotes = !in_quotes;
          } 
          else 
          {
            escaped = false;
          }
        }
        _ => 
        {
          escaped = false;
        }
      }
    }

    let item = &input_string[ start.. ];
    let parts: Vec< &str > = item.splitn( 2, ':' ).map( | s | s.trim() ).collect();
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
        map.insert( "path".to_string(), value );
      }
    }
    map
  }
}

crate::mod_interface!
{
  /// Bump version.
  protected use parse;
  protected use Value;
}
