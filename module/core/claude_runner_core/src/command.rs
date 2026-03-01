//! Claude Code Command Builder
//!
//! Provides fluent API for constructing and executing Claude Code CLI commands.
//!
//! ## Execution Modes
//!
//! This module supports two execution modes:
//!
//! - **Non-interactive mode** ([`execute`](ClaudeCommand::execute)): Captures stdout/stderr, suitable for programmatic usage
//! - **Interactive mode** ([`execute_interactive`](ClaudeCommand::execute_interactive)): Allows Claude Code to take over terminal (TTY attached)
//!
//! The distinction is critical: `.output()` captures process output which prevents Claude Code from
//! accessing the terminal for interactive sessions. Interactive mode uses `.status()` to preserve TTY access.

use std::path::PathBuf;
use error_tools::{ Result, Error };

/// Builder for Claude Code CLI commands
///
/// # Example
///
/// ```no_run
/// use claude_runner_core::ClaudeCommand;
///
/// let result = ClaudeCommand::new()
///   .with_working_directory( "/home/user/project" )
///   .with_max_output_tokens( 200_000 )
///   .execute()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive( Debug )]
pub struct ClaudeCommand {
  working_directory: Option<PathBuf>,
  max_output_tokens: Option<u32>,
  continue_conversation: bool,
  message: Option<String>,
  args: Vec<String>,

  // Tier 1: Critical parameters with different defaults (fix automation blockers)
  bash_default_timeout_ms: Option<u32>,
  bash_max_timeout_ms: Option<u32>,
  auto_continue: Option<bool>,
  telemetry: Option<bool>,

  // Tier 2: Essential parameters with standard defaults (security-sensitive)
  auto_approve_tools: Option<bool>,
  action_mode: Option<crate::types::ActionMode>,
  log_level: Option<crate::types::LogLevel>,
  temperature: Option<f64>,

  // Safety override
  skip_permissions: bool,

  // Tier 3: Optional parameters with standard defaults
  sandbox_mode: Option<bool>,
  session_dir: Option<PathBuf>,
  top_p: Option<f64>,
  top_k: Option<u32>,
}

impl ClaudeCommand {
  /// Create a new Claude Code command builder
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new();
  /// ```
  #[inline]
  #[must_use]
  pub fn new() -> Self {
    // Fix(issue-token-limit-default): Default token limit changed from 32K to 200K
    // Root cause: Migration from factory pattern didnt preserve correct default value
    // Pitfall: Always verify defaults match specification when refactoring APIs

    // Fix(issue-bash-timeout-default): Bash timeouts increased from 2min/10min to 1hr/2hr
    // Root cause: Standard 2min default causes premature timeout in real automation workflows
    // Pitfall: Always set explicit timeouts matching actual operation duration needs

    // Fix(issue-auto-continue-default): Auto-continue enabled by default (true vs false)
    // Root cause: Standard false blocks all automation with manual prompts
    // Pitfall: Programmatic usage requires automation-friendly defaults

    // Fix(issue-telemetry-default): Telemetry disabled by default (false vs true)
    // Root cause: Automation contexts shouldnt send usage data without explicit consent
    // Pitfall: Respect user privacy in programmatic execution

    Self {
      working_directory: None,
      max_output_tokens: Some( 200_000 ),
      continue_conversation: false,
      message: None,
      args: Vec::new(),

      // Tier 1: Different defaults (fix automation blockers)
      bash_default_timeout_ms: Some( 3_600_000 ),  // 1 hour (vs 2 min standard)
      bash_max_timeout_ms: Some( 7_200_000 ),      // 2 hours (vs 10 min standard)
      auto_continue: Some( true ),                 // Enable automation (vs false standard)
      telemetry: Some( false ),                    // Disable telemetry (vs true standard)

      skip_permissions: false,

      // Tier 2 & 3: Standard defaults (security-sensitive, opt-in only)
      auto_approve_tools: None,  // Inherits standard: false
      action_mode: None,         // Inherits standard: Ask
      log_level: None,           // Inherits standard: Info
      temperature: None,         // Inherits standard: 1.0
      sandbox_mode: None,        // Inherits standard: true
      session_dir: None,         // Inherits standard: auto-detect
      top_p: None,               // Inherits standard: None
      top_k: None,               // Inherits standard: None
    }
  }

  /// Set working directory for Claude Code execution
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_working_directory("/home/user/project");
  /// ```
  #[inline]
  #[must_use]
  pub fn with_working_directory<P: Into<PathBuf>>( mut self, dir: P ) -> Self {
    self.working_directory = Some( dir.into() );
    self
  }

  /// Set maximum output tokens (default: 200,000)
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_max_output_tokens(200_000);
  /// ```
  #[inline]
  #[must_use]
  pub fn with_max_output_tokens( mut self, tokens: u32 ) -> Self {
    self.max_output_tokens = Some( tokens );
    self
  }

  /// Enable conversation continuation
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_continue_conversation(true);
  /// ```
  #[inline]
  #[must_use]
  pub fn with_continue_conversation( mut self, continue_: bool ) -> Self {
    self.continue_conversation = continue_;
    self
  }

  /// Set message to send to Claude
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_message("Explain this code");
  /// ```
  #[inline]
  #[must_use]
  pub fn with_message<S: Into<String>>( mut self, message: S ) -> Self {
    self.message = Some( message.into() );
    self
  }

  /// Add a single argument to the command
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_arg("--dangerously-skip-permissions");
  /// ```
  #[inline]
  #[must_use]
  pub fn with_arg<S: Into<String>>( mut self, arg: S ) -> Self {
    self.args.push( arg.into() );
    self
  }

  /// Add multiple arguments to the command
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_args(vec!["--dangerously-skip-permissions", "-c"]);
  /// ```
  #[inline]
  #[must_use]
  pub fn with_args<I, S>( mut self, args: I ) -> Self
  where
    I: IntoIterator<Item = S>,
    S: Into<String>,
  {
    self.args.extend( args.into_iter().map( Into::into ) );
    self
  }

  /// Set Claude model
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_model("claude-opus-4-5");
  /// ```
  #[inline]
  #[must_use]
  pub fn with_model<S: Into<String>>( mut self, model: S ) -> Self {
    self.args.push( "--model".to_string() );
    self.args.push( model.into() );
    self
  }

  /// Set API key via environment variable
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_api_key("sk-ant-...");
  /// ```
  #[inline]
  #[must_use]
  pub fn with_api_key<S: Into<String>>( mut self, key: S ) -> Self {
    self.args.push( "--api-key".to_string() );
    self.args.push( key.into() );
    self
  }

  /// Enable verbose output
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_verbose(true);
  /// ```
  #[inline]
  #[must_use]
  pub fn with_verbose( mut self, verbose: bool ) -> Self {
    if verbose {
      self.args.push( "--verbose".to_string() );
    }
    self
  }

  /// Set system prompt
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_system_prompt("You are a helpful coding assistant");
  /// ```
  #[inline]
  #[must_use]
  pub fn with_system_prompt<S: Into<String>>( mut self, prompt: S ) -> Self {
    self.args.push( "--system-prompt".to_string() );
    self.args.push( prompt.into() );
    self
  }

  /// Set default bash command timeout in milliseconds
  ///
  /// Default: 3,600,000 ms (1 hour). Standard default: 120,000 ms (2 minutes).
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_bash_timeout_ms(3_600_000);  // 1 hour
  /// ```
  #[inline]
  #[must_use]
  pub fn with_bash_timeout_ms( mut self, timeout_ms: u32 ) -> Self {
    self.bash_default_timeout_ms = Some( timeout_ms );
    self
  }

  /// Set maximum bash command timeout in milliseconds
  ///
  /// Default: 7,200,000 ms (2 hours). Standard default: 600,000 ms (10 minutes).
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_bash_max_timeout_ms(7_200_000);  // 2 hours
  /// ```
  #[inline]
  #[must_use]
  pub fn with_bash_max_timeout_ms( mut self, timeout_ms: u32 ) -> Self {
    self.bash_max_timeout_ms = Some( timeout_ms );
    self
  }

  /// Enable or disable auto-continue mode
  ///
  /// Default: true. Standard default: false.
  /// When true, enables programmatic automation without manual prompts.
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_auto_continue(true);
  /// ```
  #[inline]
  #[must_use]
  pub fn with_auto_continue( mut self, auto_continue: bool ) -> Self {
    self.auto_continue = Some( auto_continue );
    self
  }

  /// Enable or disable telemetry
  ///
  /// Default: false. Standard default: true.
  /// Disables usage data collection in automation contexts.
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_telemetry(false);
  /// ```
  #[inline]
  #[must_use]
  pub fn with_telemetry( mut self, telemetry: bool ) -> Self {
    self.telemetry = Some( telemetry );
    self
  }

  /// Enable or disable auto-approval of tool executions
  ///
  /// Default: false (inherits standard). Security-sensitive: requires explicit opt-in.
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_auto_approve_tools(false);  // Explicit denial
  /// ```
  #[inline]
  #[must_use]
  pub fn with_auto_approve_tools( mut self, approve: bool ) -> Self {
    self.auto_approve_tools = Some( approve );
    self
  }

  /// Set action mode for tool execution
  ///
  /// Default: `ActionMode::Ask` (inherits standard).
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::{ ClaudeCommand, ActionMode };
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_action_mode(ActionMode::Ask);
  /// ```
  #[inline]
  #[must_use]
  pub fn with_action_mode( mut self, mode: crate::types::ActionMode ) -> Self {
    self.action_mode = Some( mode );
    self
  }

  /// Set logging verbosity level
  ///
  /// Default: `LogLevel::Info` (inherits standard).
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::{ ClaudeCommand, LogLevel };
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_log_level(LogLevel::Debug);
  /// ```
  #[inline]
  #[must_use]
  pub fn with_log_level( mut self, level: crate::types::LogLevel ) -> Self {
    self.log_level = Some( level );
    self
  }

  /// Set model temperature
  ///
  /// Default: 1.0 (inherits standard). Range: 0.0 to 1.0.
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_temperature(0.7);
  /// ```
  #[inline]
  #[must_use]
  pub fn with_temperature( mut self, temperature: f64 ) -> Self {
    self.temperature = Some( temperature );
    self
  }

  /// Enable or disable sandbox mode
  ///
  /// Default: true (inherits standard).
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_sandbox_mode(true);
  /// ```
  #[inline]
  #[must_use]
  pub fn with_sandbox_mode( mut self, sandbox: bool ) -> Self {
    self.sandbox_mode = Some( sandbox );
    self
  }

  /// Set explicit session directory
  ///
  /// Default: None (auto-detect). Overrides default session storage location.
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_session_dir("/tmp/sessions");
  /// ```
  #[inline]
  #[must_use]
  pub fn with_session_dir<P: Into<PathBuf>>( mut self, dir: P ) -> Self {
    self.session_dir = Some( dir.into() );
    self
  }

  /// Set top-p sampling parameter
  ///
  /// Default: None (inherits standard). Range: 0.0 to 1.0.
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_top_p(0.9);
  /// ```
  #[inline]
  #[must_use]
  pub fn with_top_p( mut self, top_p: f64 ) -> Self {
    self.top_p = Some( top_p );
    self
  }

  /// Set top-k sampling parameter
  ///
  /// Default: None (inherits standard).
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_top_k(40);
  /// ```
  #[inline]
  #[must_use]
  pub fn with_top_k( mut self, top_k: u32 ) -> Self {
    self.top_k = Some( top_k );
    self
  }

  /// Enable `--dangerously-skip-permissions` flag
  ///
  /// When true, adds the `--dangerously-skip-permissions` flag to bypass
  /// tool permission prompts. Use with caution in automated pipelines only.
  ///
  /// Default: false.
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let cmd = ClaudeCommand::new()
  ///   .with_skip_permissions( true );
  /// ```
  #[inline]
  #[must_use]
  pub fn with_skip_permissions( mut self, skip: bool ) -> Self {
    self.skip_permissions = skip;
    self
  }

  /// Describe the command line that would be executed
  ///
  /// Returns a human-readable representation of the command. If a working
  /// directory is set, the first line is `cd /path/to/dir`. The last line
  /// is the `claude` invocation with all flags and arguments.
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let desc = ClaudeCommand::new()
  ///   .with_working_directory( "/tmp" )
  ///   .with_skip_permissions( true )
  ///   .with_message( "hello" )
  ///   .describe();
  ///
  /// assert!( desc.contains( "cd /tmp" ) );
  /// assert!( desc.contains( "--dangerously-skip-permissions" ) );
  /// ```
  #[inline]
  #[must_use]
  pub fn describe( &self ) -> String {
    let mut lines = Vec::new();

    if let Some( ref dir ) = self.working_directory {
      lines.push( format!( "cd {}", dir.display() ) );
    }

    let mut parts = vec![ "claude".to_string() ];

    if self.skip_permissions {
      parts.push( "--dangerously-skip-permissions".to_string() );
    }

    for arg in &self.args {
      parts.push( arg.clone() );
    }

    if self.continue_conversation {
      parts.push( "-c".to_string() );
    }

    if let Some( ref msg ) = self.message {
      // Fix(issue-describe-backslash-escape): Escape `\` before `"` to prevent malformed shell output
      // Root cause: Only `"` was escaped, not `\`. Messages containing `\"` produced `\\"` in output
      // which shell parses as a closing double-quote, breaking the command representation.
      // Pitfall: Always escape `\` first, then `"`, when quoting for double-quoted shell strings.
      let escaped = msg.replace( '\\', "\\\\" ).replace( '"', "\\\"" );
      parts.push( format!( "\"{escaped}\"" ) );
    }

    lines.push( parts.join( " " ) );
    lines.join( "\n" )
  }

  /// Describe environment variables that would be set
  ///
  /// Returns one `NAME=VALUE` line per configured environment variable.
  /// Only includes variables that have been explicitly set (via defaults
  /// or builder methods). Omits `None` values.
  ///
  /// # Example
  ///
  /// ```
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let env = ClaudeCommand::new().describe_env();
  ///
  /// assert!( env.contains( "CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000" ) );
  /// assert!( env.contains( "CLAUDE_CODE_BASH_TIMEOUT=3600000" ) );
  /// ```
  #[inline]
  #[must_use]
  pub fn describe_env( &self ) -> String {
    let mut lines = Vec::new();

    if let Some( tokens ) = self.max_output_tokens {
      lines.push( format!( "CLAUDE_CODE_MAX_OUTPUT_TOKENS={tokens}" ) );
    }
    if let Some( timeout ) = self.bash_default_timeout_ms {
      lines.push( format!( "CLAUDE_CODE_BASH_TIMEOUT={timeout}" ) );
    }
    if let Some( max_timeout ) = self.bash_max_timeout_ms {
      lines.push( format!( "CLAUDE_CODE_BASH_MAX_TIMEOUT={max_timeout}" ) );
    }
    if let Some( auto_continue ) = self.auto_continue {
      lines.push( format!( "CLAUDE_CODE_AUTO_CONTINUE={auto_continue}" ) );
    }
    if let Some( telemetry ) = self.telemetry {
      lines.push( format!( "CLAUDE_CODE_TELEMETRY={telemetry}" ) );
    }
    if let Some( approve ) = self.auto_approve_tools {
      lines.push( format!( "CLAUDE_CODE_AUTO_APPROVE_TOOLS={approve}" ) );
    }
    if let Some( mode ) = self.action_mode {
      lines.push( format!( "CLAUDE_CODE_ACTION_MODE={}", mode.as_str() ) );
    }
    if let Some( level ) = self.log_level {
      lines.push( format!( "CLAUDE_CODE_LOG_LEVEL={}", level.as_str() ) );
    }
    if let Some( temp ) = self.temperature {
      lines.push( format!( "CLAUDE_CODE_TEMPERATURE={temp}" ) );
    }
    if let Some( sandbox ) = self.sandbox_mode {
      lines.push( format!( "CLAUDE_CODE_SANDBOX_MODE={sandbox}" ) );
    }
    if let Some( ref dir ) = self.session_dir {
      lines.push( format!( "CLAUDE_CODE_SESSION_DIR={}", dir.display() ) );
    }
    if let Some( top_p ) = self.top_p {
      lines.push( format!( "CLAUDE_CODE_TOP_P={top_p}" ) );
    }
    if let Some( top_k ) = self.top_k {
      lines.push( format!( "CLAUDE_CODE_TOP_K={top_k}" ) );
    }

    lines.join( "\n" )
  }

  /// Execute the Claude Code command and capture output (non-interactive mode)
  ///
  /// This is the SINGLE execution point for non-interactive Claude Code process invocations.
  /// For interactive sessions, use [`execute_interactive`](Self::execute_interactive).
  ///
  /// Returns [`ExecutionOutput`](crate::ExecutionOutput) with stdout, stderr, and exit code.
  ///
  /// # Errors
  ///
  /// Returns error if Claude Code binary not found in PATH or process fails to spawn.
  ///
  /// # Example
  ///
  /// ```no_run
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let result = ClaudeCommand::new()
  ///   .with_max_output_tokens( 200_000 )
  ///   .execute()?;
  /// println!( "{}", result.stdout );
  /// # Ok::<(), Box<dyn std::error::Error>>(())
  /// ```
  #[inline]
  pub fn execute( &self ) -> Result< crate::types::ExecutionOutput > {
    let mut cmd = self.build_command();

    let output = cmd.output()
      .map_err( |e| Error::msg( format!( "Failed to execute Claude Code: {e}" ) ) )?;

    let stdout = String::from_utf8_lossy( &output.stdout ).to_string();
    let stderr = String::from_utf8_lossy( &output.stderr ).to_string();
    let exit_code = output.status.code().unwrap_or( -1 );

    Ok( crate::types::ExecutionOutput { stdout, stderr, exit_code } )
  }

  /// Execute the Claude Code command in interactive mode (TTY attached)
  ///
  /// This method allows Claude Code to take over the terminal for interactive sessions.
  /// Unlike [`execute`](Self::execute), this doesnt capture output and instead lets
  /// Claude Code directly interact with the user's terminal.
  ///
  /// # Errors
  ///
  /// Returns error if Claude Code binary not found in PATH or process fails to spawn.
  ///
  /// # Example
  ///
  /// ```no_run
  /// use claude_runner_core::ClaudeCommand;
  ///
  /// let exit_status = ClaudeCommand::new()
  ///   .with_max_output_tokens( 200_000 )
  ///   .execute_interactive()?;
  /// # Ok::<(), Box<dyn std::error::Error>>(())
  /// ```
  #[inline]
  pub fn execute_interactive( &self ) -> Result< std::process::ExitStatus > {
    let mut cmd = self.build_command();

    let status = cmd.status()
      .map_err( |e| Error::msg( format!( "Failed to execute Claude Code: {e}" ) ) )?;

    Ok( status )
  }

  /// Build the Command instance with all configured parameters
  ///
  /// SINGLE EXECUTION POINT: This is the ONLY location where `Command::new("claude")` appears
  #[inline]
  fn build_command( &self ) -> std::process::Command {
    use std::process::Command;

    // SINGLE EXECUTION POINT: This is the ONLY location where `Command::new("claude")` appears
    let mut cmd = Command::new( "claude" );

    // Set working directory if provided
    if let Some( ref dir ) = self.working_directory {
      cmd.current_dir( dir );
    }

    // Set max output tokens (fixes token limit bug: 32K → 200K)
    if let Some( tokens ) = self.max_output_tokens {
      cmd.env( "CLAUDE_CODE_MAX_OUTPUT_TOKENS", tokens.to_string() );
    }

    // Tier 1: Critical parameters with different defaults
    if let Some( timeout ) = self.bash_default_timeout_ms {
      cmd.env( "CLAUDE_CODE_BASH_TIMEOUT", timeout.to_string() );
    }

    if let Some( max_timeout ) = self.bash_max_timeout_ms {
      cmd.env( "CLAUDE_CODE_BASH_MAX_TIMEOUT", max_timeout.to_string() );
    }

    if let Some( auto_continue ) = self.auto_continue {
      cmd.env( "CLAUDE_CODE_AUTO_CONTINUE", auto_continue.to_string() );
    }

    if let Some( telemetry ) = self.telemetry {
      cmd.env( "CLAUDE_CODE_TELEMETRY", telemetry.to_string() );
    }

    // Tier 2: Essential parameters (security-sensitive)
    if let Some( approve ) = self.auto_approve_tools {
      cmd.env( "CLAUDE_CODE_AUTO_APPROVE_TOOLS", approve.to_string() );
    }

    if let Some( mode ) = self.action_mode {
      cmd.env( "CLAUDE_CODE_ACTION_MODE", mode.as_str() );
    }

    if let Some( level ) = self.log_level {
      cmd.env( "CLAUDE_CODE_LOG_LEVEL", level.as_str() );
    }

    if let Some( temp ) = self.temperature {
      cmd.env( "CLAUDE_CODE_TEMPERATURE", temp.to_string() );
    }

    // Tier 3: Optional parameters
    if let Some( sandbox ) = self.sandbox_mode {
      cmd.env( "CLAUDE_CODE_SANDBOX_MODE", sandbox.to_string() );
    }

    if let Some( ref dir ) = self.session_dir {
      cmd.env( "CLAUDE_CODE_SESSION_DIR", dir.to_string_lossy().as_ref() );
    }

    if let Some( top_p ) = self.top_p {
      cmd.env( "CLAUDE_CODE_TOP_P", top_p.to_string() );
    }

    if let Some( top_k ) = self.top_k {
      cmd.env( "CLAUDE_CODE_TOP_K", top_k.to_string() );
    }

    // Add skip-permissions flag before custom args
    if self.skip_permissions {
      cmd.arg( "--dangerously-skip-permissions" );
    }

    // Add custom arguments
    for arg in &self.args {
      cmd.arg( arg );
    }

    // Add continuation flag if requested
    if self.continue_conversation {
      cmd.arg( "-c" );
    }

    // Add message last if provided
    if let Some( ref msg ) = self.message {
      cmd.arg( msg );
    }

    cmd
  }
}

impl Default for ClaudeCommand {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

// ============================================================================
// Testing Support
// ============================================================================
//
// Note: Uses #[doc(hidden)] instead of #[cfg(test)] because integration tests
// in tests/ directory need access to this method. Integration tests compile
// against the public API and cannot see #[cfg(test)] items from the library.

impl ClaudeCommand {
  /// Test helper: Expose built Command for inspection
  ///
  /// **FOR TESTING ONLY** - This method allows integration tests to inspect
  /// the constructed Command without executing it.
  ///
  /// # Why Public?
  ///
  /// Integration tests (in `tests/` directory) need this to verify command
  /// construction. Cannot use `#[cfg(test)]` because integration tests compile
  /// against the public API.
  ///
  /// # Do Not Use in Production
  ///
  /// This method is marked `#[doc(hidden)]` to prevent it from appearing in
  /// public documentation. It should only be used by tests in this crate.
  #[ doc( hidden ) ]
  #[ inline ]
  #[ must_use ]
  pub fn build_command_for_test( &self ) -> std::process::Command {
    self.build_command()
  }
}
