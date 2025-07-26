use unilang::data::{ArgumentDefinition, CommandDefinition, Kind, ArgumentAttributes};
use unilang_parser::{SourceLocation};
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang::types::Value;
use std::path::PathBuf;
use url::Url;
use chrono::DateTime;
use regex::Regex;

fn setup_test_environment(command: CommandDefinition) -> CommandRegistry {
  let mut registry = CommandRegistry::new();
  registry.commands.insert(command.name.clone(), command);
  registry
}

fn analyze_program(
  command_name: &str,
  positional_args: Vec<unilang_parser::Argument>,
  named_args: std::collections::HashMap<String, unilang_parser::Argument>,
  registry: &CommandRegistry,
) -> Result<Vec<unilang::semantic::VerifiedCommand>, unilang::error::Error> {
  // eprintln!( "--- analyze_program debug ---" );
  // eprintln!( "Command Name: '{}'", command_name );
  // eprintln!( "Positional Args: {:?}", positional_args );
  // eprintln!( "Named Args: {:?}", named_args );

  let instructions = vec![unilang_parser::GenericInstruction {
    command_path_slices: command_name.split('.').map(|s| s.to_string()).collect(),
    named_arguments: named_args,
    positional_arguments: positional_args,
    help_requested: false,
    overall_location: SourceLocation::StrSpan { start: 0, end: 0 }, // Placeholder
  }];
  // eprintln!( "Manually Constructed Instructions: {:?}", instructions );
  let analyzer = SemanticAnalyzer::new(&instructions, registry);
  let result = analyzer.analyze();
  // eprintln!( "Analyzer Result: {:?}", result );
  // eprintln!( "--- analyze_program end ---" );
  result
}

#[test]
fn test_path_argument_type() {
  // Test Matrix Row: T1.1
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "path_arg".to_string(),
      description: "A path argument".to_string(),
      kind: Kind::Path,
      attributes: ArgumentAttributes::former()
        .optional(false)
        .multiple(false)
        .is_default_arg(false)
        .interactive(false)
        .sensitive(false)
        .form(),
      validation_rules: vec![],
      hint: "".to_string(),
      default_value: None,
      aliases: vec![],
      tags: vec![],
    }],
    routine_link: None,
    namespace: "".to_string(),
    hint: "".to_string(),
    status: "".to_string(),
    version: None,
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
  };
  let registry = setup_test_environment(command);
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: "./some/relative/path".to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_ok());
  let verified_command = result.unwrap().remove(0);
  let arg = verified_command.arguments.get("path_arg").unwrap();
  assert_eq!(*arg, Value::Path(PathBuf::from("./some/relative/path")));

  // Test Matrix Row: T1.4
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: "".to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_err());
  let error = result.err().unwrap();
  assert!(matches!( error, unilang::error::Error::Execution( data ) if data.code == "INVALID_ARGUMENT_TYPE" ));
}

#[test]
fn test_file_argument_type() {
  let file_path = "test_file.txt";
  let _ = std::fs::remove_file(file_path); // cleanup before
  std::fs::write(file_path, "test").unwrap();
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "file_arg".to_string(),
      description: "A file argument".to_string(),
      kind: Kind::File,
      attributes: ArgumentAttributes::former()
        .optional(false)
        .multiple(false)
        .is_default_arg(false)
        .interactive(false)
        .sensitive(false)
        .form(),
      validation_rules: vec![],
      hint: "".to_string(),
      default_value: None,
      aliases: vec![],
      tags: vec![],
    }],
    routine_link: None,
    namespace: "".to_string(),
    hint: "".to_string(),
    status: "".to_string(),
    version: None,
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
  };
  let registry = setup_test_environment(command);

  // Test Matrix Row: T1.5
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: file_path.to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_ok());
  let verified_command = result.unwrap().remove(0);
  let arg = verified_command.arguments.get("file_arg").unwrap();
  assert_eq!(*arg, Value::File(PathBuf::from(file_path)));

  // Test Matrix Row: T1.6
  let dir_path = "test_dir_for_file_test";
  let _ = std::fs::remove_dir_all(dir_path); // cleanup before
  std::fs::create_dir(dir_path).unwrap();
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: dir_path.to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_err());
  let error = result.err().unwrap();
  assert!(matches!( error, unilang::error::Error::Execution( data ) if data.code == "INVALID_ARGUMENT_TYPE" ));

  // Cleanup
  let _ = std::fs::remove_file(file_path);
  let _ = std::fs::remove_dir_all(dir_path);
}

#[test]
fn test_directory_argument_type() {
  let dir_path = "test_dir_2";
  let _ = std::fs::remove_dir_all(dir_path); // cleanup before
  std::fs::create_dir(dir_path).unwrap();
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "dir_arg".to_string(),
      description: "A directory argument".to_string(),
      kind: Kind::Directory,
      attributes: ArgumentAttributes::former()
        .optional(false)
        .multiple(false)
        .is_default_arg(false)
        .interactive(false)
        .sensitive(false)
        .form(),
      validation_rules: vec![],
      hint: "".to_string(),
      default_value: None,
      aliases: vec![],
      tags: vec![],
    }],
    routine_link: None,
    namespace: "".to_string(),
    hint: "".to_string(),
    status: "".to_string(),
    version: None,
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
  };
  let registry = setup_test_environment(command);

  // Test Matrix Row: T1.8
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: dir_path.to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_ok());
  let verified_command = result.unwrap().remove(0);
  let arg = verified_command.arguments.get("dir_arg").unwrap();
  assert_eq!(*arg, Value::Directory(PathBuf::from(dir_path)));

  // Test Matrix Row: T1.9
  let file_path = "test_file_2.txt";
  let _ = std::fs::remove_file(file_path); // cleanup before
  std::fs::write(file_path, "test").unwrap();
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: file_path.to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_err());
  let error = result.err().unwrap();
  assert!(matches!( error, unilang::error::Error::Execution( data ) if data.code == "INVALID_ARGUMENT_TYPE" ));

  // Cleanup
  let _ = std::fs::remove_dir_all(dir_path);
  let _ = std::fs::remove_file(file_path);
}

#[test]
fn test_enum_argument_type() {
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "enum_arg".to_string(),
      description: "An enum argument".to_string(),
      kind: Kind::Enum(vec!["A".to_string(), "B".to_string(), "C".to_string()]),
      attributes: ArgumentAttributes::former()
        .optional(false)
        .multiple(false)
        .is_default_arg(false)
        .interactive(false)
        .sensitive(false)
        .form(),
      validation_rules: vec![],
      hint: "".to_string(),
      default_value: None,
      aliases: vec![],
      tags: vec![],
    }],
    routine_link: None,
    namespace: "".to_string(),
    hint: "".to_string(),
    status: "".to_string(),
    version: None,
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
  };
  let registry = setup_test_environment(command);

  // Test Matrix Row: T1.10
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: "A".to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_ok());
  let verified_command = result.unwrap().remove(0);
  let arg = verified_command.arguments.get("enum_arg").unwrap();
  assert_eq!(*arg, Value::Enum("A".to_string()));

  // Test Matrix Row: T1.12
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: "D".to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_err());
  let error = result.err().unwrap();
  assert!(matches!( error, unilang::error::Error::Execution( data ) if data.code == "INVALID_ARGUMENT_TYPE" ));

  // Test Matrix Row: T1.13
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: "a".to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_err());
  let error = result.err().unwrap();
  assert!(matches!( error, unilang::error::Error::Execution( data ) if data.code == "INVALID_ARGUMENT_TYPE" ));
}

#[test]
fn test_url_argument_type() {
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "url_arg".to_string(),
      description: "A URL argument".to_string(),
      kind: Kind::Url,
      attributes: ArgumentAttributes::former()
        .optional(false)
        .multiple(false)
        .is_default_arg(false)
        .interactive(false)
        .sensitive(false)
        .form(),
      validation_rules: vec![],
      hint: "".to_string(),
      default_value: None,
      aliases: vec![],
      tags: vec![],
    }],
    routine_link: None,
    namespace: "".to_string(),
    hint: "".to_string(),
    status: "".to_string(),
    version: None,
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
  };
  let registry = setup_test_environment(command);

  // Test Matrix Row: T1.14
  let url_str = "https://example.com/path?q=1";
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: url_str.to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_ok());
  let verified_command = result.unwrap().remove(0);
  let arg = verified_command.arguments.get("url_arg").unwrap();
  assert_eq!(*arg, Value::Url(Url::parse(url_str).unwrap()));

  // Test Matrix Row: T1.16
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: "not a url".to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_err());
  let error = result.err().unwrap();
  assert!(matches!( error, unilang::error::Error::Execution( data ) if data.code == "INVALID_ARGUMENT_TYPE" ));
}

#[test]
fn test_datetime_argument_type() {
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "dt_arg".to_string(),
      description: "A DateTime argument".to_string(),
      kind: Kind::DateTime,
      attributes: ArgumentAttributes::former()
        .optional(false)
        .multiple(false)
        .is_default_arg(false)
        .interactive(false)
        .sensitive(false)
        .form(),
      validation_rules: vec![],
      hint: "".to_string(),
      default_value: None,
      aliases: vec![],
      tags: vec![],
    }],
    routine_link: None,
    namespace: "".to_string(),
    hint: "".to_string(),
    status: "".to_string(),
    version: None,
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
  };
  let registry = setup_test_environment(command);

  // Test Matrix Row: T1.18
  let dt_str = "2025-06-28T12:00:00Z";
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: dt_str.to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_ok());
  let verified_command = result.unwrap().remove(0);
  let arg = verified_command.arguments.get("dt_arg").unwrap();
  assert_eq!(*arg, Value::DateTime(DateTime::parse_from_rfc3339(dt_str).unwrap()));

  // Test Matrix Row: T1.20
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: "2025-06-28".to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_err());
  let error = result.err().unwrap();
  assert!(matches!( error, unilang::error::Error::Execution( data ) if data.code == "INVALID_ARGUMENT_TYPE" ));
}

#[test]
fn test_pattern_argument_type() {
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "pattern_arg".to_string(),
      description: "A Pattern argument".to_string(),
      kind: Kind::Pattern,
      attributes: ArgumentAttributes::former()
        .optional(false)
        .multiple(false)
        .is_default_arg(false)
        .interactive(false)
        .sensitive(false)
        .form(),
      validation_rules: vec![],
      hint: "".to_string(),
      default_value: None,
      aliases: vec![],
      tags: vec![],
    }],
    routine_link: None,
    namespace: "".to_string(),
    hint: "".to_string(),
    status: "".to_string(),
    version: None,
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
  };
  let registry = setup_test_environment(command);

  // Test Matrix Row: T1.22
  let pattern_str = "^[a-z]+$";
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: pattern_str.to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_ok());
  let verified_command = result.unwrap().remove(0);
  let arg = verified_command.arguments.get("pattern_arg").unwrap();
  // Regex does not implement PartialEq, so we compare the string representation
  assert_eq!(arg.to_string(), Value::Pattern(Regex::new(pattern_str).unwrap()).to_string());

  // Test Matrix Row: T1.23
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: "[a-z".to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_err());
  let error = result.err().unwrap();
  assert!(matches!( error, unilang::error::Error::Execution( data ) if data.code == "INVALID_ARGUMENT_TYPE" ));
}

#[test]
fn test_default_argument() {
  let command = CommandDefinition {
    name: ".test.command".to_string(),
    description: "A test command".to_string(),
    arguments: vec![ArgumentDefinition {
      name: "default_arg".to_string(),
      description: "An argument with a default value".to_string(),
      kind: Kind::String,
      attributes: ArgumentAttributes::former()
        .optional(true)
        .multiple(false)
        .is_default_arg(true)
        .interactive(false)
        .sensitive(false)
        .form(),
      validation_rules: vec![],
      hint: "".to_string(),
      default_value: Some("default_value_string".to_string()),
      aliases: vec![],
      tags: vec![],
    }],
    routine_link: None,
    namespace: "".to_string(),
    hint: "".to_string(),
    status: "".to_string(),
    version: None,
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
  };
  let registry = setup_test_environment(command);

  // Test Matrix Row: T1.9 (no value provided, use default)
  let result = analyze_program(".test.command", vec![], std::collections::HashMap::new(), &registry);
  assert!(result.is_ok());
  let verified_command = result.unwrap().remove(0);
  let arg = verified_command.arguments.get("default_arg").unwrap();
  assert_eq!(*arg, Value::String("default_value_string".to_string()));

  // Test Matrix Row: T1.10 (value provided, override default)
  let result = analyze_program(
    ".test.command",
    vec![unilang_parser::Argument {
      name: None,
      value: "provided_value".to_string(),
      name_location: None,
      value_location: SourceLocation::StrSpan { start: 0, end: 0 },
    }],
    std::collections::HashMap::new(),
    &registry,
  );
  assert!(result.is_ok());
  let verified_command = result.unwrap().remove(0);
  let arg = verified_command.arguments.get("default_arg").unwrap();
  assert_eq!(*arg, Value::String("provided_value".to_string()));
}
