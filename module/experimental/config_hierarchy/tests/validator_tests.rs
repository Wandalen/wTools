//! Tests for `ConfigValidator` trait contract (`api/003_config_validator_trait`)
//!
//! These tests CANNOT be faked — swapping the V type parameter produces
//! observably different behavior, proving the validator is actually consulted.

#![ allow( missing_docs ) ]

use config_hierarchy::
{
  ConfigDefaults, ConfigPaths, ConfigValidator, ConfigManager,
  ValidationError, ConfigSource,
};
use std::collections::HashMap;
use serde_json::Value as JsonValue;

/// Pass-through validator — equivalent to the documented `NoValidator` stub
struct NoOpValidator;
impl ConfigValidator for NoOpValidator
{
  fn validate_parameter( _param_name : &str, _value : &JsonValue )
    -> Result< (), ValidationError >
  {
    Ok( () )
  }

  fn validate_all( _config : &HashMap< String, ( JsonValue, ConfigSource ) > )
    -> Vec< ValidationError >
  {
    Vec::new()
  }
}

struct MinDefaults;
impl ConfigDefaults for MinDefaults
{
  fn get_defaults() -> HashMap< String, JsonValue > { HashMap::new() }
  fn get_parameter_names() -> Vec< &'static str > { vec![ "count", "timeout", "retries" ] }
}

struct MinPaths;
impl ConfigPaths for MinPaths
{
  fn app_name() -> &'static str { "valtest" }
}

/// Validator that rejects negative integer values
struct NegativeRejectValidator;
impl ConfigValidator for NegativeRejectValidator
{
  fn validate_parameter( param_name : &str, value : &JsonValue )
    -> Result< (), ValidationError >
  {
    if let Some( n ) = value.as_i64()
    {
      if n < 0
      {
        return Err( ValidationError::new( param_name, "must be non-negative" ) );
      }
    }
    Ok( () )
  }

  fn validate_all( _config : &HashMap< String, ( JsonValue, ConfigSource ) > )
    -> Vec< ValidationError >
  {
    Vec::new()
  }
}

/// Validator that enforces: timeout > 0 whenever retries > 0
struct CrossParamValidator;
impl ConfigValidator for CrossParamValidator
{
  fn validate_parameter( _param_name : &str, _value : &JsonValue )
    -> Result< (), ValidationError >
  {
    Ok( () )
  }

  fn validate_all( config : &HashMap< String, ( JsonValue, ConfigSource ) > )
    -> Vec< ValidationError >
  {
    let mut errors = Vec::new();
    let retries = config.get( "retries" ).and_then( | ( v, _ ) | v.as_i64() ).unwrap_or( 0 );
    let timeout = config.get( "timeout" ).and_then( | ( v, _ ) | v.as_i64() ).unwrap_or( 0 );
    if retries > 0 && timeout == 0
    {
      errors.push( ValidationError::new( "timeout", "must be > 0 when retries > 0" ) );
    }
    errors
  }
}

type NoValConfig      = ConfigManager< MinDefaults, MinPaths, NoOpValidator >;
type RejectNegConfig  = ConfigManager< MinDefaults, MinPaths, NegativeRejectValidator >;
type CrossParamConfig = ConfigManager< MinDefaults, MinPaths, CrossParamValidator >;

// AP-01: NoValidator accepts all values without error
#[ test ]
fn test_no_validator_accepts_all()
{
  // Negative value — would be rejected by a real validator
  let result = NoValConfig::validate_parameter( "count", &JsonValue::Number( ( -999 ).into() ) );
  assert!( result.is_ok(), "NoValidator must accept any value" );

  let result2 = NoValConfig::validate_parameter( "count", &JsonValue::Null );
  assert!( result2.is_ok(), "NoValidator must accept null" );

  let config : HashMap< String, ( JsonValue, ConfigSource ) > = HashMap::new();
  let errors = NoValConfig::validate_all_config( &config );
  assert!( errors.is_empty(), "NoValidator::validate_all must return empty Vec" );
}

// AP-02: validate_parameter() returns Err for invalid value
#[ test ]
fn test_validator_rejects_negative()
{
  let result = RejectNegConfig::validate_parameter( "count", &JsonValue::Number( ( -1 ).into() ) );

  assert!( result.is_err(), "Validator must reject negative values" );
  let err = result.unwrap_err();
  assert_eq!( err.parameter, "count", "Error must identify the offending parameter" );
  assert!( err.message.contains( "non-negative" ), "Error message must describe constraint" );
}

// AP-03: validate_parameter() returns Ok for valid value
#[ test ]
fn test_validator_accepts_positive()
{
  let result = RejectNegConfig::validate_parameter( "count", &JsonValue::Number( 5.into() ) );
  assert!( result.is_ok(), "Validator must accept positive values" );

  let result_zero = RejectNegConfig::validate_parameter( "count", &JsonValue::Number( 0.into() ) );
  assert!( result_zero.is_ok(), "Validator must accept zero" );
}

// AP-04: validate_all() detects cross-parameter constraint violation
#[ test ]
fn test_validate_all_cross_parameter_constraint()
{
  let mut config = HashMap::new();
  config.insert( "retries".into(), ( JsonValue::Number( 3.into() ), ConfigSource::Default ) );
  config.insert( "timeout".into(), ( JsonValue::Number( 0.into() ), ConfigSource::Default ) );

  let errors = CrossParamConfig::validate_all_config( &config );

  assert_eq!( errors.len(), 1, "Must detect exactly one cross-parameter violation" );
  assert_eq!( errors[ 0 ].parameter, "timeout" );
  assert!( errors[ 0 ].message.contains( "timeout" ) || errors[ 0 ].message.contains( "retries" ) );
}

// AP-05: validate_all() returns empty for valid config
#[ test ]
fn test_validate_all_valid_config()
{
  let mut config = HashMap::new();
  config.insert( "retries".into(), ( JsonValue::Number( 3.into() ), ConfigSource::Default ) );
  config.insert( "timeout".into(), ( JsonValue::Number( 30.into() ), ConfigSource::Default ) );

  let errors = CrossParamConfig::validate_all_config( &config );
  assert!( errors.is_empty(), "Valid config must produce zero validation errors" );
}

// AP-06: Validator type parameter governs behavior (cannot-be-faked)
//
// This test proves the V type parameter is actually consulted by ConfigManager.
// If ConfigManager hardcoded NoValidator behavior, both types would return Ok(())
// for negative values. The fact that swapping V produces different results proves
// the type parameter is read and dispatched at runtime.
#[ test ]
fn test_validator_type_param_governs_behavior()
{
  let value = &JsonValue::Number( ( -42 ).into() );

  let no_val_result    = NoValConfig::validate_parameter( "x", value );
  let reject_result    = RejectNegConfig::validate_parameter( "x", value );

  assert!( no_val_result.is_ok(),  "NoValidator must return Ok for negative" );
  assert!( reject_result.is_err(), "NegativeRejectValidator must return Err for negative" );

  // The behavioral difference between the two types proves V is consulted
  // (if V were ignored, both would return Ok)
}
