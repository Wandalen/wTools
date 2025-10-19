/// Tests for Values storage system (FR5, FR6, FR7)
use super :: *;

//

#[ test ]
fn values_stores_hashmap()
{
  // FR5: Must store HashMap of parameter names to Option<V> values
  let mut values = Values ::< Value >::new();

  // Initially empty
  assert_eq!( values.len(), 0 );

  // Can insert values
  values.insert( "key1", Value ::String( "value1".into() ) );
  values.insert( "key2", Value ::Number( 42 ) );

  assert_eq!( values.len(), 2 );
}

#[ test ]
fn values_insert_if_empty_only_when_none()
{
  // FR5: insert_if_empty(key, value) must only insert if key has None value
  let mut values = Values ::< Value >::new();

  // First insert should succeed
  values.insert_if_empty( "param1", Value ::String( "first".into() ) );
  assert_eq!( values.get( "param1" ).unwrap().to_template_string(), "first" );

  // Second insert with same key should NOT overwrite
  values.insert_if_empty( "param1", Value ::String( "second".into() ) );
  assert_eq!( values.get( "param1" ).unwrap().to_template_string(), "first" );

  // Insert with different key should succeed
  values.insert_if_empty( "param2", Value ::String( "other".into() ) );
  assert_eq!( values.get( "param2" ).unwrap().to_template_string(), "other" );
}

#[ test ]
fn values_insert_if_empty_with_explicit_none()
{
  // FR5: insert_if_empty should insert if key exists but value is None
  let mut values = Values ::< Value >::new();

  // Insert explicit None
  values.insert_none( "param1" );

  // insert_if_empty should succeed because value is None
  values.insert_if_empty( "param1", Value ::String( "value".into() ) );
  assert_eq!( values.get( "param1" ).unwrap().to_template_string(), "value" );
}

#[ test ]
fn values_to_serializable()
{
  // FR5: to_serializable() must convert all values to BTreeMap<String, serde_json::Value>
  let mut values = Values ::< Value >::new();

  values.insert( "name", Value ::String( "genfile".into() ) );
  values.insert( "count", Value ::Number( 42 ) );
  values.insert( "enabled", Value ::Bool( true ) );
  values.insert( "items", Value ::List( vec![ "a".into(), "b".into() ] ) );

  let serialized = values.to_serializable();

  // Should be BTreeMap (sorted) with preserved types
  assert_eq!( serialized.get( "name" ), Some( &serde_json ::Value ::String( "genfile".to_string() ) ) );
  assert_eq!( serialized.get( "count" ), Some( &serde_json ::Value ::Number( 42.into() ) ) );
  assert_eq!( serialized.get( "enabled" ), Some( &serde_json ::Value ::Bool( true ) ) );
  // List is serialized as comma-separated string for simple interpolation
  assert_eq!( serialized.get( "items" ), Some( &serde_json ::Value ::String( "a, b".to_string() ) ) );
}

#[ test ]
fn values_to_serializable_handles_none()
{
  // FR5: to_serializable should handle None values
  let mut values = Values ::< Value >::new();

  values.insert( "has_value", Value ::String( "test".into() ) );
  values.insert_none( "no_value" );

  let serialized = values.to_serializable();

  assert_eq!( serialized.get( "has_value" ), Some( &serde_json ::Value ::String( "test".to_string() ) ) );
  // None values should be represented as placeholder
  assert_eq!( serialized.get( "no_value" ), Some( &serde_json ::Value ::String( "___UNSPECIFIED___".to_string() ) ) );
}

#[ test ]
fn values_generic_over_template_value()
{
  // FR5: Must support generic V: TemplateValue type parameter

  // Custom value type
  #[ derive( Clone, serde ::Serialize ) ]
  struct CustomValue( String );

  impl TemplateValue for CustomValue
  {
    fn to_template_string( &self ) -> String
    {
      format!( "custom:{}", self.0 )
    }

    fn from_string( s: String ) -> Self
    {
      CustomValue( s )
    }

    fn is_empty( &self ) -> bool
    {
      self.0.is_empty()
    }
  }

  // Should work with custom value type
  let mut values = Values ::< CustomValue >::new();
  values.insert( "test", CustomValue( "data".into() ) );

  let serialized = values.to_serializable();
  assert_eq!( serialized.get( "test" ), Some( &serde_json ::Value ::String( "data".to_string() ) ) );
}

#[ test ]
fn values_get_returns_option()
{
  // Values should provide get() method
  let mut values = Values ::< Value >::new();

  values.insert( "exists", Value ::String( "yes".into() ) );

  assert!( values.get( "exists" ).is_some() );
  assert!( values.get( "not_exists" ).is_none() );
}

#[ test ]
fn values_has_value_checks_if_set()
{
  // Values should provide method to check if value is set
  let mut values = Values ::< Value >::new();

  values.insert( "set", Value ::String( "value".into() ) );
  values.insert_none( "not_set" );

  assert!( values.has_value( "set" ) );
  assert!( !values.has_value( "not_set" ) );
  assert!( !values.has_value( "never_added" ) );
}

// FR6: Interactive Prompting tests
// Note: Actual stdin interaction can't be tested in unit tests
// We test the logic but skip interactive parts

#[ test ]
fn values_skip_prompt_if_value_present()
{
  // FR6: Must skip prompting if value already present
  let mut values = Values ::< Value >::new();

  values.insert( "param", Value ::String( "existing".into() ) );

  // Should return false indicating no prompt needed
  assert!( !values.needs_prompt( "param" ) );
}

#[ test ]
fn values_needs_prompt_if_none()
{
  // FR6: Should indicate prompt needed if value is None
  let mut values = Values ::< Value >::new();

  values.insert_none( "param" );

  // Should return true indicating prompt needed
  assert!( values.needs_prompt( "param" ) );
}

// FR7: TOML Parameter Persistence tests

#[ test ]
fn values_can_be_created_empty()
{
  // Should be able to create empty Values
  let values = Values ::< Value >::new();
  assert_eq!( values.len(), 0 );
}

#[ test ]
fn values_default_creates_empty()
{
  // Should support Default trait
  let values = Values ::< Value >::default();
  assert_eq!( values.len(), 0 );
}
