//! Error types for claude_storage

use std::
{
  fmt,
  io,
  path::PathBuf,
};

/// Result type used throughout claude_storage
pub type Result< T > = std::result::Result< T, Error >;

/// Error type for claude_storage operations
#[derive( Debug )]
pub enum Error
{
  /// I/O error (file read/write failures)
  Io
  {
    /// Source I/O error
    source : io::Error,
    /// Context about what operation failed
    context : String,
  },

  /// JSONL parsing error
  Parse
  {
    /// Line number where parsing failed
    line : usize,
    /// The problematic content
    content : String,
    /// Description of what went wrong
    reason : String,
  },

  /// Invalid path encoding
  PathEncoding
  {
    /// The path that failed to encode/decode
    path : String,
    /// Reason for failure
    reason : String,
  },

  /// Project not found
  ProjectNotFound
  {
    /// Project identifier that was not found
    id : String,
  },

  /// Session not found
  SessionNotFound
  {
    /// Session ID that was not found
    id : String,
  },

  /// Invalid storage structure
  InvalidStructure
  {
    /// Path where structure is invalid
    path : PathBuf,
    /// Description of the problem
    reason : String,
  },

  /// Write operation failed (append-only violations)
  WriteFailed
  {
    /// What was being written
    target : String,
    /// Why it failed
    reason : String,
  },
}

impl fmt::Display for Error
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    match self
    {
      Error::Io { source, context } =>
        write!( f, "I/O error during {}: {}", context, source ),

      Error::Parse { line, content, reason } =>
        write!
        (
          f,
          "JSONL parse error at line {}: {} (content: {})",
          line, reason, content
        ),

      Error::PathEncoding { path, reason } =>
        write!( f, "Path encoding error for '{}': {}", path, reason ),

      Error::ProjectNotFound { id } =>
        write!( f, "Project not found: {}", id ),

      Error::SessionNotFound { id } =>
        write!( f, "Session not found: {}", id ),

      Error::InvalidStructure { path, reason } =>
        write!( f, "Invalid storage structure at {:?}: {}", path, reason ),

      Error::WriteFailed { target, reason } =>
        write!( f, "Write failed for {}: {}", target, reason ),
    }
  }
}

impl std::error::Error for Error
{
  fn source( &self ) -> Option< &( dyn std::error::Error + 'static ) >
  {
    match self
    {
      Error::Io { source, .. } => Some( source ),
      _ => None,
    }
  }
}

impl From< io::Error > for Error
{
  fn from( err : io::Error ) -> Self
  {
    Error::Io
    {
      source : err,
      context : "unknown operation".into(),
    }
  }
}

impl From< crate::json::JsonError > for Error
{
  fn from( err : crate::json::JsonError ) -> Self
  {
    Error::Parse
    {
      line : 0,
      content : String::new(),
      reason : err.message,
    }
  }
}

impl Error
{
  /// Create I/O error with context
  pub fn io< S : Into< String > >( source : io::Error, context : S ) -> Self
  {
    Error::Io
    {
      source,
      context : context.into(),
    }
  }

  /// Create parse error
  pub fn parse< S : Into< String > >( line : usize, content : S, reason : S ) -> Self
  {
    Error::Parse
    {
      line,
      content : content.into(),
      reason : reason.into(),
    }
  }

  /// Create path encoding error
  pub fn path_encoding< S : Into< String > >( path : S, reason : S ) -> Self
  {
    Error::PathEncoding
    {
      path : path.into(),
      reason : reason.into(),
    }
  }

  /// Create project not found error
  pub fn project_not_found< S : Into< String > >( id : S ) -> Self
  {
    Error::ProjectNotFound
    {
      id : id.into(),
    }
  }

  /// Create session not found error
  pub fn session_not_found< S : Into< String > >( id : S ) -> Self
  {
    Error::SessionNotFound
    {
      id : id.into(),
    }
  }

  /// Create invalid structure error
  pub fn invalid_structure< S : Into< String > >( path : PathBuf, reason : S ) -> Self
  {
    Error::InvalidStructure
    {
      path,
      reason : reason.into(),
    }
  }

  /// Create write failed error
  pub fn write_failed< S : Into< String > >( target : S, reason : S ) -> Self
  {
    Error::WriteFailed
    {
      target : target.into(),
      reason : reason.into(),
    }
  }
}
