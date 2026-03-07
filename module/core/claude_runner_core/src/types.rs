//! Type definitions for Claude Code configuration
//!
//! Provides type-safe enums for configuration options that map to Claude Code
//! environment variables.

/// Tool approval behavior mode
///
/// Controls how Claude Code handles tool execution requests.
///
/// # Environment Variable
///
/// Maps to `CLAUDE_CODE_ACTION_MODE` environment variable.
///
/// # Examples
///
/// ```
/// use claude_runner_core::ActionMode;
///
/// let mode = ActionMode::Ask;  // Default: prompt user for each tool
/// let mode = ActionMode::Allow;  // Auto-approve all tools (use with caution)
/// let mode = ActionMode::Deny;  // Reject all tool executions
/// ```
#[derive( Debug, Clone, Copy, PartialEq, Eq )]
pub enum ActionMode {
  /// Prompt user before each tool execution (default, safest)
  Ask,
  /// Automatically approve all tool executions (requires explicit opt-in)
  Allow,
  /// Deny all tool execution requests
  Deny,
}

impl ActionMode {
  /// Convert to environment variable string value
  ///
  /// # Examples
  ///
  /// ```
  /// use claude_runner_core::ActionMode;
  ///
  /// assert_eq!( ActionMode::Ask.as_str(), "ask" );
  /// assert_eq!( ActionMode::Allow.as_str(), "allow" );
  /// assert_eq!( ActionMode::Deny.as_str(), "deny" );
  /// ```
  #[inline]
  #[must_use]
  pub fn as_str( self ) -> &'static str {
    match self {
      Self::Ask => "ask",
      Self::Allow => "allow",
      Self::Deny => "deny",
    }
  }
}

impl Default for ActionMode {
  #[inline]
  fn default() -> Self {
    // Fix(issue-action-mode-default): Default is Ask for security
    // Root cause: Allow would auto-approve all tools without user consent
    // Pitfall: Never default to Allow - requires explicit opt-in
    Self::Ask
  }
}

/// Logging verbosity level
///
/// Controls the verbosity of Claude Code logging output.
///
/// # Environment Variable
///
/// Maps to `CLAUDE_CODE_LOG_LEVEL` environment variable.
///
/// # Examples
///
/// ```
/// use claude_runner_core::LogLevel;
///
/// let level = LogLevel::Info;   // Default: standard information
/// let level = LogLevel::Debug;  // Verbose debugging output
/// let level = LogLevel::Error;  // Only errors
/// ```
#[derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord )]
pub enum LogLevel {
  /// Only critical errors
  Error,
  /// Warnings and errors
  Warn,
  /// Standard information (default)
  Info,
  /// Detailed debugging information
  Debug,
  /// All possible logging output
  Trace,
}

impl LogLevel {
  /// Convert to environment variable string value
  ///
  /// # Examples
  ///
  /// ```
  /// use claude_runner_core::LogLevel;
  ///
  /// assert_eq!( LogLevel::Error.as_str(), "error" );
  /// assert_eq!( LogLevel::Warn.as_str(), "warn" );
  /// assert_eq!( LogLevel::Info.as_str(), "info" );
  /// assert_eq!( LogLevel::Debug.as_str(), "debug" );
  /// assert_eq!( LogLevel::Trace.as_str(), "trace" );
  /// ```
  #[inline]
  #[must_use]
  pub fn as_str( self ) -> &'static str {
    match self {
      Self::Error => "error",
      Self::Warn => "warn",
      Self::Info => "info",
      Self::Debug => "debug",
      Self::Trace => "trace",
    }
  }
}

impl Default for LogLevel {
  #[inline]
  fn default() -> Self {
    Self::Info
  }
}

/// Output from a non-interactive Claude Code execution
///
/// Contains captured stdout, stderr, and exit code from the process.
///
/// # Examples
///
/// ```no_run
/// use claude_runner_core::ClaudeCommand;
///
/// let output = ClaudeCommand::new()
///   .with_message( "hello" )
///   .execute()?;
///
/// println!( "stdout: {}", output.stdout );
/// if !output.stderr.is_empty()
/// {
///   eprintln!( "stderr: {}", output.stderr );
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive( Debug, Clone, PartialEq, Eq )]
pub struct ExecutionOutput
{
  /// Captured stdout from Claude Code process.
  pub stdout : String,
  /// Captured stderr from Claude Code process.
  pub stderr : String,
  /// Process exit code (0 = success).
  pub exit_code : i32,
}

impl core::fmt::Display for ExecutionOutput
{
  #[inline]
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    write!( f, "{}", self.stdout )
  }
}
