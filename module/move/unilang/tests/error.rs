//!
//! Tests for the error module
//!

use unilang::error::Error;
use unilang::data::ErrorData;

#[test]
fn test_error_execution_display()
{
  let error_data = ErrorData::new(
    "TEST_ERROR".to_string(),
    "This is a test error message".to_string(),
  );
  let error = Error::Execution(error_data);
  
  let error_string = error.to_string();
  assert!(error_string.contains("Execution Error"));
  assert!(error_string.contains("This is a test error message"));
}

#[test]
fn test_error_registration_display()
{
  let error = Error::Registration("Failed to register command".to_string());
  let error_string = error.to_string();
  assert!(error_string.contains("Registration Error"));
  assert!(error_string.contains("Failed to register command"));
}

#[test]
fn test_error_yaml_display()
{
  let yaml_error = serde_yaml::from_str::<serde_yaml::Value>("invalid: yaml: {").unwrap_err();
  let error = Error::Yaml(yaml_error);
  let error_string = error.to_string();
  assert!(error_string.contains("YAML Deserialization Error"));
}

#[test]
fn test_error_json_display()
{
  let json_error = serde_json::from_str::<serde_json::Value>("{invalid json").unwrap_err();
  let error = Error::Json(json_error);
  let error_string = error.to_string();
  assert!(error_string.contains("JSON Deserialization Error"));
}

#[test]
fn test_error_parse_display()
{
  let parse_error = unilang_parser::error::ParseError::new(
    unilang_parser::error::ErrorKind::Syntax("test parse error".to_string()),
    unilang_parser::SourceLocation::StrSpan { start: 0, end: 5 }
  );
  let error = Error::Parse(parse_error);
  let error_string = error.to_string();
  assert!(error_string.contains("Parse Error"));
  assert!(error_string.contains("test parse error"));
}

#[test]
fn test_type_error_conversion()
{
  let type_error = unilang::types::TypeError {
    expected_kind: unilang::data::Kind::Integer,
    reason: "Invalid integer format".to_string(),
  };
  
  let error: Error = type_error.into();
  
  if let Error::Execution(error_data) = error {
    assert_eq!(error_data.code, "UNILANG_TYPE_MISMATCH");
    assert!(error_data.message.contains("Type Error: Invalid integer format"));
    assert!(error_data.message.contains("Please provide a valid value for this type"));
  } else {
    panic!("Expected Execution error");
  }
}

#[test]
fn test_error_data_conversion()
{
  let error_data = ErrorData::new(
    "CUSTOM_ERROR".to_string(),
    "Custom error message".to_string(),
  );
  
  let error: Error = error_data.into();
  
  if let Error::Execution(data) = error {
    assert_eq!(data.code, "CUSTOM_ERROR");
    assert_eq!(data.message, "Custom error message");
  } else {
    panic!("Expected Execution error");
  }
}

#[test]
fn test_yaml_error_from_conversion()
{
  let yaml_error = serde_yaml::from_str::<serde_yaml::Value>("invalid: yaml: content: {").unwrap_err();
  let error: Error = yaml_error.into();
  
  assert!(matches!(error, Error::Yaml(_)));
}

#[test]
fn test_json_error_from_conversion()
{
  let json_error = serde_json::from_str::<serde_json::Value>("{malformed json").unwrap_err();
  let error: Error = json_error.into();
  
  assert!(matches!(error, Error::Json(_)));
}

#[test]
fn test_parse_error_from_conversion()
{
  let parse_error = unilang_parser::error::ParseError::new(
    unilang_parser::error::ErrorKind::Syntax("parsing failed".to_string()),
    unilang_parser::SourceLocation::StrSpan { start: 0, end: 3 }
  );
  let error: Error = parse_error.into();
  
  assert!(matches!(error, Error::Parse(_)));
}

#[test]
fn test_error_debug_format()
{
  let error_data = ErrorData::new(
    "DEBUG_ERROR".to_string(),
    "Debug error message".to_string(),
  );
  let error = Error::Execution(error_data);
  
  let debug_string = format!("{error:?}");
  assert!(debug_string.contains("Execution"));
  assert!(debug_string.contains("DEBUG_ERROR"));
}

#[test]
fn test_multiple_error_types()
{
  let execution_error = Error::Execution(ErrorData::new(
    "EXEC_ERROR".to_string(),
    "Execution failed".to_string(),
  ));
  
  let registration_error = Error::Registration("Registration failed".to_string());
  
  // Test that different error types display differently
  assert!(execution_error.to_string().contains("Execution Error"));
  assert!(registration_error.to_string().contains("Registration Error"));
  assert!(!execution_error.to_string().contains("Registration"));
  assert!(!registration_error.to_string().contains("Execution"));
}