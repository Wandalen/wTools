//!
//! Tests for Phase 2 validated newtypes (CommandName, Namespace, Version, CommandStatus).
//!
//! # Test Coverage
//! - CommandName: construction, validation, serde, accessors
//! - NamespaceType: empty namespace, dot prefix validation, serde
//! - VersionType: non-empty validation, serde
//! - CommandStatus: enum variants, deprecation metadata, serde, backward compatibility
//!
//! # Phase 2 Context
//! These tests verify that the type-safe redesign correctly enforces
//! invariants at the type level, making invalid states unrepresentable.

use unilang::data::{ CommandName, NamespaceType, VersionType, CommandStatus };

//
// CommandName Tests
//

#[ test ]
fn command_name_valid_construction()
{
  // Valid command names
  let names = vec!
  [
    ".build",
    ".test",
    ".integration.test",
    ".a.b.c",
    ".help",
  ];

  for name in names
  {
    let result = CommandName::new( name );
    assert!(
      result.is_ok(),
      "CommandName::new({:?}) should succeed",
      name
    );

    let cmd_name = result.unwrap();
    assert_eq!(
      cmd_name.as_str(),
      name,
      "as_str() should return the original name"
    );
  }
}

#[ test ]
fn command_name_rejects_empty_name()
{
  let result = CommandName::new( "" );

  assert!(
    result.is_err(),
    "CommandName::new(\"\") should fail with EmptyCommandName error"
  );

  let err = result.unwrap_err();
  let err_msg = err.to_string();

  assert!(
    err_msg.contains( "empty" ) || err_msg.contains( "cannot be empty" ),
    "Error message should mention 'empty': {}",
    err_msg
  );
}

#[ test ]
fn command_name_rejects_missing_dot_prefix()
{
  let invalid_names = vec!
  [
    "build",
    "test",
    "integration.test",
    "a.b.c",
  ];

  for name in invalid_names
  {
    let result = CommandName::new( name );

    assert!(
      result.is_err(),
      "CommandName::new({:?}) should fail - missing dot prefix",
      name
    );

    let err = result.unwrap_err();
    let err_msg = err.to_string();

    assert!(
      err_msg.contains( "dot prefix" ) || err_msg.contains( "start with" ),
      "Error message should mention 'dot prefix' for {:?}: {}",
      name,
      err_msg
    );

    assert!(
      err_msg.contains( name ),
      "Error message should include the invalid name {:?}: {}",
      name,
      err_msg
    );
  }
}

#[ test ]
fn command_name_display_trait()
{
  let name = CommandName::new( ".build" ).unwrap();
  let display_str = format!( "{}", name );

  assert_eq!(
    display_str,
    ".build",
    "Display trait should show the command name"
  );
}

#[ test ]
fn command_name_debug_trait()
{
  let name = CommandName::new( ".test" ).unwrap();
  let debug_str = format!( "{:?}", name );

  assert!(
    debug_str.contains( ".test" ),
    "Debug trait should include the command name: {}",
    debug_str
  );
}

#[ test ]
fn command_name_accessors()
{
  let name_str = ".integration";
  let name = CommandName::new( name_str ).unwrap();

  // Test as_str()
  assert_eq!(
    name.as_str(),
    name_str,
    "as_str() should return the name as &str"
  );

  // Test into_inner()
  let inner = name.into_inner();
  assert_eq!(
    inner,
    name_str,
    "into_inner() should return the owned String"
  );
}

#[ test ]
fn command_name_clone_and_equality()
{
  let name1 = CommandName::new( ".build" ).unwrap();
  let name2 = name1.clone();

  assert_eq!(
    name1,
    name2,
    "Cloned CommandName should equal the original"
  );

  let name3 = CommandName::new( ".test" ).unwrap();
  assert_ne!(
    name1,
    name3,
    "Different CommandNames should not be equal"
  );
}

//
// Serde Tests
//

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn command_name_serde_json_serialize()
{
  let name = CommandName::new( ".build" ).unwrap();
  let json = serde_json::to_string( &name ).expect( "serialization should succeed" );

  assert_eq!(
    json,
    "\".build\"",
    "CommandName should serialize as a JSON string"
  );
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn command_name_serde_json_deserialize_valid()
{
  let json = "\".build\"";
  let name : CommandName = serde_json::from_str( json )
    .expect( "deserialization should succeed for valid name" );

  assert_eq!(
    name.as_str(),
    ".build",
    "Deserialized CommandName should have correct value"
  );
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn command_name_serde_json_deserialize_rejects_invalid()
{
  // Test empty name
  let json_empty = "\"\"";
  let result : Result< CommandName, _ > = serde_json::from_str( json_empty );
  assert!(
    result.is_err(),
    "Deserialization should fail for empty name"
  );

  // Test missing dot prefix
  let json_no_prefix = "\"build\"";
  let result : Result< CommandName, _ > = serde_json::from_str( json_no_prefix );
  assert!(
    result.is_err(),
    "Deserialization should fail for name without dot prefix"
  );
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn command_name_serde_json_roundtrip()
{
  let original = CommandName::new( ".integration.test" ).unwrap();

  // Serialize
  let json = serde_json::to_string( &original )
    .expect( "serialization should succeed" );

  // Deserialize
  let deserialized : CommandName = serde_json::from_str( &json )
    .expect( "deserialization should succeed" );

  assert_eq!(
    original,
    deserialized,
    "Roundtrip serialization should preserve equality"
  );

  assert_eq!(
    original.as_str(),
    deserialized.as_str(),
    "Roundtrip serialization should preserve value"
  );
}

#[ cfg( feature = "yaml_parser" ) ]
#[ test ]
fn command_name_serde_yaml_deserialize_valid()
{
  let yaml = ".build";
  let name : CommandName = serde_yaml::from_str( yaml )
    .expect( "YAML deserialization should succeed for valid name" );

  assert_eq!(
    name.as_str(),
    ".build",
    "Deserialized CommandName from YAML should have correct value"
  );
}

#[ cfg( feature = "yaml_parser" ) ]
#[ test ]
fn command_name_serde_yaml_deserialize_rejects_invalid()
{
  // Test empty name
  let yaml_empty = "\"\"";
  let result : Result< CommandName, _ > = serde_yaml::from_str( yaml_empty );
  assert!(
    result.is_err(),
    "YAML deserialization should fail for empty name"
  );

  // Test missing dot prefix
  let yaml_no_prefix = "build";
  let result : Result< CommandName, _ > = serde_yaml::from_str( yaml_no_prefix );
  assert!(
    result.is_err(),
    "YAML deserialization should fail for name without dot prefix"
  );
}

//
// Edge Cases
//

#[ test ]
fn command_name_with_special_characters()
{
  // Command names with special characters (after dot prefix)
  let names = vec!
  [
    ".test-command",
    ".test_command",
    ".test.sub-command",
  ];

  for name in names
  {
    let result = CommandName::new( name );
    assert!(
      result.is_ok(),
      "CommandName::new({:?}) should succeed - special chars are allowed",
      name
    );
  }
}

#[ test ]
fn command_name_long_names()
{
  // Test very long command names
  let long_name = format!( ".{}", "a".repeat( 100 ) );
  let result = CommandName::new( &long_name );

  assert!(
    result.is_ok(),
    "CommandName should accept long names"
  );

  assert_eq!(
    result.unwrap().as_str(),
    long_name.as_str(),
    "Long name should be preserved exactly"
  );
}

#[ test ]
fn command_name_single_char_after_dot()
{
  let name = ".a";
  let result = CommandName::new( name );

  assert!(
    result.is_ok(),
    "CommandName::new(\".a\") should succeed - single char is valid"
  );
}

#[ test ]
fn command_name_multiple_dots()
{
  let names = vec!
  [
    "..",
    "...",
    ".a..b",
    ".a.b.c.d.e",
  ];

  for name in names
  {
    let result = CommandName::new( name );
    assert!(
      result.is_ok(),
      "CommandName::new({:?}) should succeed - multiple dots are allowed",
      name
    );
  }
}

//
// NamespaceType Tests
//

#[ test ]
fn namespace_valid_empty()
{
  // Empty namespace is allowed (root-level commands)
  let empty = NamespaceType::new( "" );
  
  assert!(
    empty.is_ok(),
    "NamespaceType::new(\"\") should succeed - empty namespace is valid"
  );

  let ns = empty.unwrap();
  assert_eq!(
    ns.as_str(),
    "",
    "Empty namespace should have empty string value"
  );

  assert!(
    ns.is_root(),
    "Empty namespace should be identified as root"
  );
}

#[ test ]
fn namespace_valid_with_dot_prefix()
{
  // Valid namespaces with dot prefix
  let namespaces = vec!
  [
    ".video",
    ".git",
    ".config",
    ".integration.test",
  ];

  for ns_str in namespaces
  {
    let result = NamespaceType::new( ns_str );
    assert!(
      result.is_ok(),
      "NamespaceType::new({:?}) should succeed",
      ns_str
    );

    let ns = result.unwrap();
    assert_eq!(
      ns.as_str(),
      ns_str,
      "Namespace should preserve original value"
    );

    assert!(
      !ns.is_root(),
      "Non-empty namespace should not be root"
    );
  }
}

#[ test ]
fn namespace_rejects_missing_dot_prefix()
{
  // Invalid: non-empty without dot prefix
  let invalid_namespaces = vec!
  [
    "video",
    "git",
    "config",
  ];

  for ns_str in invalid_namespaces
  {
    let result = NamespaceType::new( ns_str );

    assert!(
      result.is_err(),
      "NamespaceType::new({:?}) should fail - missing dot prefix",
      ns_str
    );

    let err = result.unwrap_err();
    let err_msg = err.to_string();

    assert!(
      err_msg.contains( ns_str ),
      "Error message should include invalid namespace {:?}: {}",
      ns_str,
      err_msg
    );
  }
}

#[ test ]
fn namespace_display_trait()
{
  let empty = NamespaceType::new( "" ).unwrap();
  assert_eq!( format!( "{}", empty ), "" );

  let ns = NamespaceType::new( ".video" ).unwrap();
  assert_eq!( format!( "{}", ns ), ".video" );
}

#[ test ]
fn namespace_accessors()
{
  let ns_str = ".video";
  let ns = NamespaceType::new( ns_str ).unwrap();

  // Test as_str()
  assert_eq!( ns.as_str(), ns_str );

  // Test into_inner()
  let inner = ns.into_inner();
  assert_eq!( inner, ns_str );
}

#[ test ]
fn namespace_clone_and_equality()
{
  let ns1 = NamespaceType::new( ".video" ).unwrap();
  let ns2 = ns1.clone();

  assert_eq!( ns1, ns2, "Cloned namespace should equal original" );

  let ns3 = NamespaceType::new( ".git" ).unwrap();
  assert_ne!( ns1, ns3, "Different namespaces should not be equal" );
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn namespace_serde_json_serialize()
{
  let ns = NamespaceType::new( ".video" ).unwrap();
  let json = serde_json::to_string( &ns ).expect( "serialization should succeed" );

  assert_eq!( json, "\".video\"" );
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn namespace_serde_json_deserialize_valid()
{
  // Test valid namespace
  let json = "\".video\"";
  let ns : NamespaceType = serde_json::from_str( json )
    .expect( "deserialization should succeed" );
  assert_eq!( ns.as_str(), ".video" );

  // Test empty namespace
  let json_empty = "\"\"";
  let empty : NamespaceType = serde_json::from_str( json_empty )
    .expect( "deserialization of empty namespace should succeed" );
  assert_eq!( empty.as_str(), "" );
  assert!( empty.is_root() );
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn namespace_serde_json_deserialize_rejects_invalid()
{
  // Test missing dot prefix
  let json_invalid = "\"video\"";
  let result : Result< NamespaceType, _ > = serde_json::from_str( json_invalid );
  assert!(
    result.is_err(),
    "Deserialization should fail for namespace without dot prefix"
  );
}

#[ cfg( feature = "yaml_parser" ) ]
#[ test ]
fn namespace_serde_yaml_deserialize_valid()
{
  let yaml = ".video";
  let ns : NamespaceType = serde_yaml::from_str( yaml )
    .expect( "YAML deserialization should succeed" );
  assert_eq!( ns.as_str(), ".video" );

  // Test empty namespace
  let yaml_empty = "\"\"";
  let empty : NamespaceType = serde_yaml::from_str( yaml_empty )
    .expect( "YAML deserialization of empty namespace should succeed" );
  assert!( empty.is_root() );
}

//
// VersionType Tests
//

#[ test ]
fn version_valid_construction()
{
  // Valid version strings
  let versions = vec!
  [
    "1.0.0",
    "2.1",
    "0.1.0-alpha",
    "1.2.3+build.456",
    "v1.0",
  ];

  for ver_str in versions
  {
    let result = VersionType::new( ver_str );
    assert!(
      result.is_ok(),
      "VersionType::new({:?}) should succeed",
      ver_str
    );

    let ver = result.unwrap();
    assert_eq!(
      ver.as_str(),
      ver_str,
      "Version should preserve original value"
    );
  }
}

#[ test ]
fn version_rejects_empty_string()
{
  let result = VersionType::new( "" );

  assert!(
    result.is_err(),
    "VersionType::new(\"\") should fail - version cannot be empty"
  );

  let err = result.unwrap_err();
  let err_msg = err.to_string();

  assert!(
    err_msg.contains( "empty" ),
    "Error message should mention 'empty': {}",
    err_msg
  );
}

#[ test ]
fn version_display_trait()
{
  let ver = VersionType::new( "1.0.0" ).unwrap();
  assert_eq!( format!( "{}", ver ), "1.0.0" );
}

#[ test ]
fn version_accessors()
{
  let ver_str = "1.2.3";
  let ver = VersionType::new( ver_str ).unwrap();

  // Test as_str()
  assert_eq!( ver.as_str(), ver_str );

  // Test into_inner()
  let inner = ver.into_inner();
  assert_eq!( inner, ver_str );
}

#[ test ]
fn version_clone_and_equality()
{
  let ver1 = VersionType::new( "1.0.0" ).unwrap();
  let ver2 = ver1.clone();

  assert_eq!( ver1, ver2, "Cloned version should equal original" );

  let ver3 = VersionType::new( "2.0.0" ).unwrap();
  assert_ne!( ver1, ver3, "Different versions should not be equal" );
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn version_serde_json_serialize()
{
  let ver = VersionType::new( "1.0.0" ).unwrap();
  let json = serde_json::to_string( &ver ).expect( "serialization should succeed" );

  assert_eq!( json, "\"1.0.0\"" );
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn version_serde_json_deserialize_valid()
{
  let json = "\"1.0.0\"";
  let ver : VersionType = serde_json::from_str( json )
    .expect( "deserialization should succeed" );
  assert_eq!( ver.as_str(), "1.0.0" );
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn version_serde_json_deserialize_rejects_empty()
{
  let json_empty = "\"\"";
  let result : Result< VersionType, _ > = serde_json::from_str( json_empty );
  assert!(
    result.is_err(),
    "Deserialization should fail for empty version"
  );
}

#[ cfg( feature = "yaml_parser" ) ]
#[ test ]
fn version_serde_yaml_deserialize_valid()
{
  let yaml = "1.0.0";
  let ver : VersionType = serde_yaml::from_str( yaml )
    .expect( "YAML deserialization should succeed" );
  assert_eq!( ver.as_str(), "1.0.0" );
}

#[ cfg( feature = "yaml_parser" ) ]
#[ test ]
fn version_serde_yaml_deserialize_rejects_empty()
{
  let yaml_empty = "\"\"";
  let result : Result< VersionType, _ > = serde_yaml::from_str( yaml_empty );
  assert!(
    result.is_err(),
    "YAML deserialization should fail for empty version"
  );
}

//
// CommandStatus Tests
//

#[ test ]
fn command_status_active()
{
  let active = CommandStatus::Active;
  
  assert!(active.is_active(), "Active status should report is_active()");
  assert!(!active.is_deprecated(), "Active status should not be deprecated");
  assert!(!active.is_experimental(), "Active status should not be experimental");
  assert!(!active.is_internal(), "Active status should not be internal");

  assert_eq!(format!("{}", active), "active");
}

#[ test ]
fn command_status_experimental()
{
  let experimental = CommandStatus::Experimental;
  
  assert!(!experimental.is_active(), "Experimental status should not be active");
  assert!(experimental.is_experimental(), "Experimental status should report is_experimental()");
  assert!(!experimental.is_deprecated(), "Experimental status should not be deprecated");
  assert!(!experimental.is_internal(), "Experimental status should not be internal");

  assert_eq!(format!("{}", experimental), "experimental");
}

#[ test ]
fn command_status_internal()
{
  let internal = CommandStatus::Internal;
  
  assert!(!internal.is_active(), "Internal status should not be active");
  assert!(internal.is_internal(), "Internal status should report is_internal()");
  assert!(!internal.is_deprecated(), "Internal status should not be deprecated");
  assert!(!internal.is_experimental(), "Internal status should not be experimental");

  assert_eq!(format!("{}", internal), "internal");
}

#[ test ]
fn command_status_deprecated_full()
{
  let deprecated = CommandStatus::Deprecated
  {
    reason: "Use .new_command instead".to_string(),
    since: Some("2.0.0".to_string()),
    replacement: Some(".new_command".to_string()),
  };
  
  assert!(!deprecated.is_active(), "Deprecated status should not be active");
  assert!(deprecated.is_deprecated(), "Deprecated status should report is_deprecated()");
  assert!(!deprecated.is_experimental(), "Deprecated status should not be experimental");
  assert!(!deprecated.is_internal(), "Deprecated status should not be internal");

  let (reason, since, replacement) = deprecated.deprecation_info().unwrap();
  assert_eq!(reason, "Use .new_command instead");
  assert_eq!(since.as_ref().unwrap(), "2.0.0");
  assert_eq!(replacement.as_ref().unwrap(), ".new_command");

  let display = format!("{}", deprecated);
  assert!(display.contains("deprecated"));
  assert!(display.contains("2.0.0"));
  assert!(display.contains("Use .new_command instead"));
  assert!(display.contains(".new_command"));
}

#[ test ]
fn command_status_deprecated_minimal()
{
  let deprecated = CommandStatus::Deprecated
  {
    reason: String::new(),
    since: None,
    replacement: None,
  };
  
  assert!(deprecated.is_deprecated());

  let (reason, since, replacement) = deprecated.deprecation_info().unwrap();
  assert_eq!(reason, "");
  assert!(since.is_none());
  assert!(replacement.is_none());
}

#[ test ]
fn command_status_default()
{
  let default = CommandStatus::default();
  assert!(default.is_active(), "Default status should be Active");
}

#[ test ]
fn command_status_clone_and_equality()
{
  let active1 = CommandStatus::Active;
  let active2 = active1.clone();
  assert_eq!(active1, active2);

  let experimental = CommandStatus::Experimental;
  assert_ne!(active1, experimental);

  let deprecated1 = CommandStatus::Deprecated
  {
    reason: "Old API".to_string(),
    since: Some("1.0.0".to_string()),
    replacement: Some(".new".to_string()),
  };
  let deprecated2 = deprecated1.clone();
  assert_eq!(deprecated1, deprecated2);
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn command_status_serde_json_active()
{
  let active = CommandStatus::Active;
  let json = serde_json::to_string(&active).expect("serialization should succeed");
  assert_eq!(json, "\"active\"");

  let deserialized: CommandStatus = serde_json::from_str(&json)
    .expect("deserialization should succeed");
  assert_eq!(active, deserialized);
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn command_status_serde_json_experimental()
{
  let experimental = CommandStatus::Experimental;
  let json = serde_json::to_string(&experimental).expect("serialization should succeed");
  assert_eq!(json, "\"experimental\"");

  let deserialized: CommandStatus = serde_json::from_str(&json)
    .expect("deserialization should succeed");
  assert_eq!(experimental, deserialized);
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn command_status_serde_json_internal()
{
  let internal = CommandStatus::Internal;
  let json = serde_json::to_string(&internal).expect("serialization should succeed");
  assert_eq!(json, "\"internal\"");

  let deserialized: CommandStatus = serde_json::from_str(&json)
    .expect("deserialization should succeed");
  assert_eq!(internal, deserialized);
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn command_status_serde_json_deprecated()
{
  let deprecated = CommandStatus::Deprecated
  {
    reason: "Use .new instead".to_string(),
    since: Some("2.0.0".to_string()),
    replacement: Some(".new".to_string()),
  };

  let json = serde_json::to_string(&deprecated).expect("serialization should succeed");
  
  // Should be a JSON object with fields
  assert!(json.contains("\"status\""));
  assert!(json.contains("\"deprecated\""));
  assert!(json.contains("\"reason\""));
  assert!(json.contains("Use .new instead"));

  let deserialized: CommandStatus = serde_json::from_str(&json)
    .expect("deserialization should succeed");
  
  assert!(deserialized.is_deprecated());
  let (reason, since, replacement) = deserialized.deprecation_info().unwrap();
  assert_eq!(reason, "Use .new instead");
  assert_eq!(since.as_ref().unwrap(), "2.0.0");
  assert_eq!(replacement.as_ref().unwrap(), ".new");
}

#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn command_status_serde_json_backward_compatible()
{
  // Test backward compatibility with string-based status
  let test_cases = vec![
    ("\"stable\"", CommandStatus::Active),
    ("\"active\"", CommandStatus::Active),
    ("\"experimental\"", CommandStatus::Experimental),
    ("\"internal\"", CommandStatus::Internal),
    ("\"deprecated\"", CommandStatus::Deprecated { 
      reason: String::new(), 
      since: None, 
      replacement: None 
    }),
  ];

  for (json, expected) in test_cases
  {
    let deserialized: CommandStatus = serde_json::from_str(json)
      .expect(&format!("deserialization of {} should succeed", json));
    assert_eq!(deserialized, expected, "Failed for JSON: {}", json);
  }
}

#[ cfg( feature = "yaml_parser" ) ]
#[ test ]
fn command_status_serde_yaml_simple()
{
  let test_cases = vec![
    ("active", CommandStatus::Active),
    ("stable", CommandStatus::Active),
    ("experimental", CommandStatus::Experimental),
    ("internal", CommandStatus::Internal),
    ("deprecated", CommandStatus::Deprecated { 
      reason: String::new(), 
      since: None, 
      replacement: None 
    }),
  ];

  for (yaml, expected) in test_cases
  {
    let deserialized: CommandStatus = serde_yaml::from_str(yaml)
      .expect(&format!("YAML deserialization of {} should succeed", yaml));
    assert_eq!(deserialized, expected, "Failed for YAML: {}", yaml);
  }
}

#[ cfg( feature = "yaml_parser" ) ]
#[ test ]
fn command_status_serde_yaml_deprecated_object()
{
  let yaml = r#"
status: deprecated
reason: Use .new instead
since: 2.0.0
replacement: .new
"#;

  let deserialized: CommandStatus = serde_yaml::from_str(yaml)
    .expect("YAML deserialization should succeed");
  
  assert!(deserialized.is_deprecated());
  let (reason, since, replacement) = deserialized.deprecation_info().unwrap();
  assert_eq!(reason, "Use .new instead");
  assert_eq!(since.as_ref().unwrap(), "2.0.0");
  assert_eq!(replacement.as_ref().unwrap(), ".new");
}
