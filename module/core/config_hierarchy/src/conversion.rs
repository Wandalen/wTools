use serde_json::Value as JsonValue;

/// Convert YAML value to JSON value
///
/// Only available with `file_ops` feature
#[ cfg( feature = "file_ops" ) ]
#[ inline ]
pub fn yaml_to_json( yaml : serde_yaml::Value ) -> JsonValue
{
  match yaml
  {
    serde_yaml::Value::Bool( b ) => JsonValue::Bool( b ),
    serde_yaml::Value::Number( n ) =>
    {
      if let Some( i ) = n.as_i64()
      {
        JsonValue::Number( i.into() )
      }
      else if let Some( f ) = n.as_f64()
      {
        JsonValue::Number( serde_json::Number::from_f64( f ).unwrap_or( serde_json::Number::from( 0 ) ) )
      }
      else
      {
        JsonValue::Null
      }
    },
    serde_yaml::Value::String( s ) => JsonValue::String( s ),
    serde_yaml::Value::Sequence( seq ) =>
    {
      JsonValue::Array( seq.into_iter().map( yaml_to_json ).collect() )
    },
    serde_yaml::Value::Mapping( map ) =>
    {
      let mut obj = serde_json::Map::new();
      for ( k, v ) in map
      {
        if let serde_yaml::Value::String( key_str ) = k
        {
          obj.insert( key_str, yaml_to_json( v ) );
        }
      }
      JsonValue::Object( obj )
    },
    serde_yaml::Value::Null | serde_yaml::Value::Tagged( _ ) => JsonValue::Null,
  }
}

/// Convert JSON value to YAML value
///
/// Only available with `file_ops` feature
#[ cfg( feature = "file_ops" ) ]
#[ inline ]
pub fn json_to_yaml( json : JsonValue ) -> serde_yaml::Value
{
  match json
  {
    JsonValue::Null => serde_yaml::Value::Null,
    JsonValue::Bool( b ) => serde_yaml::Value::Bool( b ),
    JsonValue::Number( n ) =>
    {
      if let Some( i ) = n.as_i64()
      {
        serde_yaml::Value::Number( i.into() )
      }
      else if let Some( f ) = n.as_f64()
      {
        serde_yaml::Value::Number( serde_yaml::Number::from( f ) )
      }
      else
      {
        serde_yaml::Value::Null
      }
    },
    JsonValue::String( s ) => serde_yaml::Value::String( s ),
    JsonValue::Array( arr ) =>
    {
      serde_yaml::Value::Sequence( arr.into_iter().map( json_to_yaml ).collect() )
    },
    JsonValue::Object( obj ) =>
    {
      let mut map = serde_yaml::Mapping::new();
      for ( k, v ) in obj
      {
        map.insert( serde_yaml::Value::String( k ), json_to_yaml( v ) );
      }
      serde_yaml::Value::Mapping( map )
    },
  }
}

/// Extract clean string value from `JsonValue`
#[ inline ]
#[ must_use ]
pub fn json_value_to_display_string( value : &JsonValue ) -> String
{
  match value
  {
    JsonValue::String( s ) => s.clone(),
    JsonValue::Number( n ) => n.to_string(),
    JsonValue::Bool( b ) => b.to_string(),
    JsonValue::Null => "null".to_string(),
    JsonValue::Array( _ ) | JsonValue::Object( _ ) => value.to_string(),
  }
}
