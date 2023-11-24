mod private
{
  use std::collections::HashMap;
  use std::str::FromStr;

  /// enum for parsing string
  #[ derive( Debug, PartialEq ) ]
  pub enum Value
  {
    /// represent string value
    StringValue( String ),
    /// represent int value
    IntValue( i32 ),
    /// represent bool value
    BoolValue( bool ),
  }

  impl FromStr for Value
  {
    type Err = anyhow::Error;

    fn from_str( s: &str ) -> Result< Self, Self::Err >
    {
      if let Ok( int ) = s.parse::< i32 >()
      {
        Ok( Value::IntValue( int ) )
      }
      else if let Ok( boolean ) = s.parse::< bool >()
      {
        Ok( Value::BoolValue( boolean ) )
      }
      else
      {
        Ok( Value::StringValue( s.to_string() ) )
      }
    }
  }

  impl From< &Value > for bool
  {
    fn from( value: &Value ) -> Self
    {
      match value
      {
        Value::BoolValue( b ) => *b,
        Value::IntValue( i ) => i == &1,
        Value::StringValue( s ) => s.as_str() == "1",
      }
    }
  }

  /// parse string to HashMap< String, Value >
  pub fn string_parse( input: &str ) -> HashMap< String, Value >
  {
    let mut map = HashMap::new();

    for item in input.split( "," )
    {
      let parts: Vec< &str > = item.split( ":" ).collect();
      if parts.len() == 2
      {
        let key = parts[ 0 ].trim().to_string();
        let value = parts[ 1 ].trim().parse::< Value >().unwrap();
        map.insert( key, value );
      }
    }
    map
  }
}

crate::mod_interface!
{
  /// Bump version.
  protected use string_parse;
  protected use Value;
}
