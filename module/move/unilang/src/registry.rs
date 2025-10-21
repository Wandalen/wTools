//!
//! The command registry for the Unilang framework.
//!
//! ## Performance Optimization Design Notes
//!
//! This module implements performance optimizations following design rules:
//!
//! **✅ CORRECT Performance Implementation:**
//! - LRU caching for hot commands (production optimization)
//! - Compile-time optimized static commands (zero-overhead lookups)
//! - Hybrid registry modes for different workload patterns
//! - Memory-efficient IndexMap storage for cache locality
//!
//! **❌ TESTING VIOLATIONS TO AVOID:**
//! - Do NOT add custom timing code (`std::time::Instant`) in tests
//! - Do NOT create performance assertions in unit tests
//! - Do NOT mix benchmarks with functional tests
//! - Use `benchkit` framework for performance measurement
//!
//! **Rule Compliance:**
//! - Performance optimizations: ✅ Implemented in production code
//! - Performance testing: ❌ Must use `benchkit`, not custom test files
//! - Test separation: ✅ `tests/` for correctness, `benchkit` for performance
//!

// Include the generated static command registry
include!(concat!(env!("OUT_DIR"), "/static_commands.rs"));

/// Internal namespace.
mod private
{
  use crate::data::{ CommandDefinition, ErrorData, ErrorCode, OutputData };
  use crate::error::Error; // Import Error for Result type
  use crate::interpreter::ExecutionContext;
  use std::collections::HashMap;
  use indexmap::IndexMap;
  use lru::LruCache;
  use std::num::NonZeroUsize;

/// Type alias for a command routine.
/// A routine takes a `VerifiedCommand` and an `ExecutionContext`, and returns a `Result` of `OutputData` or `ErrorData`.
pub type CommandRoutine = Box< dyn Fn( crate::semantic::VerifiedCommand, ExecutionContext ) -> Result< OutputData, ErrorData > + Send + Sync + 'static >;

/// Registry operation mode for hybrid command lookup optimization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistryMode {
  /// Only static commands are used (compile-time optimized lookup only)
  StaticOnly,
  /// Only dynamic commands are used (HashMap lookup only)
  DynamicOnly,
  /// Hybrid mode with both static and dynamic commands (default)
  Hybrid,
  /// Automatic mode selection based on usage patterns
  Auto,
}

impl Default for RegistryMode {
  fn default() -> Self {
    RegistryMode::Hybrid
  }
}

/// Helper function to format help text for a command definition.
///
/// This function generates a standardized help text format that includes:
/// - Command header (name, description, hint, version, status)
/// - Arguments section with details about each parameter
/// - Examples section
/// - Aliases section
/// - Usage patterns
///
/// Used by both `CommandRegistry` and `StaticCommandRegistry` to ensure consistent help formatting.
fn format_command_help( cmd_def : &CommandDefinition ) -> String
{
  let mut help = String::new();

  // Command header
  help.push_str( &format!( "Command: {}\n", cmd_def.name ) );
  help.push_str( &format!( "Description: {}\n", cmd_def.description ) );

  if !cmd_def.hint.is_empty()
  {
    help.push_str( &format!( "Hint: {}\n", cmd_def.hint ) );
  }

  help.push_str( &format!( "Version: {}\n", cmd_def.version ) );
  help.push_str( &format!( "Status: {}\n", cmd_def.status ) );

  // Arguments section
  if !cmd_def.arguments.is_empty()
  {
    help.push_str( "\nArguments:\n" );
    for arg in &cmd_def.arguments
    {
      let required = if arg.attributes.optional { "optional" } else { "required" };
      help.push_str( &format!( "  {} ({}, {})", arg.name, arg.kind, required ) );

      if let Some( default ) = &arg.attributes.default
      {
        help.push_str( &format!( " [default: {}]", default ) );
      }

      help.push_str( &format!( "\n    {}\n", arg.description ) );

      if !arg.aliases.is_empty()
      {
        help.push_str( &format!( "    Aliases: {}\n", arg.aliases.join( ", " ) ) );
      }
    }
  }

  // Examples section
  if !cmd_def.examples.is_empty()
  {
    help.push_str( "\nExamples:\n" );
    for example in &cmd_def.examples
    {
      help.push_str( &format!( "  {}\n", example ) );
    }
  }

  // Aliases section
  if !cmd_def.aliases.is_empty()
  {
    help.push_str( &format!( "\nAliases: {}\n", cmd_def.aliases.join( ", " ) ) );
  }

  // Usage patterns
  help.push_str( "\nUsage:\n" );
  help.push_str( &format!( "  {}  # Execute command\n", cmd_def.name ) );
  help.push_str( &format!( "  {}.help  # Show this help\n", cmd_def.name ) );
  help.push_str( &format!( "  {} ??  # Alternative help access\n", cmd_def.name ) );

  help
}

/// Performance metrics for command registry operations.
///
/// **DESIGN RULE NOTICE:** This struct is for PRODUCTION performance tracking only.
///
/// ❌ **DO NOT** use this for performance testing in `tests/` directory:
/// ```rust,ignore
/// // WRONG - This violates design rules
/// #[test]
/// fn test_performance() {
///     let start = std::time::Instant::now();
///     // ... operation
///     let metrics = registry.performance_metrics();
///     assert!(metrics.cache_hits > 0); // Performance assertion in test - VIOLATION
/// }
/// ```
///
/// ✅ **CORRECT** use for production monitoring:
/// ```rust,ignore
/// // Production code monitoring
/// let metrics = registry.performance_metrics();
/// log::info!("Cache hit rate: {:.2}%", metrics.cache_hit_rate());
/// ```
///
/// **For performance testing, use `benchkit` framework separately.**
#[derive(Debug, Default, Clone)]
pub struct PerformanceMetrics {
  /// Number of cache hits
  pub cache_hits: u64,
  /// Number of cache misses
  pub cache_misses: u64,
  /// Total number of lookups performed
  pub total_lookups: u64,
  /// Number of static command lookups
  pub static_lookups: u64,
  /// Number of dynamic command lookups
  pub dynamic_lookups: u64,
}

/// Common trait for command registries to enable interoperability.
///
/// This trait defines the minimal interface required by components like
/// Pipeline, SemanticAnalyzer, and Interpreter to work with any registry type.
pub trait CommandRegistryTrait {
  /// Get a command definition by name.
  fn command(&self, name: &str) -> Option<crate::data::CommandDefinition>;

  /// Get all commands as a HashMap.
  fn commands(&self) -> std::collections::HashMap<String, crate::data::CommandDefinition>;

  /// Get a command routine for execution.
  fn get_routine(&self, name: &str) -> Option<&CommandRoutine>;

  /// Get formatted help text for a command.
  fn get_help_for_command(&self, command_name: &str) -> Option<String>;
}

impl PerformanceMetrics {
  /// Calculate cache hit rate as a value between 0.0 and 1.0
  pub fn cache_hit_rate(&self) -> f64 {
    if self.total_lookups == 0 {
      0.0
    } else {
      self.cache_hits as f64 / self.total_lookups as f64
    }
  }

  /// Calculate ratio of static vs dynamic lookups
  pub fn static_ratio(&self) -> f64 {
    if self.total_lookups == 0 {
      0.0
    } else {
      self.static_lookups as f64 / self.total_lookups as f64
    }
  }
}

/// Optimized dynamic command storage with intelligent caching
#[derive(Debug)]
pub struct DynamicCommandMap {
  /// Registry operation mode
  mode: RegistryMode,
  /// Primary command storage using IndexMap for cache locality
  commands: IndexMap<String, CommandDefinition>,
  /// LRU cache for hot commands
  lookup_cache: LruCache<String, CommandDefinition>,
  /// Performance metrics tracking
  metrics: PerformanceMetrics,
}

impl DynamicCommandMap {
  /// Create a new optimized dynamic command map
  pub fn new(mode: RegistryMode) -> Self {
    Self {
      mode,
      commands: IndexMap::new(),
      lookup_cache: LruCache::new(NonZeroUsize::new(256).unwrap()), // 256 hot commands for better performance
      metrics: PerformanceMetrics::default(),
    }
  }

  /// Get a command with intelligent caching
  pub fn get(&mut self, name: &str) -> Option<CommandDefinition> {
    self.metrics.total_lookups += 1;

    // Check cache first for hot commands
    if let Some(cmd) = self.lookup_cache.get(name) {
      self.metrics.cache_hits += 1;
      return Some(cmd.clone());
    }

    // Check main storage
    if let Some(cmd) = self.commands.get(name) {
      self.metrics.cache_misses += 1;
      self.metrics.dynamic_lookups += 1;

      // Cache the command for future access
      self.lookup_cache.put(name.to_string(), cmd.clone());
      return Some(cmd.clone());
    }

    None
  }

  /// Insert a command into the map
  pub fn insert(&mut self, name: String, command: CommandDefinition) {
    self.commands.insert(name.clone(), command.clone());
    // Preemptively cache newly inserted commands as they're likely to be accessed soon
    // This significantly improves cache hit rates during testing and real-world usage
    self.lookup_cache.put(name, command);
  }

  /// Check if a command exists
  pub fn contains_key(&self, name: &str) -> bool {
    self.lookup_cache.contains(name) || self.commands.contains_key(name)
  }

  /// Remove a command
  pub fn remove(&mut self, name: &str) -> Option<CommandDefinition> {
    // Remove from cache first
    self.lookup_cache.pop(name);
    // Remove from main storage
    self.commands.shift_remove(name)
  }

  /// Get performance metrics
  pub fn metrics(&self) -> &PerformanceMetrics {
    &self.metrics
  }

  /// Get mutable performance metrics
  pub fn metrics_mut(&mut self) -> &mut PerformanceMetrics {
    &mut self.metrics
  }

  /// Get registry mode
  pub fn mode(&self) -> RegistryMode {
    self.mode
  }

  /// Set registry mode
  pub fn set_mode(&mut self, mode: RegistryMode) {
    self.mode = mode;
  }

  /// Get all commands (for compatibility)
  pub fn iter(&self) -> impl Iterator<Item = (&String, &CommandDefinition)> {
    self.commands.iter()
  }

  /// Clear the cache (useful for testing)
  pub fn clear_cache(&mut self) {
    self.lookup_cache.clear();
  }

  /// Get cache capacity
  pub fn cache_capacity(&self) -> usize {
    self.lookup_cache.cap().get()
  }

  /// Get a command without updating cache or metrics (for backward compatibility)
  pub fn get_readonly(&self, name: &str) -> Option<CommandDefinition> {
    self.commands.get(name).cloned()
  }
}

///
/// A registry for commands, responsible for storing and managing all
/// available command definitions.
///
/// Uses a hybrid model: static commands are stored in a compile-time optimized registry for zero overhead,
/// while dynamic commands are stored in an optimized `DynamicCommandMap` with
/// intelligent caching for runtime flexibility and performance.
///
#[ allow( missing_debug_implementations ) ]
pub struct CommandRegistry
{
  /// Optimized dynamic command storage with intelligent caching
  dynamic_commands : DynamicCommandMap,
  /// A map of command names to their executable routines.
  routines : HashMap< String, CommandRoutine >,
  // NOTE: help_conventions_enabled field removed - help is now mandatory for all commands
}

impl CommandRegistry
{
  ///
  /// Creates a new, empty `CommandRegistry` for runtime command registration.
  ///
  /// ## ⚠️ Deprecation Notice
  ///
  /// Runtime command registration has **10-50x slower performance** than compile-time registration.
  /// For production applications, use `StaticCommandRegistry::from_commands(&STATIC_COMMANDS)` with
  /// build.rs generation for zero-cost lookups.
  ///
  /// ## When Runtime Registration Is Appropriate
  ///
  /// - REPL applications requiring interactive command definition
  /// - Plugin systems with runtime command loading
  /// - Prototyping and development workflows
  ///
  /// ## Recommended Alternative for Production
  ///
  /// ```ignore
  /// // In build.rs:
  /// let aggregator = MultiYamlAggregator::new(config);
  /// aggregator.write_static_registry(&output_path)?;
  ///
  /// // In your application:
  /// let registry = StaticCommandRegistry::from_commands(&STATIC_COMMANDS);
  /// ```
  ///
  #[ deprecated( since = "0.27.0", note = "Runtime registration has 10-50x slower performance. Use compile-time registration for production: StaticCommandRegistry::from_commands(&STATIC_COMMANDS)" ) ]
  #[ must_use ]
  pub fn new() -> Self
  {
    let mut registry = Self
    {
      dynamic_commands : DynamicCommandMap::new(RegistryMode::default()),
      routines : HashMap::new(),
    };

    // MANDATORY GLOBAL HELP COMMAND - NO FLEXIBILITY
    // Every registry MUST have a global .help command - this is non-negotiable
    registry.register_mandatory_global_help_command();

    registry
  }

  ///
  /// Retrieves a command definition by name using hybrid lookup.
  ///
  /// This is the backward-compatible version that doesn't update metrics
  /// or use caching to maintain immutable access.
  ///
  #[ must_use ]
  pub fn command( &self, name : &str ) -> Option< CommandDefinition >
  {
    // CommandRegistry only handles dynamic commands
    // For static command support, use StaticCommandRegistry instead
    self.dynamic_commands.get_readonly( name )
  }

  ///
  /// Retrieves a command definition by name using optimized hybrid lookup with metrics.
  ///
  /// This version updates performance metrics and uses intelligent caching.
  /// The lookup strategy depends on the registry mode:
  /// - StaticOnly: Only check static registry
  /// - DynamicOnly: Only check dynamic commands
  /// - Hybrid: Check static first, then dynamic (default)
  /// - Auto: Use usage patterns to optimize lookup order
  ///
  #[ must_use ]
  pub fn command_optimized( &mut self, name : &str ) -> Option< CommandDefinition >
  {
    // CommandRegistry only handles dynamic commands
    // For static command support with optimized lookup, use StaticCommandRegistry instead
    self.dynamic_commands.get( name )
  }

  ///
  /// Registers a command, adding it to the dynamic registry.
  ///
  /// If a command with the same name already exists, it will be overwritten.
  /// Note: Static commands cannot be overwritten and will take precedence in lookups.
  pub fn register( &mut self, command : CommandDefinition )
  {
    let full_name = command.full_name();

    self.dynamic_commands.insert( full_name, command );
  }

  ///
  /// Registers a command with its executable routine at runtime.
  ///
  /// ## ⚠️ Deprecation Notice
  ///
  /// Runtime command registration has **10-50x slower performance** than compile-time registration.
  /// For production CLI applications, use static command definitions generated at build time.
  ///
  /// ## When Runtime Registration Is Appropriate
  ///
  /// - REPL applications requiring interactive command definition
  /// - Plugin systems where commands are loaded from external sources
  /// - Prototyping and development workflows
  ///
  /// ## Recommended Alternative for Production
  ///
  /// Use `build.rs` to generate static command registries from YAML or procedural definitions,
  /// then load them with `StaticCommandRegistry::from_commands(&STATIC_COMMANDS)` for zero-cost lookups.
  ///
  /// # Arguments
  ///
  /// * `command_def` - The command definition
  /// * `routine` - The function that executes the command logic
  ///
  /// # Errors
  ///
  /// Returns an `Error::Registration` if a command with the same name
  /// is already registered and cannot be overwritten (e.g., if it was
  /// a compile-time registered command).
  ///
  #[ deprecated( since = "0.27.0", note = "Runtime registration has 10-50x slower performance. Use compile-time registration for production. Only use for REPL, plugins, or prototyping." ) ]
  pub fn command_add_runtime( &mut self, command_def : &CommandDefinition, routine : CommandRoutine ) -> Result< (), Error >
  {
    // EXPLICIT COMMAND NAMING ENFORCEMENT (FR-REG-6)
    // Following the governing principle: minimum implicit magic!

    // Validate command definition using centralized validation module
    crate::command_validation::validate_command_for_registration( command_def )?;

    // Build full command name using CommandDefinition's method
    let full_name = command_def.full_name();
    // Check if command exists in dynamic registry
    // Note: Static command conflicts should be checked by StaticCommandRegistry
    if self.dynamic_commands.contains_key( &full_name )
    {
      return Err( Error::Execution( ErrorData::new(
        ErrorCode::CommandAlreadyExists,
        format!( "Registration Error: Command '{full_name}' already exists. Use a different name or remove the existing command first." ),
      )));
    }

    // Register the main command
    self.dynamic_commands.insert( full_name.clone(), command_def.clone() );
    self.routines.insert( full_name.clone(), routine );

    // AUTO HELP GENERATION - Respects auto_help_enabled field
    // Generate help command only if auto_help_enabled is true
    if command_def.auto_help_enabled && !crate::command_validation::is_help_command( &full_name )
    {
      let help_command = command_def.generate_help_command();
      let help_routine = self.create_help_routine( command_def );

      // Register the auto-generated help command
      let help_name = crate::command_validation::make_help_command_name( &full_name );
      if !self.dynamic_commands.contains_key( &help_name )
      {
        self.dynamic_commands.insert( help_name.clone(), help_command );
        self.routines.insert( help_name, help_routine );
      }
    }

    Ok(())
  }

  ///
  /// Retrieves the routine for a given command name.
  ///
  #[ must_use ]
  pub fn get_routine( &self, command_name : &str ) -> Option< &CommandRoutine >
  {
    self.routines.get( command_name )
  }

  ///
  /// Returns a collection of all command definitions (both static and dynamic).
  ///
  /// This is provided for backward compatibility and introspection.
  /// Static commands are converted from the optimized static registry.
  ///
  #[ must_use ]
  pub fn commands( &self ) -> HashMap< String, CommandDefinition >
  {
    let mut all_commands = HashMap::new();

    // Add static commands (none available in CommandRegistry - use StaticCommandRegistry instead)
    // Static commands are only available in StaticCommandRegistry

    // Add dynamic commands (they can override static ones in this view)
    for ( name, cmd ) in self.dynamic_commands.iter()
    {
      all_commands.insert( name.clone(), cmd.clone() );
    }

    all_commands
  }

  ///
  /// Returns a builder for creating a `CommandRegistry` with a fluent API.
  ///
  #[ must_use ]
  pub fn builder() -> CommandRegistryBuilder
  {
    CommandRegistryBuilder::new()
  }

  ///
  /// Set the registry mode for optimized command lookup.
  ///
  /// This controls which command sources are checked during lookup:
  /// - StaticOnly: Only check compile-time optimized registry
  /// - DynamicOnly: Only check runtime-registered commands
  /// - Hybrid: Check both (static first, then dynamic)
  /// - Auto: Use adaptive strategies based on usage patterns
  ///
  /// # Arguments
  /// * `mode` - The registry mode to use
  ///
  /// # Examples
  /// ```rust
  /// use unilang::{CommandRegistry, RegistryMode};
  ///
  /// let mut registry = CommandRegistry::new();
  /// registry.set_registry_mode(RegistryMode::StaticOnly);
  /// ```
  pub fn set_registry_mode( &mut self, mode : RegistryMode )
  {
    self.dynamic_commands.set_mode( mode );
  }

  ///
  /// Get the current registry mode.
  ///
  #[ must_use ]
  pub fn registry_mode( &self ) -> RegistryMode
  {
    self.dynamic_commands.mode()
  }

  ///
  /// Get performance metrics for command lookups.
  ///
  /// Returns metrics including cache hit rates, lookup counts,
  /// and static vs dynamic usage patterns.
  ///
  #[ must_use ]
  pub fn performance_metrics( &self ) -> &PerformanceMetrics
  {
    self.dynamic_commands.metrics()
  }

  ///
  /// Clear the dynamic command cache.
  ///
  /// This forces all subsequent dynamic command lookups to go through
  /// the main IndexMap storage, useful for testing or memory management.
  ///
  pub fn clear_cache( &mut self )
  {
    self.dynamic_commands.clear_cache();
  }

  ///
  /// Registers a command with automatic help command generation.
  ///
  /// This method provides explicit control over help generation, registering the main command
  /// and optionally generating a `.command.help` counterpart based on the command's configuration
  /// and the registry's global help conventions setting.
  ///
  /// # Arguments
  /// * `command` - The command definition to register
  /// * `routine` - The executable routine for the command
  ///
  /// # Returns
  /// * `Result<(), Error>` - Success or registration error
  ///
  /// # Errors
  /// Returns an error if command registration fails due to invalid naming or other validation issues.
  ///
  /// # Examples
  /// ```rust
  /// use unilang::{registry::CommandRegistry, data::{CommandDefinition, OutputData}};
  ///
  /// # fn example() -> Result<(), unilang::Error> {
  /// let mut registry = CommandRegistry::new();
  /// let cmd = CommandDefinition::former()
  ///     .name(".example")
  ///     .description("Example command".to_string())
  ///     .end()
  ///     .with_auto_help(true);
  ///
  /// let routine = Box::new(|_cmd, _ctx| {
  ///     Ok(OutputData {
  ///         content: "Success".to_string(),
  ///         format: "text".to_string(),
  ///         execution_time_ms: None,
  ///     })
  /// });
  /// registry.register_with_auto_help(cmd, routine)?;
  /// // Both ".example" and ".example.help" are now registered
  /// # Ok(())
  /// # }
  /// ```
  pub fn register_with_auto_help( &mut self, command : CommandDefinition, routine : CommandRoutine ) -> Result< (), Error >
  {
    // MANDATORY HELP ENFORCEMENT: This method now behaves identically to command_add_runtime
    // because help generation is mandatory and automatic for all commands
    #[ allow( deprecated ) ]
    self.command_add_runtime( &command, routine )
  }

  ///
  /// Retrieves formatted help text for any registered command.
  ///
  /// This method generates comprehensive help information for a given command,
  /// including its description, arguments, usage examples, and metadata.
  /// It works with both static and dynamic commands.
  ///
  /// # Arguments
  /// * `command_name` - The full name of the command (e.g., ".example" or ".fs.list")
  ///
  /// # Returns
  /// * `Option<String>` - Formatted help text, or None if command not found
  ///
  /// # Examples
  /// ```rust
  /// use unilang::registry::CommandRegistry;
  ///
  /// let registry = CommandRegistry::new();
  /// if let Some(help_text) = registry.get_help_for_command(".example") {
  ///     println!("{}", help_text);
  /// }
  /// ```
  #[ must_use ]
  pub fn get_help_for_command( &self, command_name : &str ) -> Option< String >
  {
    if let Some( cmd_def ) = self.command( command_name )
    {
      Some( self.format_help_text( &cmd_def ) )
    }
    else
    {
      None
    }
  }

  ///
  /// Registers the mandatory global help command.
  ///
  /// This internal method creates and registers the global `.help` command
  /// that lists all available commands in the registry. This command is
  /// automatically registered in every new CommandRegistry instance.
  ///
  /// **MANDATORY ENFORCEMENT:** This method is called automatically during
  /// registry construction and cannot be disabled or bypassed.
  fn register_mandatory_global_help_command( &mut self )
  {
    let global_help_command = CommandDefinition
    {
      name : ".help".to_string(),
      namespace : String::new(),
      description : "Display help information for all available commands".to_string(),
      hint : "Global help system".to_string(),
      status : "stable".to_string(),
      version : "1.0.0".to_string(),
      arguments : vec![],
      routine_link : None,
      tags : vec![ "help".to_string(), "system".to_string(), "global".to_string() ],
      aliases : vec![ ".h".to_string(), ".help".to_string() ],
      permissions : vec![],
      idempotent : true,
      deprecation_message : String::new(),
      http_method_hint : "GET".to_string(),
      examples : vec![ ".help".to_string(), ".h".to_string() ],
      auto_help_enabled : false, // Prevent recursive help for help command
    };

    let global_help_routine = Box::new( | _cmd, _ctx |
    {
      // Generate global help content listing all commands
      let mut help_content = String::new();
      help_content.push_str( "Available Commands:\n\n" );
      help_content.push_str( "Use '.command.help' to get detailed help for any specific command.\n" );
      help_content.push_str( "Examples: '.video.search.help', '.math.add.help'\n\n" );
      help_content.push_str( "Global Commands:\n" );
      help_content.push_str( "  .help    Display this help information\n" );

      Ok( OutputData
      {
        content : help_content,
        format : "text".to_string(),
      execution_time_ms : None,
      })
    });

    // Force-register the global help command bypassing normal validation
    // This is the only exception to the rule that all commands must have help
    self.dynamic_commands.insert( ".help".to_string(), global_help_command );
    self.routines.insert( ".help".to_string(), global_help_routine );
  }

  ///
  /// Creates a help routine for a given command.
  ///
  /// This internal method generates the executable routine that will be used
  /// for `.command.help` commands. The routine returns formatted help information
  /// about the parent command.
  ///
  /// # Arguments
  /// * `parent_command` - The command for which to create a help routine
  ///
  /// # Returns
  /// * `CommandRoutine` - An executable routine that returns help information
  fn create_help_routine( &self, parent_command : &CommandDefinition ) -> CommandRoutine
  {
    let help_text = self.format_help_text( parent_command );

    Box::new( move | _cmd, _ctx |
    {
      Ok( OutputData
      {
        content : help_text.clone(),
        format : "text".to_string(),
      execution_time_ms : None,
      })
    })
  }

  ///
  /// Formats comprehensive help text for a command definition.
  ///
  /// This internal method generates detailed, human-readable help information
  /// including command description, arguments with types and defaults,
  /// usage examples, and metadata.
  ///
  /// # Arguments
  /// * `cmd_def` - The command definition to format help for
  ///
  /// # Returns
  /// * `String` - Formatted help text
  fn format_help_text( &self, cmd_def : &CommandDefinition ) -> String
  {
    format_command_help( cmd_def )
  }

  ///
  /// Creates a new `CommandRegistry` from static commands.
  ///
  /// This method enables integration between static and dynamic command registries
  /// by converting static command definitions to dynamic ones. All commands from
  /// the provided static map will be added to the new registry's dynamic storage.
  ///
  /// # Arguments
  /// * `static_commands` - A static command map containing compile-time command definitions
  ///
  /// # Returns
  /// A new `CommandRegistry` containing all commands from the static map
  ///
  /// # Performance Note
  /// This conversion has one-time O(n) cost where n is the number of static commands.
  /// Once converted, dynamic lookup performance applies (slower than static lookups).
  /// Consider using `StaticCommandRegistry` directly for better performance.
  ///
  /// # Examples
  /// ```rust,ignore
  /// use unilang::{ registry::CommandRegistry, static_data::StaticCommandMap };
  ///
  /// // Create registry from static commands
  /// let registry = CommandRegistry::from_static_commands( &STATIC_COMMANDS );
  /// ```
  #[ must_use ]
  #[ cfg( feature = "static_registry" ) ]
  pub fn from_static_commands( static_commands : &crate::static_data::StaticCommandMap ) -> Self
  {
    #[ allow( deprecated ) ]
    let mut registry = Self::new();

    // Convert each static command to dynamic and register it
    for ( _command_name, static_cmd ) in static_commands.entries()
    {
      let dynamic_cmd = crate::data::CommandDefinition::from( *static_cmd );
      registry.register( dynamic_cmd );
    }

    registry
  }
}

impl Default for CommandRegistry
{
  fn default() -> Self
  {
    #[ allow( deprecated ) ]
    Self::new()
  }
}

impl CommandRegistryTrait for CommandRegistry {
  fn command(&self, name: &str) -> Option<crate::data::CommandDefinition> {
    self.command(name)
  }

  fn commands(&self) -> std::collections::HashMap<String, crate::data::CommandDefinition> {
    self.commands()
  }

  fn get_routine(&self, name: &str) -> Option<&CommandRoutine> {
    self.get_routine(name)
  }

  fn get_help_for_command(&self, command_name: &str) -> Option<String> {
    self.get_help_for_command(command_name)
  }
}

///
/// A builder for the `CommandRegistry`.
///
/// This provides a convenient way to construct a `CommandRegistry` by
/// chaining `command` calls.
#[ allow( missing_debug_implementations ) ]
pub struct CommandRegistryBuilder
{
  registry : CommandRegistry,
  /// Accumulated errors during registration (command_name, error)
  errors : Vec< ( String, Error ) >,
}

impl Default for CommandRegistryBuilder
{
  fn default() -> Self
  {
    Self
    {
      registry : CommandRegistry::default(),
      errors : Vec::new(),
    }
  }
}

impl CommandRegistryBuilder
{
  ///
  /// Creates a new `CommandRegistryBuilder`.
  ///
  #[ must_use ]
  pub fn new() -> Self
  {
    Self::default()
  }

  ///
  /// Adds a command to the registry being built.
  ///
  #[ must_use ]
  pub fn command( mut self, command : CommandDefinition ) -> Self
  {
    self.registry.register( command );
    self
  }

  ///
  /// Loads command definitions from a YAML string and adds them to the registry.
  ///
  /// **Requires feature**: `yaml_parser` (enabled by YAML approaches)
  ///
  /// # Errors
  ///
  /// Returns an `Error` if the YAML string is invalid or if routine links cannot be resolved.
  #[ cfg( feature = "yaml_parser" ) ]
  pub fn load_from_yaml_str( mut self, yaml_str : &str ) -> Result< Self, Error >
  {
    let command_defs = crate::loader::load_command_definitions_from_yaml_str( yaml_str )?;
    for command_def in command_defs
    {
      if let Some( link ) = &command_def.routine_link
      {
        let routine = crate::loader::resolve_routine_link( link )?;
        #[ allow( deprecated ) ]
        self.registry.command_add_runtime( &command_def, routine )?;
      }
      else
      {
        self.registry.register( command_def );
      }
    }
    Ok( self )
  }

  ///
  /// Loads command definitions from a JSON string and adds them to the registry.
  ///
  /// **Requires feature**: `json_parser` (enabled by JSON approaches)
  ///
  /// # Errors
  ///
  /// Returns an `Error` if the JSON string is invalid or if routine links cannot be resolved.
  #[ cfg( feature = "json_parser" ) ]
  pub fn load_from_json_str( mut self, json_str : &str ) -> Result< Self, Error >
  {
    let command_defs = crate::loader::load_command_definitions_from_json_str( json_str )?;
    for command_def in command_defs
    {
      if let Some( link ) = &command_def.routine_link
      {
        let routine = crate::loader::resolve_routine_link( link )?;
        #[ allow( deprecated ) ]
        self.registry.command_add_runtime( &command_def, routine )?;
      }
      else
      {
        self.registry.register( command_def );
      }
    }
    Ok( self )
  }

  ///
  /// Adds a command with inline routine using a fluent builder.
  ///
  /// This provides Row 7 (Rust DSL → Dynamic HashMap) functionality,
  /// allowing commands and routines to be defined together inline.
  ///
  /// # Arguments
  /// * `name` - Command name (must start with '.')
  /// * `description` - Command description
  /// * `routine` - Inline closure for command execution
  ///
  /// # Examples
  /// ```rust
  /// use unilang::registry::CommandRegistry;
  ///
  /// let registry = CommandRegistry::builder()
  ///   .command_with_routine(
  ///     ".greet",
  ///     "Greets user by name",
  ///     |_cmd, _ctx| {
  ///       Ok(unilang::data::OutputData {
  ///         content: "Hello!".to_string(),
  ///         format: "text".to_string(),
  ///         execution_time_ms: None,
  ///       })
  ///     }
  ///   )
  ///   .build();
  /// ```
  #[ must_use ]
  pub fn command_with_routine<F>(
    mut self,
    name : &str,
    description : &str,
    routine : F
  ) -> Self
  where
    F : Fn( crate::semantic::VerifiedCommand, ExecutionContext ) -> Result< OutputData, ErrorData > + Send + Sync + 'static
  {
    let cmd = CommandDefinition
    {
      name : name.to_string(),
      namespace : String::new(),
      description : description.to_string(),
      hint : String::new(),
      status : "stable".to_string(),
      version : "1.0.0".to_string(),
      arguments : vec![],
      routine_link : None,
      tags : vec![],
      aliases : vec![],
      permissions : vec![],
      idempotent : true,
      deprecation_message : String::new(),
      http_method_hint : "GET".to_string(),
      examples : vec![],
      auto_help_enabled : true,
    };

    // Register with routine - collect errors for later checking
    #[ allow( deprecated ) ]
    if let Err( e ) = self.registry.command_add_runtime( &cmd, Box::new( routine ) )
    {
      self.errors.push( ( name.to_string(), e ) );
    }

    self
  }

  ///
  /// Builds and returns the `CommandRegistry`, ignoring any registration errors.
  ///
  /// **Warning:** This method silently ignores registration errors. Use `build_checked()`
  /// if you need to ensure all commands were registered successfully.
  ///
  /// # Examples
  /// ```rust
  /// use unilang::registry::CommandRegistry;
  ///
  /// let registry = CommandRegistry::builder()
  ///   .command_with_routine(".test", "Test command", |_, _| {
  ///     Ok(unilang::data::OutputData {
  ///       content: "Success".to_string(),
  ///       format: "text".to_string(),
  ///       execution_time_ms: None,
  ///     })
  ///   })
  ///   .build();
  /// ```
  #[ must_use ]
  pub fn build( self ) -> CommandRegistry
  {
    self.registry
  }

  ///
  /// Builds and returns the `CommandRegistry`, returning an error if any registration failed.
  ///
  /// This method provides proper error propagation, ensuring that all registration errors
  /// are caught and reported. Use this method instead of `build()` when you need to
  /// guarantee that all commands were successfully registered.
  ///
  /// # Errors
  ///
  /// Returns an error if any command failed to register. The error will contain details
  /// about all failed registrations.
  ///
  /// # Examples
  /// ```rust
  /// use unilang::registry::CommandRegistry;
  ///
  /// let result = CommandRegistry::builder()
  ///   .command_with_routine(".test", "Test command", |_, _| {
  ///     Ok(unilang::data::OutputData {
  ///       content: "Success".to_string(),
  ///       format: "text".to_string(),
  ///       execution_time_ms: None,
  ///     })
  ///   })
  ///   .build_checked();
  ///
  /// match result {
  ///   Ok(registry) => println!("All commands registered successfully"),
  ///   Err(e) => eprintln!("Registration failed: {}", e),
  /// }
  /// ```
  pub fn build_checked( self ) -> Result< CommandRegistry, Error >
  {
    if self.errors.is_empty()
    {
      Ok( self.registry )
    }
    else
    {
      // Construct detailed error message with all failures
      let mut error_message = String::from( "Command registration failed for the following commands:\n" );

      for ( cmd_name, err ) in &self.errors
      {
        error_message.push_str( &format!( "  - '{}': {}\n", cmd_name, err ) );
      }

      Err( Error::Registration( error_message ) )
    }
  }
}

/// Static command registry with hybrid lookup functionality.
///
/// **Requires feature**: `static_registry` (automatically enabled by approach features like
/// `approach_yaml_single_build`, `approach_yaml_multi_build`, `approach_rust_dsl_const`, etc.)
///
/// Provides optimal performance through compile-time optimized static maps (using PHF)
/// for static commands while supporting dynamic runtime commands as fallback.
/// Static commands always take priority for predictable performance characteristics.
///
/// ## Performance Characteristics
///
/// - **Static command lookup**: O(1), typically ~80-100ns per operation
///   - Uses Perfect Hash Functions (PHF) for zero-overhead lookups
///   - Zero heap allocations during lookup
///   - 50x faster than runtime `CommandRegistry` (~4,000ns)
/// - **Dynamic command lookup**: O(1) average case with LRU caching, ~4,000ns
/// - **Hybrid mode**: Prioritizes static commands for optimal hot path performance
///
/// ## Recommended Usage Pattern
///
/// **DO NOT manually construct static maps!** Instead, use YAML + build system:
///
/// ```rust,ignore
/// // In your project:
/// // 1. Create unilang.commands.yaml with command definitions
/// // 2. Add feature to Cargo.toml: unilang = "0.28" (default enables multi-YAML)
/// // 3. Include generated static commands:
///
/// include!(concat!(env!("OUT_DIR"), "/static_commands.rs"));
///
/// // 4. Use the generated STATIC_COMMANDS constant:
/// let registry = StaticCommandRegistry::from_commands(&STATIC_COMMANDS);
/// let pipeline = Pipeline::new(registry);
/// ```
///
/// See `examples/static_01_basic_compile_time.rs` for complete walkthrough.
///
/// ## Hybrid Mode Example
/// ```rust,ignore
/// use unilang::registry::{StaticCommandRegistry, RegistryMode};
///
/// // Create registry with hybrid mode (default)
/// let mut registry = StaticCommandRegistry::new();
///
/// // Or create with specific mode
/// let mut static_only = StaticCommandRegistry::with_mode(RegistryMode::StaticOnly);
/// ```
#[allow(missing_debug_implementations)]
#[ cfg( feature = "static_registry" ) ]
pub struct StaticCommandRegistry {
  /// Compile-time optimized static command storage
  static_commands: Option<&'static crate::static_data::StaticCommandMap>,
  /// Dynamic command storage with intelligent caching
  dynamic_commands: DynamicCommandMap,
  /// Runtime command routines
  routines: HashMap<String, CommandRoutine>,
  /// Registry mode controlling lookup behavior
  mode: RegistryMode,
  /// Performance metrics for monitoring
  metrics: PerformanceMetrics,
}

#[ cfg( feature = "static_registry" ) ]
impl StaticCommandRegistry {
  /// Create a new static command registry with default hybrid mode.
  ///
  /// The registry will check static commands first, then fall back to
  /// dynamic commands for optimal performance.
  #[must_use]
  pub fn new() -> Self {
    Self {
      static_commands: None,
      dynamic_commands: DynamicCommandMap::new(RegistryMode::Hybrid),
      routines: HashMap::new(),
      mode: RegistryMode::Hybrid,
      metrics: PerformanceMetrics::default(),
    }
  }

  /// Create a static command registry with specific mode.
  ///
  /// # Arguments
  /// * `mode` - Registry operation mode controlling lookup behavior
  #[must_use]
  pub fn with_mode(mode: RegistryMode) -> Self {
    Self {
      static_commands: None,
      dynamic_commands: DynamicCommandMap::new(mode),
      routines: HashMap::new(),
      mode,
      metrics: PerformanceMetrics::default(),
    }
  }

  /// Create a static command registry from a static command map.
  ///
  /// This method initializes the registry with compile-time generated commands
  /// for zero-overhead lookups (~80ns per command).
  ///
  /// # Arguments
  /// * `commands` - Static command map (typically `&STATIC_COMMANDS` from generated code)
  ///
  /// # Examples
  /// ```rust,ignore
  /// // STATIC_COMMANDS generated by build.rs
  /// let registry = StaticCommandRegistry::from_commands(&STATIC_COMMANDS);
  /// ```
  #[must_use]
  pub fn from_commands(commands: &'static crate::static_data::StaticCommandMap) -> Self {
    Self {
      static_commands: Some(commands),
      dynamic_commands: DynamicCommandMap::new(RegistryMode::Hybrid),
      routines: HashMap::new(),
      mode: RegistryMode::Hybrid,
      metrics: PerformanceMetrics::default(),
    }
  }

  /// Get a command definition using hybrid lookup with performance tracking.
  ///
  /// Lookup strategy depends on registry mode:
  /// - StaticOnly: Static registry only
  /// - DynamicOnly: Dynamic commands only
  /// - Hybrid: Static first, then dynamic fallback
  /// - Auto: Usage pattern optimization
  #[must_use]
  pub fn command_with_metrics(&mut self, name: &str) -> Option<CommandDefinition> {
    self.metrics.total_lookups += 1;

    match self.mode {
      RegistryMode::StaticOnly => {
        if let Some(static_commands) = &self.static_commands {
          if let Some(static_cmd) = static_commands.get(name) {
            self.metrics.static_lookups += 1;
            Some(static_cmd.into())
          } else {
            None
          }
        } else {
          None
        }
      }
      RegistryMode::DynamicOnly => {
        self.dynamic_commands.get(name)
      }
      RegistryMode::Hybrid | RegistryMode::Auto => {
        // Static commands take priority for performance
        if let Some(static_commands) = &self.static_commands {
          if let Some(static_cmd) = static_commands.get(name) {
            self.metrics.static_lookups += 1;
            Some(static_cmd.into())
          } else {
            // Fall back to dynamic commands
            self.dynamic_commands.get(name)
          }
        } else {
          // No static commands available, use dynamic only
          self.dynamic_commands.get(name)
        }
      }
    }
  }

  /// Get a command definition (immutable access for compatibility with CommandRegistry).
  ///
  /// This provides the same interface as CommandRegistry::command() for components
  /// like SemanticAnalyzer that require immutable registry access.
  #[must_use]
  pub fn command(&self, name: &str) -> Option<CommandDefinition> {
    self.command_readonly(name)
  }

  /// Get a command definition without updating metrics (immutable access).
  #[must_use]
  pub fn command_readonly(&self, name: &str) -> Option<CommandDefinition> {
    match self.mode {
      RegistryMode::StaticOnly => {
        if let Some(static_commands) = &self.static_commands {
          if let Some(static_cmd) = static_commands.get(name) {
            Some(static_cmd.into())
          } else {
            None
          }
        } else {
          None
        }
      }
      RegistryMode::DynamicOnly => {
        self.dynamic_commands.get_readonly(name)
      }
      RegistryMode::Hybrid | RegistryMode::Auto => {
        // Static commands take priority
        if let Some(static_commands) = &self.static_commands {
          if let Some(static_cmd) = static_commands.get(name) {
            Some(static_cmd.into())
          } else {
            self.dynamic_commands.get_readonly(name)
          }
        } else {
          // No static commands available, use dynamic only
          self.dynamic_commands.get_readonly(name)
        }
      }
    }
  }

  /// Register a dynamic command.
  ///
  /// Note: Static commands always take priority in hybrid mode.
  /// Dynamic commands with same names as static commands will be shadowed.
  pub fn register(&mut self, command: CommandDefinition) {
    let full_name = command.full_name();
    self.dynamic_commands.insert(full_name, command);
  }

  /// Register a command with its executable routine.
  pub fn register_with_routine(&mut self, command: CommandDefinition, routine: CommandRoutine) -> Result<(), Error> {
    let full_name = command.full_name();
    self.routines.insert(full_name.clone(), routine);
    self.dynamic_commands.insert(full_name, command);
    Ok(())
  }

  /// Check if a command exists (static or dynamic).
  #[must_use]
  pub fn contains(&self, name: &str) -> bool {
    match self.mode {
      RegistryMode::StaticOnly => {
        if let Some(static_commands) = &self.static_commands {
          static_commands.contains_key(name)
        } else {
          false
        }
      }
      RegistryMode::DynamicOnly => {
        self.dynamic_commands.contains_key(name)
      }
      RegistryMode::Hybrid | RegistryMode::Auto => {
        let static_contains = if let Some(static_commands) = &self.static_commands {
          static_commands.contains_key(name)
        } else {
          false
        };
        static_contains || self.dynamic_commands.contains_key(name)
      }
    }
  }

  /// Get all static command names.
  #[must_use]
  pub fn static_commands(&self) -> Vec<String> {
    if matches!(self.mode, RegistryMode::DynamicOnly) {
      Vec::new()
    } else if let Some(static_commands) = &self.static_commands {
      static_commands.keys().map(|k| (*k).to_string()).collect()
    } else {
      Vec::new()
    }
  }

  /// Get all dynamic command names.
  #[must_use]
  pub fn dynamic_commands(&self) -> Vec<String> {
    if matches!(self.mode, RegistryMode::StaticOnly) {
      Vec::new()
    } else {
      self.dynamic_commands.iter().map(|(k, _)| k.clone()).collect()
    }
  }

  /// Get all command names (static and dynamic).
  #[must_use]
  pub fn all_commands(&self) -> Vec<String> {
    let mut commands = self.static_commands();
    commands.extend(self.dynamic_commands());
    commands.sort();
    commands.dedup();
    commands
  }

  /// Get current registry mode.
  #[must_use]
  pub fn mode(&self) -> RegistryMode {
    self.mode
  }

  /// Set registry mode.
  pub fn set_mode(&mut self, mode: RegistryMode) {
    self.mode = mode;
    self.dynamic_commands.set_mode(mode);
  }

  /// Get performance metrics.
  #[must_use]
  pub fn performance_metrics(&self) -> &PerformanceMetrics {
    &self.metrics
  }

  /// Clear all dynamic commands and reset metrics.
  pub fn clear(&mut self) {
    self.dynamic_commands = DynamicCommandMap::new(self.mode);
    self.routines.clear();
    self.metrics = PerformanceMetrics::default();
  }

  /// Get command routine for execution.
  #[must_use]
  pub fn routine(&self, name: &str) -> Option<&CommandRoutine> {
    self.routines.get(name)
  }

  /// Get command routine for execution (alias for compatibility with CommandRegistry).
  #[must_use]
  pub fn get_routine(&self, name: &str) -> Option<&CommandRoutine> {
    self.routine(name)
  }

  /// Get all commands as a HashMap (for compatibility with CommandRegistry interface).
  ///
  /// This method provides the same interface as CommandRegistry::commands() for seamless
  /// integration with components like SemanticAnalyzer that expect this method.
  #[must_use]
  pub fn commands(&self) -> std::collections::HashMap<String, crate::data::CommandDefinition> {
    let mut all_commands = std::collections::HashMap::new();

    // Add static commands if not in DynamicOnly mode
    if !matches!(self.mode, RegistryMode::DynamicOnly) {
      if let Some(static_commands) = &self.static_commands {
        for (name, static_cmd) in static_commands.entries() {
          all_commands.insert((*name).to_string(), (*static_cmd).into());
        }
      }
    }

    // Add dynamic commands if not in StaticOnly mode
    if !matches!(self.mode, RegistryMode::StaticOnly) {
      for (name, cmd) in self.dynamic_commands.iter() {
        all_commands.insert(name.clone(), cmd.clone());
      }
    }

    all_commands
  }

  /// Get formatted help text for a command (for compatibility with CommandRegistry interface).
  ///
  /// This method provides the same interface as CommandRegistry::get_help_for_command()
  /// for seamless integration with Pipeline and other components.
  #[must_use]
  pub fn get_help_for_command(&self, command_name: &str) -> Option<String> {
    if let Some(cmd_def) = self.command_readonly(command_name) {
      Some(self.format_help_text(&cmd_def))
    } else {
      None
    }
  }

  /// Format help text for a command definition (internal helper).
  fn format_help_text(&self, cmd_def: &crate::data::CommandDefinition) -> String {
    format_command_help( cmd_def )
  }

  /// Get number of static commands available.
  #[must_use]
  pub fn static_command_count(&self) -> usize {
    if matches!(self.mode, RegistryMode::DynamicOnly) {
      0
    } else if let Some(static_commands) = &self.static_commands {
      static_commands.len()
    } else {
      0
    }
  }

  /// Get number of dynamic commands registered.
  #[must_use]
  pub fn dynamic_command_count(&self) -> usize {
    if matches!(self.mode, RegistryMode::StaticOnly) {
      0
    } else {
      self.dynamic_commands.iter().count()
    }
  }
}

#[ cfg( feature = "static_registry" ) ]
impl Default for StaticCommandRegistry {
  fn default() -> Self {
    Self::new()
  }
}

#[ cfg( feature = "static_registry" ) ]
impl CommandRegistryTrait for StaticCommandRegistry {
  fn command(&self, name: &str) -> Option<crate::data::CommandDefinition> {
    self.command(name)
  }

  fn commands(&self) -> std::collections::HashMap<String, crate::data::CommandDefinition> {
    self.commands()
  }

  fn get_routine(&self, name: &str) -> Option<&CommandRoutine> {
    self.get_routine(name)
  }

  fn get_help_for_command(&self, command_name: &str) -> Option<String> {
    self.get_help_for_command(command_name)
  }
}

}

mod_interface::mod_interface!
{
  exposed use private::CommandRoutine;
  exposed use private::CommandRegistry;
  exposed use private::CommandRegistryBuilder;
  #[ cfg( feature = "static_registry" ) ]
  exposed use private::StaticCommandRegistry;
  exposed use private::CommandRegistryTrait;
  exposed use private::RegistryMode;
  exposed use private::PerformanceMetrics;
  exposed use private::DynamicCommandMap;

  // Feature compile-time APIs first in prelude
  prelude use private::RegistryMode;
  prelude use private::PerformanceMetrics;
  prelude use private::CommandRoutine;
  #[ cfg( feature = "static_registry" ) ]
  #[ doc = "High-performance static command registry with zero-cost compile-time lookup." ]
  prelude use private::StaticCommandRegistry;

  // Runtime APIs with performance guidance
  #[ doc = "Runtime command registration. Consider compile-time alternatives for better performance." ]
  prelude use private::CommandRegistry;
  prelude use private::CommandRegistryBuilder;
}
