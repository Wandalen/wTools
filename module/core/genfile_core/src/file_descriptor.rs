/// File descriptors and write modes
use std ::path ::PathBuf;

/// Write mode for file generation.
///
/// Determines how generated content should be written to the target file.
///
/// # Variants
///
/// - `Rewrite`: Completely replace existing file content
/// - `TomlExtend`: Smart merge with existing TOML file (preserves comments and order)
///
/// # Examples
///
/// ```rust
/// use genfile_core::WriteMode;
///
/// // For code generation - replace entire file
/// let mode = WriteMode::Rewrite;
///
/// // For configuration files - merge intelligently
/// let mode = WriteMode::TomlExtend;
/// ```
#[ derive( Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize ) ]
pub enum WriteMode
{
  /// Completely replace file content with generated output.
  ///
  /// Use for code generation where the entire file is managed by the template.
  Rewrite,

  /// Smart merge with existing TOML file.
  ///
  /// Preserves comments, formatting, and key order while updating values.
  /// New keys are appended at appropriate locations. Only works with TOML files.
  TomlExtend,
}

/// File descriptor for template materialization.
///
/// Specifies where to read the template from, where to write the output,
/// and how to handle existing content.
///
/// # Fields
///
/// - `file_path`: Destination path for generated file
/// - `template_path`: Source template file path
/// - `write_mode`: How to handle existing file content
///
/// # Examples
///
/// ```rust
/// use genfile_core::{ FileDescriptor, WriteMode };
/// use std::path::PathBuf;
///
/// let descriptor = FileDescriptor
/// {
///   file_path: PathBuf::from( "src/generated.rs" ),
///   template_path: PathBuf::from( "templates/module.hbs" ),
///   write_mode: WriteMode::Rewrite,
/// };
/// ```
#[ derive( Debug, Clone ) ]
pub struct FileDescriptor
{
  /// Path where the generated file will be written
  pub file_path: PathBuf,

  /// Path to the template file
  pub template_path: PathBuf,

  /// How to write the generated content
  pub write_mode: WriteMode,
}
