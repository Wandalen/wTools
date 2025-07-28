//!
//! Core data structures for the Unilang framework.
//!

use crate::error::Error;

// use former::Former;

///
/// Defines a command, including its name, arguments, and other metadata.
///
/// This struct is the central piece of a command's definition, providing all
/// the necessary information for parsing, validation, and execution.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, former::Former)]
pub struct CommandDefinition {
  /// The name of the command, used to invoke it from the command line.
  pub name: String,
  /// A brief, one-line description of what the command does.
  pub description: String,
  /// A list of arguments that the command accepts.
  // #[ former( default ) ]
  pub arguments: Vec<ArgumentDefinition>,
  /// An optional link to the routine that executes this command.
  pub routine_link: Option<String>,
  /// The namespace of the command.
  pub namespace: String, // Changed from Option<String> to String
  /// A short hint for the command.
  pub hint: String,
  /// The status of the command.
  pub status: String,
  /// The version of the command.
  pub version: String,
  /// Tags associated with the command.
  pub tags: Vec<String>,
  /// Aliases for the command.
  pub aliases: Vec<String>,
  /// Permissions required to execute the command.
  pub permissions: Vec<String>,
  /// Indicates if the command is idempotent.
  pub idempotent: bool,
  /// If `status` is `Deprecated`, explains the reason and suggests alternatives.
  pub deprecation_message: String, // Added
  /// A suggested HTTP method (`GET`, `POST`, etc.) for the Web API modality.
  pub http_method_hint: String, // Added
  /// Illustrative usage examples for help text.
  pub examples: Vec<String>, // Added
}

///
/// Attributes for an argument, encapsulating its boolean properties.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, former::Former)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Default)]
pub struct ArgumentAttributes {
  /// If `true`, the argument is not required for the command to execute.
  pub optional: bool,
  /// If `true`, the argument can be specified multiple times.
  pub multiple: bool,
  /// Indicates if the argument is a default argument.
  pub is_default_arg: bool,
  /// Indicates if the argument is interactive.
  pub interactive: bool,
  /// Indicates if the argument is sensitive.
  pub sensitive: bool,
}

///
/// Defines an argument for a command.
///
/// Each argument has a name, a description, a data type, and can be
/// marked as optional.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, former::Former)]
pub struct ArgumentDefinition {
  /// The name of the argument, used for identification.
  pub name: String,
  /// A brief description of the argument's purpose.
  pub description: String,
  /// The expected data type of the argument.
  pub kind: Kind,
  /// Attributes for the argument.
  pub attributes: ArgumentAttributes,
  /// Custom validation rules for the argument.
  pub validation_rules: Vec<String>,
  /// A short hint for the argument.
  pub hint: String,
  /// The default value of the argument.
  pub default_value: Option<String>,
  /// Aliases for the argument.
  pub aliases: Vec<String>,
  /// Tags associated with the argument.
  pub tags: Vec<String>,
}

///
/// Represents the data type of an argument.
///
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum Kind {
  /// A sequence of characters.
  String,
  /// A whole number.
  Integer,
  /// A floating-point number.
  Float,
  /// A true or false value.
  Boolean,
  /// A URI representing a file system path.
  Path,
  /// A `Path` that must point to a file.
  File,
  /// A `Path` that must point to a directory.
  Directory,
  /// A string that must be one of the predefined, case-sensitive choices.
  Enum(Vec<String>),
  /// A Uniform Resource Locator.
  Url,
  /// A date and time.
  DateTime,
  /// A regular expression pattern string.
  Pattern,
  /// A list of elements of a specified `Type`.
  List(Box<Kind>, Option<char>),
  /// A key-value map.
  Map(Box<Kind>, Box<Kind>, Option<char>, Option<char>),
  /// A JSON string.
  JsonString,
  /// A JSON object.
  Object,
}

impl core::str::FromStr for Kind {
  type Err = crate::error::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "String" => Ok(Kind::String),
      "Integer" => Ok(Kind::Integer),
      "Float" => Ok(Kind::Float),
      "Boolean" => Ok(Kind::Boolean),
      "Path" => Ok(Kind::Path),
      "File" => Ok(Kind::File),
      "Directory" => Ok(Kind::Directory),
      "Url" => Ok(Kind::Url),
      "DateTime" => Ok(Kind::DateTime),
      "Pattern" => Ok(Kind::Pattern),
      "JsonString" => Ok(Kind::JsonString),
      "Object" => Ok(Kind::Object),
      _ => {
        // Handle List, Map, Enum with parameters
        if s.starts_with("List(") && s.ends_with(')') {
          let inner = &s["List(".len()..s.len() - 1];
          let parts: Vec<&str> = inner.splitn(2, ',').collect();
          let item_kind = parts[0].parse()?;
          let delimiter = if parts.len() > 1 {
            Some(parts[1].chars().next().ok_or_else(|| {
              Error::Execution(crate::data::ErrorData {
                code: "INVALID_KIND_FORMAT".to_string(),
                message: format!("Invalid List delimiter format: {}", parts[1]),
              })
            })?)
          } else {
            None
          };
          Ok(Kind::List(Box::new(item_kind), delimiter))
        } else if s.starts_with("Map(") && s.ends_with(')') {
          let inner = &s["Map(".len()..s.len() - 1];
          let parts: Vec<&str> = inner.splitn(4, ',').collect();
          if parts.len() < 2 {
            return Err(Error::Execution(crate::data::ErrorData {
              code: "INVALID_KIND_FORMAT".to_string(),
              message: format!("Invalid Map format: {s}"),
            }));
          }
          let key_kind = parts[0].parse()?;
          let value_kind = parts[1].parse()?;
          let entry_delimiter = if parts.len() > 2 {
            Some(parts[2].chars().next().ok_or_else(|| {
              Error::Execution(crate::data::ErrorData {
                code: "INVALID_KIND_FORMAT".to_string(),
                message: format!("Invalid Map entry delimiter format: {}", parts[2]),
              })
            })?)
          } else {
            None
          };
          let kv_delimiter = if parts.len() > 3 {
            Some(parts[3].chars().next().ok_or_else(|| {
              Error::Execution(crate::data::ErrorData {
                code: "INVALID_KIND_FORMAT".to_string(),
                message: format!("Invalid Map key-value delimiter format: {}", parts[3]),
              })
            })?)
          } else {
            None
          };
          Ok(Kind::Map(
            Box::new(key_kind),
            Box::new(value_kind),
            entry_delimiter,
            kv_delimiter,
          ))
        } else if s.starts_with("Enum(") && s.ends_with(')') {
          let inner = &s["Enum(".len()..s.len() - 1];
          let choices: Vec<String> = inner.split(',').map(|c| c.trim().to_string()).collect();
          Ok(Kind::Enum(choices))
        } else {
          Err(Error::Execution(crate::data::ErrorData {
            code: "UNKNOWN_KIND".to_string(),
            message: format!("Unknown argument kind: {s}"),
          }))
        }
      }
    }
  }
}

impl core::fmt::Display for Kind {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", String::from(self.clone()))
  }
}

///
/// Represents a namespace for organizing commands.
///
/// Namespaces allow for grouping related commands under a common prefix,
/// improving discoverability and reducing naming conflicts.
#[derive(Debug, Clone /*, Former*/)]
pub struct Namespace {
  /// The name of the namespace.
  pub name: String,
  /// A list of commands belonging to this namespace.
  // #[ former( default ) ]
  pub commands: Vec<CommandDefinition>,
}

///
/// Represents the successful output of a command execution.
///
/// This struct standardizes the way command results are returned, allowing
/// for consistent handling across different modalities.
#[derive(Debug, Clone /*, Former*/)]
pub struct OutputData {
  /// The primary content of the output.
  pub content: String,
  /// The format of the content (e.g., "text", "json").
  pub format: String,
}

///
/// Represents an error that occurred during command execution.
///
/// This struct provides a standardized way to report errors, including a
/// unique, machine-readable code and a human-readable message.
#[derive(Debug, Clone /*, Former*/)]
pub struct ErrorData {
  /// A unique, machine-readable code for the error (e.g., "`COMMAND_NOT_FOUND`").
  pub code: String,
  /// A human-readable message explaining the error.
  pub message: String,
}

impl core::fmt::Display for ErrorData {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{} (Code: {})", self.message, self.code)
  }
}

impl From<Kind> for String {
  fn from(kind: Kind) -> Self {
    match kind {
      Kind::String => "String".to_string(),
      Kind::Integer => "Integer".to_string(),
      Kind::Float => "Float".to_string(),
      Kind::Boolean => "Boolean".to_string(),
      Kind::Path => "Path".to_string(),
      Kind::File => "File".to_string(),
      Kind::Directory => "Directory".to_string(),
      Kind::Enum(choices) => format!("Enum({})", choices.join(",")),
      Kind::Url => "Url".to_string(),
      Kind::DateTime => "DateTime".to_string(),
      Kind::Pattern => "Pattern".to_string(),
      Kind::List(item_kind, delimiter) => {
        let item_kind_str: String = (*item_kind).into();
        if let Some(d) = delimiter {
          format!("List({item_kind_str},{d})")
        } else {
          format!("List({item_kind_str})")
        }
      }
      Kind::Map(key_kind, value_kind, entry_delimiter, kv_delimiter) => {
        let key_kind_str: String = (*key_kind).into();
        let value_kind_str: String = (*value_kind).into();
        let mut s = format!("Map({key_kind_str},{value_kind_str})");
        if let Some(ed) = entry_delimiter {
          s.push(',');
          s.push(ed);
        }
        if let Some(kvd) = kv_delimiter {
          s.push(',');
          s.push(kvd);
        }
        s
      }
      Kind::JsonString => "JsonString".to_string(),
      Kind::Object => "Object".to_string(),
    }
  }
}

impl core::convert::TryFrom<String> for Kind {
  type Error = crate::error::Error;

  fn try_from(s: String) -> Result<Self, Self::Error> {
    s.parse()
  }
}
