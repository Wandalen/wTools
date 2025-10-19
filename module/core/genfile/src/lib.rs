#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/genfile/latest/genfile/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]

//!
//! File generation tools for code generation and template materialization.
//!
//! genfile provides a trait-based template processing and file generation library
//! that supports custom value types, pluggable rendering engines, and testable
//! in-memory file systems.
//!

/// Template value types and trait
#[ cfg( feature = "enabled" ) ]
pub mod value;

/// Parameter descriptors and collections
#[ cfg( feature = "enabled" ) ]
pub mod parameter;

/// Values storage for template substitution
#[ cfg( feature = "enabled" ) ]
pub mod values;

/// Error types for genfile operations
#[ cfg( feature = "enabled" ) ]
pub mod error;

/// Security validation functions
#[ cfg( feature = "enabled" ) ]
pub mod security;

/// Template rendering engines
#[ cfg( feature = "enabled" ) ]
pub mod renderer;

/// File descriptors and write modes
#[ cfg( feature = "enabled" ) ]
pub mod file_descriptor;

/// File system abstractions for testability
#[ cfg( feature = "enabled" ) ]
pub mod filesystem;

/// Template materialization orchestrator
#[ cfg( feature = "enabled" ) ]
pub mod template;

/// Template archive system for packaging and materialization
#[ cfg( feature = "enabled" ) ]
pub mod archive;

/// Content source abstractions for external data
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
