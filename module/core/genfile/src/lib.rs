#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/genfile/latest/genfile/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]

//!
//! File generation tools for code generation and template materialization.
//!
//! genfile_core provides a trait-based template processing and file generation library
//! that supports custom value types, pluggable rendering engines, and testable
//! in-memory file systems.
//!
//! # Architecture
//!
//! The library is organized around two main APIs:
//!
//! 1. **[`TemplateArchive`]** - Self-contained archive with files, parameters, and values
//!    - Recommended for most use cases
//!    - Supports JSON/YAML serialization
//!    - Parameter definitions and values stored inside the archive
//!    - External content sources with internalize/externalize operations
//!
//! 2. **[`Template`]** - Lower-level template processor
//!    - Generic over value types and renderers
//!    - Direct filesystem and renderer control
//!    - Useful when you need custom value types or renderers
//!
//! # Core Concepts
//!
//! ## Values and Parameters
//!
//! - [`TemplateValue`] - Trait for custom value types
//! - [`Value`] - Default value enum (String, Number, Bool, List)
//! - [`ParameterDescriptor`] - Parameter metadata (name, mandatory, default, description)
//! - [`Values`] - Runtime storage for parameter values
//!
//! ## Rendering
//!
//! - [`TemplateRenderer`] - Trait for pluggable template engines
//! - [`HandlebarsRenderer`] - Default Handlebars implementation
//!
//! ## File System
//!
//! - [`FileSystem`] - Abstraction for file I/O operations
//! - [`RealFileSystem`] - Production filesystem implementation
//! - [`MemoryFileSystem`] - In-memory filesystem for testing
//!
//! ## Content Sources
//!
//! - [`ContentSource`] - Inline, File, or URL references
//! - [`ContentResolver`] - Custom content fetching logic
//! - [`ContentStorage`] - Custom storage backends (S3, Azure, etc.)
//!
//! # Module Organization
//!
//! The crate is organized into focused modules:
//!
//! - [`value`] - Value types and the TemplateValue trait
//! - [`parameter`] - Parameter descriptors and collections
//! - [`values`] - Runtime value storage
//! - [`renderer`] - Template rendering engines
//! - [`filesystem`] - File system abstractions
//! - [`archive`] - Self-contained template archives
//! - [`template`] - Low-level template processor
//! - [`content_source`] - External content references
//! - [`security`] - Path traversal validation
//! - [`error`] - Error types
//!
//! # See Also
//!
//! - [README](index.html) for quick start guide and examples
//! - [`TemplateArchive`] for the recommended high-level API
//! - [`Template`] for custom value types and renderers
//!

/// Template value types and the [`TemplateValue`] trait.
///
/// Defines how values are converted to strings for template substitution.
/// Implement [`TemplateValue`] for custom value types.
///
/// See also: [`value::Value`] for the default enum-based implementation.
#[ cfg( feature = "enabled" ) ]
pub mod value;

/// Parameter descriptors and collections.
///
/// Defines template parameters with metadata like mandatory/optional status,
/// default values, and descriptions.
///
/// See also: [`ParameterDescriptor`] for individual parameters, [`Parameters`] for collections.
#[ cfg( feature = "enabled" ) ]
pub mod parameter;

/// Runtime value storage for template substitution.
///
/// Provides [`Values<V>`] container for storing parameter values during
///materialization. Supports type-preserving serialization for template engines.
///
/// See also: [`value`] for value types, [`parameter`] for parameter definitions.
#[ cfg( feature = "enabled" ) ]
pub mod values;

/// Error types for genfile operations.
///
/// Defines [`Error`] enum with variants for different failure modes:
/// rendering errors, missing parameters, filesystem errors, and invalid templates.
#[ cfg( feature = "enabled" ) ]
pub mod error;

/// Security validation functions for path traversal prevention.
///
/// Provides [`validate_path`] function to reject paths containing `..` segments,
/// preventing directory traversal attacks.
///
/// See also: [`archive::TemplateArchive::materialize_with_components`] which uses this validation.
#[ cfg( feature = "enabled" ) ]
pub mod security;

/// Template rendering engines and the [`TemplateRenderer`] trait.
///
/// Abstracts template processing behind a trait for pluggable engines.
/// Includes [`HandlebarsRenderer`] as the default implementation.
///
/// See also: [`Template`] which accepts custom renderers.
#[ cfg( feature = "enabled" ) ]
pub mod renderer;

/// File descriptors and write modes for template materialization.
///
/// Defines [`FileDescriptor`] linking template files to output paths,
/// and [`WriteMode`] controlling how generated content is written.
///
/// See also: [`template::Template::add_file`] for usage.
#[ cfg( feature = "enabled" ) ]
pub mod file_descriptor;

/// File system abstractions for testability.
///
/// Provides [`FileSystem`] trait with implementations for real disk I/O
/// ([`RealFileSystem`]) and in-memory testing ([`MemoryFileSystem`]).
///
/// See also: [`Template`] for usage with custom filesystems.
#[ cfg( feature = "enabled" ) ]
pub mod filesystem;

/// Low-level template processor with custom value types and renderers.
///
/// Provides [`Template<V,R,FS>`] struct for direct control over
/// value types, rendering engines, and file systems.
///
/// See also: [`TemplateArchive`] for higher-level self-contained archives.
#[ cfg( feature = "enabled" ) ]
pub mod template;

/// Self-contained template archives with serialization support.
///
/// Provides [`TemplateArchive`] - the main high-level API for most use cases.
/// Archives are self-contained with files, parameters, and values stored together,
/// supporting JSON/YAML serialization and external content references.
///
/// See also: [`Template`] for lower-level custom implementations.
#[ cfg( feature = "enabled" ) ]
pub mod archive;

/// Content source abstractions for external data references.
///
/// Enables archives to reference external content from files, URLs, or custom sources.
/// Provides [`ContentSource`], [`ContentResolver`], and [`ContentStorage`] for
/// flexible content management.
///
/// See also: [`archive::TemplateArchive::internalize`] and [`archive::TemplateArchive::externalize`].
#[ cfg( feature = "enabled" ) ]
pub mod content_source;

#[ cfg( feature = "enabled" ) ]
pub use value :: { TemplateValue, Value };

#[ cfg( feature = "enabled" ) ]
pub use parameter :: { ParameterDescriptor, Parameters };

#[ cfg( feature = "enabled" ) ]
pub use values ::Values;

#[ cfg( feature = "enabled" ) ]
pub use error ::Error;

#[ cfg( feature = "enabled" ) ]
pub use security ::validate_path;

#[ cfg( feature = "enabled" ) ]
pub use renderer :: { TemplateRenderer, HandlebarsRenderer };

#[ cfg( feature = "enabled" ) ]
pub use file_descriptor :: { FileDescriptor, WriteMode };

#[ cfg( feature = "enabled" ) ]
pub use filesystem :: { FileSystem, MemoryFileSystem, RealFileSystem };

#[ cfg( feature = "enabled" ) ]
pub use template ::Template;

#[ cfg( feature = "enabled" ) ]
pub use archive ::
{
  TemplateArchive,
  TemplateFile,
  FileContent,
  FileMetadata,
  ArchiveMetadata,
  MaterializationReport,
};

#[ cfg( feature = "enabled" ) ]
pub use content_source ::
{
  ContentSource,
  IntoContentSource,
  FileRef,
  UrlRef,
  InlineContent,
  ContentResolver,
  ContentStorage,
  DefaultContentResolver,
  DefaultContentStorage,
};

/// Prelude for convenient imports.
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{
  pub use super ::
  {
    TemplateValue,
    Value,
    ParameterDescriptor,
    Parameters,
    Values,
    Error,
    validate_path,
    TemplateRenderer,
    HandlebarsRenderer,
    FileDescriptor,
    WriteMode,
    FileSystem,
    MemoryFileSystem,
    RealFileSystem,
    Template,
    TemplateArchive,
    TemplateFile,
    FileContent,
    FileMetadata,
    ArchiveMetadata,
    MaterializationReport,
    ContentSource,
    IntoContentSource,
    FileRef,
    UrlRef,
    InlineContent,
    ContentResolver,
    ContentStorage,
    DefaultContentResolver,
    DefaultContentStorage,
  };
}
