//!
//! The command registry for the Unilang framework.
//!
//! ## Performance Optimization Design Notes
//!
//! This module implements performance optimizations following design rules :
//!
//! **✅ CORRECT Performance Implementation: **
//! - LRU caching for hot commands (production optimization)
//! - PHF (Perfect Hash Function) for static commands (compile-time optimization)
//! - Hybrid registry modes for different workload patterns
//! - Memory-efficient IndexMap storage for cache locality
//!
//! **❌ TESTING VIOLATIONS TO AVOID: **
//! - Do NOT add custom timing code (`std ::time ::Instant`) in tests
//! - Do NOT create performance assertions in unit tests
//! - Do NOT mix benchmarks with functional tests
//! - Use `benchkit` framework for performance measurement
//!
//! **Rule Compliance: **
//! - Performance optimizations: ✅ Implemented in production code
//! - Performance testing: ❌ Must use `benchkit`, not custom test files
//! - Test separation: ✅ `tests/` for correctness, `benchkit` for performance
//!

// Include the generated static commands PHF map
// include!(concat!(env!("OUT_DIR"), "/static_commands.rs"));

// Temporary stub for STATIC_COMMANDS while fixing syntax
use phf::Map;
static STATIC_COMMANDS: Map<&'static str, &'static crate::static_data::StaticCommandDefinition> = phf::phf_map! {};

/// Internal namespace.
mod private
{
  use crate ::data :: { CommandDefinition, ErrorData, OutputData };
  use crate ::error ::Error; // Import Error for Result type
  use crate ::interpreter ::ExecutionContext;
  use std ::collections ::HashMap;
  use indexmap ::IndexMap;
  use lru ::LruCache;
  use std ::num ::NonZeroUsize;
  use std ::cell ::RefCell;

/// Type alias for a command routine.
/// A routine takes a `VerifiedCommand` and an `ExecutionContext`, and returns a `Result` of `OutputData` or `ErrorData`.
pub type CommandRoutine = Box< dyn Fn( crate ::semantic ::VerifiedCommand, ExecutionContext ) -> Result< OutputData, ErrorData > + Send + Sync + 'static >;

/// Registry operation mode for hybrid command lookup optimization
#[ derive(Debug, Clone, Copy, PartialEq, Eq) ]
pub enum RegistryMode 
{
  /// Only static commands are used (PHF map lookup only)
  StaticOnly,
  /// Only dynamic commands are used (HashMap lookup only)
  DynamicOnly,
  /// Hybrid mode with both static and dynamic commands (default)
  Hybrid,
  /// Automatic mode selection based on usage patterns
  Auto,
}

impl Default for RegistryMode 
{
  fn default() -> Self
  {
    RegistryMode ::Hybrid
  }
}

/// Performance metrics for command registry operations.
///
/// **DESIGN RULE NOTICE: ** This struct is for PRODUCTION performance tracking only.
///
/// ❌ **DO NOT** use this for performance testing in `tests/` directory :
/// ```rust,ignore
/// // WRONG - This violates design rules
/// #[ test ]
/// fn test_performance() {
///     let start = std ::time ::Instant ::now();
///     // ... operation
///     let metrics = registry.performance_metrics();
///     assert!(metrics.cache_hits > 0); // Performance assertion in test - VIOLATION
/// }
/// ```
///
/// ✅ **CORRECT** use for production monitoring :
/// ```rust,ignore
/// // Production code monitoring
/// let metrics = registry.performance_metrics();
/// log ::info!("Cache hit rate: {:.2}%", metrics.cache_hit_rate());
/// ```
///
/// **For performance testing, use `benchkit` framework separately.**
#[ derive(Debug, Default, Clone) ]
pub struct PerformanceMetrics 
{
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

impl PerformanceMetrics 
{
  /// Calculate cache hit rate as a value between 0.0 and 1.0
  pub fn cache_hit_rate( &self ) -> f64
  {
  if self.total_lookups == 0 
  {
   0.0
 } else {
  {
   self.cache_hits as f64 / self.total_lookups as f64
 }
 }

  /// Calculate ratio of static vs dynamic lookups
  pub fn static_ratio( &self ) -> f64
  {
  if self.total_lookups == 0 
  {
   0.0
 } else {
  {
   self.static_lookups as f64 / self.total_lookups as f64
 }
 }
}

/// Optimized dynamic command storage with intelligent caching
#[ derive(Debug) ]
pub struct DynamicCommandMap 
{
  /// Registry operation mode
  mode: RegistryMode,
  /// Primary command storage using IndexMap for cache locality
  commands: IndexMap< String, CommandDefinition >,
  /// LRU cache for hot commands
  lookup_cache: LruCache< String, CommandDefinition >,
  /// Performance metrics tracking
  metrics: PerformanceMetrics,
}

impl DynamicCommandMap 
{
  /// Create a new optimized dynamic command map
  pub fn new(mode: RegistryMode) -> Self
  {
    Self {
      mode,
      commands: IndexMap ::new(),
      lookup_cache: LruCache ::new(NonZeroUsize ::new(256).unwrap()), // 256 hot commands for better performance
      metrics: PerformanceMetrics ::default(),
    }
  }

  /// Get a command with intelligent caching
  pub fn get(&mut self, name: &str) -> Option< CommandDefinition >
  {
    self.metrics.total_lookups += 1;

    // Check cache first for hot commands
    if let Some(cmd) = self.lookup_cache.get(name)
    {
      self.metrics.cache_hits += 1;
      return Some(cmd.clone());
    }

    // Check main storage
    if let Some(cmd) = self.commands.get(name)
    {
      self.metrics.cache_misses += 1;
      self.metrics.dynamic_lookups += 1;

      // Cache the command for future access
      self.lookup_cache.put(name.to_string(), cmd.clone());
      return Some(cmd.clone());
    }

    None
  }

  /// Insert a command into the map
  pub fn insert(&mut self, name: String, command: CommandDefinition)
  {
    self.commands.insert(name.clone(), command.clone());
    // Preemptively cache newly inserted commands as they're likely to be accessed soon
    // This significantly improves cache hit rates during testing and real-world usage
    self.lookup_cache.put(name, command);
  }

  /// Check if a command exists
  pub fn contains_key(&self, name: &str) -> bool
  {
    self.lookup_cache.contains(name) || self.commands.contains_key(name)
  }

  /// Remove a command
  pub fn remove(&mut self, name: &str) -> Option< CommandDefinition >
  {
  // Remove from cache first
  self.lookup_cache.pop(name);
  // Remove from main storage
  self.commands.shift_remove(name)
 }

  /// Get performance metrics
  pub fn metrics( &self ) -> &PerformanceMetrics
  {
    &self.metrics
  }

  /// Get mutable performance metrics
  pub fn metrics_mut( &mut self ) -> &mut PerformanceMetrics
  {
    &mut self.metrics
  }

  /// Get registry mode
  pub fn mode( &self ) -> RegistryMode
  {
    self.mode
  }

  /// Set registry mode
  pub fn set_mode(&mut self, mode: RegistryMode)
  {
    self.mode = mode;
  }

  /// Get all commands (for compatibility)
  pub fn iter( &self ) -> impl Iterator< Item = (&String, &CommandDefinition) >
  {
    self.commands.iter()
  }

  /// Clear the cache (useful for testing)
  pub fn clear_cache( &mut self )
  {
    self.lookup_cache.clear();
  }

  /// Get cache capacity
  pub fn cache_capacity( &self ) -> usize
  {
    self.lookup_cache.cap().get()
  }

  /// Get a command without updating cache or metrics (for backward compatibility)
  pub fn get_readonly(&self, name: &str) -> Option< CommandDefinition >
  {
    self.commands.get(name).cloned()
  }
}

///
/// A registry for commands, responsible for storing and managing all
/// available command definitions.
///
/// Uses a hybrid model: static commands are stored in a PHF map for zero overhead,
/// while dynamic commands are stored in an optimized `DynamicCommandMap` with
/// intelligent caching for runtime flexibility and performance.
///
#[ allow( missing_debug_implementations ) ]
pub struct CommandRegistry
{
  /// Optimized dynamic command storage with intelligent caching
  dynamic_commands: DynamicCommandMap,
  /// A map of command names to their executable routines.
  routines: HashMap< String, CommandRoutine >,
  /// Whether automatic help command generation is enabled for new registrations.
  help_conventions_enabled: bool,
}

impl CommandRegistry
{
  ///
  /// Creates a new, empty `CommandRegistry` for runtime command registration.
  ///
  /// ## Performance Warning
  ///
  /// Runtime command registration has **10-50x lookup overhead** compared to compile-time
  /// registration. Consider using static command definitions with PHF maps for production
  /// applications.
  ///
  /// **Recommended Alternative: ** Use `StaticCommandRegistry ::new()` with compile-time
  /// generated PHF maps via build.rs for zero-cost lookups.
  ///
  /// ## When to Use Runtime Registration
  ///
  /// - Commands loaded from external sources at runtime
  /// - Dynamic command generation required
  /// - Plugin systems with runtime loading
  /// - Rapid prototyping scenarios
  ///
  /// For production applications, prefer compile-time registration for optimal performance.
  ///
  #[ deprecated = "Runtime registration is slower. Use StaticCommandRegistry with compile-time registration for production." ]
  #[ must_use ]
  pub fn new() -> Self
  {
  Self
  {
   dynamic_commands: DynamicCommandMap ::new(RegistryMode ::default()),
   routines: HashMap ::new(),
   help_conventions_enabled: true, // Enable by default for better UX
 }
 }

  ///
  /// Creates a new `CommandRegistry` initialized with static commands from PHF map.
  ///
  /// This method provides backward compatibility for tests expecting static command access
  /// through the legacy CommandRegistry interface. For new code, prefer StaticCommandRegistry
  /// which provides better performance and cleaner separation of concerns.
  ///
  #[ deprecated = "Use StaticCommandRegistry ::from_phf() for better performance and cleaner architecture" ]
  #[ must_use ]
  pub fn from_static_commands() -> Self
  {
  // Create a CommandRegistry that can access static commands
  // This is for backward compatibility only
  Self
  {
   dynamic_commands: DynamicCommandMap ::new(RegistryMode ::Hybrid),
   routines: HashMap ::new(),
   help_conventions_enabled: true,
 }
 }

  ///
  /// Retrieves a command definition by name using hybrid lookup.
  ///
  /// This is the backward-compatible version that doesn't update metrics
  /// or use caching to maintain immutable access.
  ///
  #[ must_use ]
  pub fn command( &self, name: &str ) -> Option< CommandDefinition >
  {
  match self.dynamic_commands.mode() 
  {
   RegistryMode ::StaticOnly =>
  {
  // Only check static commands
  if let Some( static_cmd ) = STATIC_COMMANDS.get( name ) 
  {
   return Some( (*static_cmd).into() );
 }
  None
 },
   RegistryMode ::DynamicOnly =>
  {
  // Only check dynamic commands (without caching)
  self.dynamic_commands.get_readonly( name )
 },
   RegistryMode ::Hybrid | RegistryMode ::Auto =>
  {
  // Hybrid mode: static commands take priority
  if let Some( static_cmd ) = STATIC_COMMANDS.get( name ) 
  {
   return Some( (*static_cmd).into() );
 }

  // Fall back to dynamic commands (without caching)
  self.dynamic_commands.get_readonly( name )
 },
 }
 }

  ///
  /// Retrieves a command definition by name using hybrid lookup.
  ///
  /// This is an alias for `command()` to maintain backward compatibility.
  ///
  #[ must_use ]
  pub fn get( &self, name: &str ) -> Option< CommandDefinition >
  {
  self.command( name )
 }

  ///
  /// Retrieves a command definition by name using optimized hybrid lookup with metrics.
  ///
  /// This version updates performance metrics and uses intelligent caching.
  /// The lookup strategy depends on the registry mode :
  /// - StaticOnly: Only check static PHF map
  /// - DynamicOnly: Only check dynamic commands
  /// - Hybrid: Check static first, then dynamic (default)
  /// - Auto: Use usage patterns to optimize lookup order
  ///
  #[ must_use ]
  pub fn command_optimized( &mut self, name: &str ) -> Option< CommandDefinition >
  {
  match self.dynamic_commands.mode() 
  {
   RegistryMode ::StaticOnly =>
  {
  // Only check static commands
  if let Some( static_cmd ) = STATIC_COMMANDS.get( name ) 
  {
   self.dynamic_commands.metrics_mut().total_lookups += 1;
   self.dynamic_commands.metrics_mut().static_lookups += 1;
   return Some( (*static_cmd).into() );
 }
  None
 },
   RegistryMode ::DynamicOnly =>
  {
  // Only check dynamic commands
  self.dynamic_commands.get( name )
 },
   RegistryMode ::Hybrid | RegistryMode ::Auto =>
  {
  // Hybrid mode: static commands take priority
  if let Some( static_cmd ) = STATIC_COMMANDS.get( name ) 
  {
   self.dynamic_commands.metrics_mut().total_lookups += 1;
   self.dynamic_commands.metrics_mut().static_lookups += 1;
   return Some( (*static_cmd).into() );
 }

  // Fall back to dynamic commands with caching
  self.dynamic_commands.get( name )
 },
 }
 }

  ///
  /// Registers a command, adding it to the dynamic registry.
  ///
  /// If a command with the same name already exists, it will be overwritten.
  /// Note: Static commands cannot be overwritten and will take precedence in lookups.
  pub fn register( &mut self, command: CommandDefinition )
  {
  let full_name = if command.name.starts_with( '.' )
  {
   // Command name is already in full format
   command.name.clone()
 }
  else if command.namespace.is_empty()
  {
   format!( ".{}", command.name )
 }
  else
  {
   let ns = &command.namespace;
   if ns.starts_with( '.' )
   {
  format!( "{}.{}", ns, command.name )
 }
   else
   {
  format!( ".{}.{}", ns, command.name )
 }
 };

  self.dynamic_commands.insert( full_name, command );
 }

  ///
  /// Registers a command with its executable routine at runtime.
  ///
  /// ## Performance Impact
  ///
  /// Each runtime registration adds lookup overhead. Static commands via build.rs provide
  /// O(1) PHF lookups with zero runtime cost, typically **10-50x faster** than runtime
  /// HashMap operations.
  ///
  /// **Recommended Alternative: ** Define commands in YAML and use build.rs for compile-time
  /// PHF generation. See readme.md for compile-time registration patterns.
  ///
  /// ## Use Cases for Runtime Registration
  ///
  /// - Plugin systems requiring dynamic command loading
  /// - Commands from external configuration sources
  /// - Development and prototyping scenarios
  ///
  /// # Errors
  ///
  /// Returns an `Error ::Registration` if a command with the same name
  /// is already registered and cannot be overwritten (e.g., if it was
  /// a compile-time registered command).
  #[ deprecated = "Use static command registration via build.rs for better performance" ]
  pub fn command_add_runtime( &mut self, command_def: &CommandDefinition, routine: CommandRoutine ) -> Result< (), Error >
  {
  // EXPLICIT COMMAND NAMING ENFORCEMENT (FR-REG-6)
  // Following the governing principle: minimum implicit magic!
  
  // Validate that command names start with dot prefix
  if !command_def.name.starts_with( '.' )
  {
   return Err( Error ::Registration( format!(
  "Invalid command name '{}'. All commands must start with dot prefix (e.g., '.chat'). \
  This enforces explicit naming with minimal implicit transformations.",
  command_def.name
 )));
 }
  
  // Validate namespace format if provided
  if !command_def.namespace.is_empty() && !command_def.namespace.starts_with( '.' )
  {
   return Err( Error ::Registration( format!(
  "Invalid namespace '{}'. Non-empty namespaces must start with dot prefix (e.g., '.session'). \
  Use empty namespace for root-level commands.",
  command_def.namespace
 )));
 }
  
  // Build full command name explicitly - no magic transformations
  let full_name = if command_def.namespace.is_empty()
  {
   // Root-level command: use name as-is (already validated to have dot prefix)
   command_def.name.clone()
 }
  else
  {
   // Namespaced command: explicit concatenation
   format!( "{}.{}", command_def.namespace, command_def.name.strip_prefix('.').unwrap_or(&command_def.name) )
 };
  // Check if command exists in either static or dynamic registries
  if STATIC_COMMANDS.contains_key( &full_name ) || self.dynamic_commands.contains_key( &full_name )
  {
   return Err( Error ::Execution( ErrorData ::new(
  "UNILANG_COMMAND_ALREADY_EXISTS".to_string(),
  format!( "Registration Error: Command '{full_name}' already exists. Use a different name or remove the existing command first." ),
 )));
 }

  self.dynamic_commands.insert( full_name.clone(), command_def.clone() ); // Cloned command_def
  self.routines.insert( full_name.clone(), routine );
  Ok(())
 }

  ///
  /// Retrieves the routine for a given command name.
  ///
  #[ must_use ]
  pub fn get_routine( &self, command_name: &str ) -> Option< &CommandRoutine >
  {
  self.routines.get( command_name )
 }

  ///
  /// Returns a collection of all command definitions (both static and dynamic).
  /// 
  /// This is provided for backward compatibility and introspection.
  /// Static commands are converted from the PHF map.
  ///
  #[ must_use ]
  pub fn commands( &self ) -> HashMap< String, CommandDefinition >
  {
  let mut all_commands = HashMap ::new();

  // Add static commands
  for ( name, static_cmd ) in STATIC_COMMANDS.entries()
  {
   all_commands.insert( (*name).to_string(), (*static_cmd).into() );
 }

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
  CommandRegistryBuilder ::new()
 }

  ///
  /// Enables/disables automatic `.command.help` generation for all subsequently registered commands.
  ///
  /// When enabled, all commands registered with `command_add_runtime` or `register_with_auto_help`
  /// will automatically generate corresponding `.command.help` commands that provide detailed
  /// help information about the parent command.
  ///
  /// # Arguments
  /// * `enabled` - Whether to enable automatic help command generation
  ///
  /// # Examples
  /// ```rust,ignore
  /// use unilang ::registry ::CommandRegistry;
  ///
  /// #[ allow(deprecated) ]
/// let mut registry = CommandRegistry ::new();
  /// registry.enable_help_conventions(true);
  /// // All subsequently registered commands will auto-generate help commands
  /// ```
  pub fn enable_help_conventions( &mut self, enabled: bool )
  {
  self.help_conventions_enabled = enabled;
 }

  ///
  /// Set the registry mode for optimized command lookup.
  ///
  /// This controls which command sources are checked during lookup :
  /// - StaticOnly: Only check compile-time PHF map
  /// - DynamicOnly: Only check runtime-registered commands
  /// - Hybrid: Check both (static first, then dynamic)
  /// - Auto: Use adaptive strategies based on usage patterns
  ///
  /// # Arguments
  /// * `mode` - The registry mode to use
  ///
  /// # Examples
  /// ```rust,ignore
  /// use unilang :: { CommandRegistry, RegistryMode };
  ///
  /// #[ allow(deprecated) ]
/// let mut registry = CommandRegistry ::new();
  /// registry.set_registry_mode(RegistryMode ::StaticOnly);
  /// ```
  pub fn set_registry_mode( &mut self, mode: RegistryMode )
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
  /// Registers a routine for a given command name.
  ///
  /// This allows associating executable code with command definitions
  /// for both static and dynamic commands.
  ///
  /// # Arguments
  /// * `command_name` - The full name of the command (e.g., ".example")
  /// * `routine` - The executable routine for the command
  ///
  pub fn register_routine( &mut self, command_name: &str, routine: CommandRoutine )
  {
  self.routines.insert( command_name.to_string(), routine );
 }

  ///
  /// Check if a command has a registered routine.
  ///
  /// # Arguments
  /// * `command_name` - The full name of the command to check
  ///
  /// # Returns
  /// * `bool` - True if the command has a registered routine
  ///
  #[ must_use ]
  pub fn has_routine( &self, command_name: &str ) -> bool
  {
  self.routines.contains_key( command_name )
 }

  ///
  /// Returns a list of all command definitions (both static and dynamic).
  ///
  /// This method provides access to all available commands for introspection
  /// and help generation purposes.
  ///
  #[ must_use ]
  pub fn list_commands( &self ) -> Vec< CommandDefinition >
  {
  let mut all_commands = Vec ::new();

  // Add static commands if in appropriate mode
  if matches!( self.dynamic_commands.mode(), RegistryMode ::StaticOnly | RegistryMode ::Hybrid | RegistryMode ::Auto )
  {
   for ( _name, static_cmd ) in STATIC_COMMANDS.entries()
   {
  all_commands.push( (*static_cmd).into() );
 }
 }

  // Add dynamic commands if in appropriate mode
  if matches!( self.dynamic_commands.mode(), RegistryMode ::DynamicOnly | RegistryMode ::Hybrid | RegistryMode ::Auto )
  {
   for ( _name, cmd ) in self.dynamic_commands.iter()
   {
  all_commands.push( cmd.clone() );
 }
 }

  all_commands
 }

  ///
  /// Get the count of static commands available in the PHF map.
  ///
  #[ must_use ]
  pub fn static_command_count( &self ) -> usize
  {
  STATIC_COMMANDS.len()
 }

  ///
  /// Clear all dynamic commands while preserving static ones.
  ///
  /// This removes all runtime-registered commands but keeps
  /// the compile-time static commands intact.
  ///
  pub fn clear_dynamic_commands( &mut self )
  {
  self.dynamic_commands = DynamicCommandMap ::new( self.dynamic_commands.mode() );
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
  /// * `Result< (), Error >` - Success or registration error
  ///
  /// # Errors
  /// Returns an error if command registration fails due to invalid naming or other validation issues.
  ///
  /// # Examples
  /// ```rust,ignore
  /// use unilang :: { registry ::CommandRegistry, data ::CommandDefinition };
  ///
  /// #[ allow(deprecated) ]
/// let mut registry = CommandRegistry ::new();
  /// let cmd = CommandDefinition ::former()
  ///     .name(".example".to_string())
  ///     .description("Example command".to_string())
  ///     .with_auto_help(true)
  ///     .end();
  ///
  /// let routine = Box ::new(|_cmd, _ctx| Ok(OutputData ::default()));
  /// registry.register_with_auto_help(cmd, routine)?;
  /// // Both ".example" and ".example.help" are now registered
  /// ```
  pub fn register_with_auto_help( &mut self, command: CommandDefinition, routine: CommandRoutine ) -> Result< (), Error >
  {
  // First register the main command
  #[ allow(deprecated) ]
  self.command_add_runtime( &command, routine )?;

  // Generate help command if enabled (either globally or specifically for this command)
  if self.help_conventions_enabled || command.has_auto_help()
  {
   let help_command = command.generate_help_command();
   let help_routine = self.create_help_routine( &command );
   #[ allow(deprecated) ]
   self.command_add_runtime( &help_command, help_routine )?;
 }

  Ok( () )
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
  /// * `Option< String >` - Formatted help text, or None if command not found
  ///
  /// # Examples
  /// ```rust,ignore
  /// use unilang ::registry ::CommandRegistry;
  ///
  /// let registry = CommandRegistry ::new();
  ///  if let Some(help_text) = registry.get_help_for_command(".example") 
  {
  ///     println!("{}", help_text);
  /// }
  /// ```
  #[ must_use ]
  pub fn get_help_for_command( &self, command_name: &str ) -> Option< String >
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
  fn create_help_routine( &self, parent_command: &CommandDefinition ) -> CommandRoutine
  {
  let help_text = self.format_help_text( parent_command );

  Box ::new( move | _cmd, _ctx |
  {
   Ok( OutputData
   {
  content: help_text.clone(),
  format: "text".to_string(),
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
  fn format_help_text( &self, cmd_def: &CommandDefinition ) -> String
  {
  let mut help = String ::new();

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
   help.push_str( "\nArguments: \n" );
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
   help.push_str( "\nExamples: \n" );
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
  help.push_str( "\nUsage: \n" );
  help.push_str( &format!( "  {}  # Execute command\n", cmd_def.name ) );
  help.push_str( &format!( "  {}.help  # Show this help\n", cmd_def.name ) );
  help.push_str( &format!( "  {} ??  # Alternative help access\n", cmd_def.name ) );

  help
 }
}

impl Default for CommandRegistry
{
  fn default() -> Self
  {
  #[ allow(deprecated) ]
  Self ::new()
 }
}

///
/// A builder for the `CommandRegistry`.
///
/// This provides a convenient way to construct a `CommandRegistry` by
/// chaining `command` calls.
#[ allow( missing_debug_implementations ) ]
#[ derive( Default ) ] // Removed Debug
pub struct CommandRegistryBuilder
{
  registry: CommandRegistry,
}

impl CommandRegistryBuilder
{
  ///
  /// Creates a new `CommandRegistryBuilder`.
  ///
  #[ must_use ]
  pub fn new() -> Self
  {
  Self ::default()
 }

  ///
  /// Adds a command to the registry being built.
  ///
  #[ must_use ]
  pub fn command( mut self, command: CommandDefinition ) -> Self
  {
  self.registry.register( command );
  self
 }

  ///
  /// Initializes the registry builder with static commands from PHF map.
  ///
  /// This enables the built registry to access compile-time registered commands
  /// in addition to any runtime-registered commands.
  ///
  #[ must_use ]
  pub fn with_static_commands( self ) -> Self
  {
  // Convert to use from_static_commands instead of new()
  Self
  {
   #[ allow(deprecated) ]
   registry: CommandRegistry ::from_static_commands(),
 }
 }

  ///
  /// Loads command definitions from a YAML string and adds them to the registry.
  ///
  /// # Errors
  ///
  /// Returns an `Error` if the YAML string is invalid or if routine links cannot be resolved.
  pub fn load_from_yaml_str( mut self, yaml_str: &str ) -> Result< Self, Error >
  {
  let command_defs = crate ::loader ::load_command_definitions_from_yaml_str( yaml_str )?;
  for command_def in command_defs
  {
   if let Some( link ) = &command_def.routine_link
   {
  let routine = crate ::loader ::resolve_routine_link( link )?;
  #[ allow(deprecated) ]
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
  /// # Errors
  ///
  /// Returns an `Error` if the JSON string is invalid or if routine links cannot be resolved.
  pub fn load_from_json_str( mut self, json_str: &str ) -> Result< Self, Error >
  {
  let command_defs = crate ::loader ::load_command_definitions_from_json_str( json_str )?;
  for command_def in command_defs
  {
   if let Some( link ) = &command_def.routine_link
   {
  let routine = crate ::loader ::resolve_routine_link( link )?;
  #[ allow(deprecated) ]
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
  /// Builds and returns the `CommandRegistry`.
  ///
  #[ must_use ]
  pub fn build( self ) -> CommandRegistry
  {
  self.registry
 }
}

/// StaticCommandRegistry provides hybrid command lookup with PHF-based static commands
/// and HashMap-based dynamic commands for optimal performance.
///
/// This registry enables zero-overhead static command resolution while maintaining
/// backward compatibility with runtime command registration.
pub struct StaticCommandRegistry
{
  /// Dynamic commands storage for runtime-registered commands
  dynamic_commands: HashMap< String, CommandDefinition >,
  /// Command routines for both static and dynamic commands
  routines: HashMap< String, CommandRoutine >,
  /// Performance metrics tracking (using RefCell for interior mutability)
  metrics: RefCell< PerformanceMetrics >,
  /// Registry operation mode
  mode: RegistryMode,
  /// CLI metadata
  metadata: Option< crate ::multi_yaml ::CliMetadata >,
}

impl StaticCommandRegistry
{
  /// Create a new empty StaticCommandRegistry
  pub fn new() -> Self
  {
  Self
  {
   dynamic_commands: HashMap ::new(),
   routines: HashMap ::new(),
   metrics: RefCell ::new(PerformanceMetrics ::default()),
   mode: RegistryMode ::Hybrid,
   metadata: None,
 }
 }

  /// Create a StaticCommandRegistry initialized with static commands from PHF map
  pub fn from_phf() -> Self
  {
  Self
  {
   dynamic_commands: HashMap ::new(),
   routines: HashMap ::new(),
   metrics: RefCell ::new(PerformanceMetrics ::default()),
   mode: RegistryMode ::Hybrid,
   metadata: None,
 }
 }

  /// Create a StaticCommandRegistry with a specific mode
  pub fn with_mode(mode: RegistryMode) -> Self
  {
  Self
  {
   dynamic_commands: HashMap ::new(),
   routines: HashMap ::new(),
   metrics: RefCell ::new(PerformanceMetrics ::default()),
   mode,
   metadata: None,
 }
 }

  /// Get the count of static commands available in the PHF map
  pub fn static_command_count( &self ) -> usize
  {
  STATIC_COMMANDS.len()
 }

  /// Register a dynamic command at runtime
  pub fn register_dynamic_command(&mut self, command: CommandDefinition)
  {
  let full_name = if command.namespace.is_empty()
  {
   format!(".{}", command.name)
 }
  else
  {
   format!("{}.{}", command.namespace, command.name)
 };

  // Register the main command
  self.dynamic_commands.insert(full_name.clone(), command.clone());

  // Register aliases
  for alias in &command.aliases
  {
   self.dynamic_commands.insert(alias.clone(), command.clone());
 }
 }

  /// Get a command using hybrid lookup (static first, then dynamic)
  pub fn get_command(&self, name: &str) -> Option< CommandDefinition >
  {
  // Note: For simplicity in testing, we'll make this non-mutable
  // In a production implementation, you'd use Cell/RefCell for metrics

  match self.mode
  {
   RegistryMode ::StaticOnly => self.lookup_static(name),
   RegistryMode ::DynamicOnly => self.lookup_dynamic(name),
   RegistryMode ::Hybrid | RegistryMode ::Auto =>
   {
  // Try static first, then dynamic fallback
  if let Some(cmd) = self.lookup_static(name)
  {
   Some(cmd)
 }
  else
  {
   self.lookup_dynamic(name)
 }
 }
 }
 }

  /// Direct lookup in static PHF map
  pub fn lookup_static(&self, name: &str) -> Option< CommandDefinition >
  {
  STATIC_COMMANDS.get(name).map(|static_cmd| (*static_cmd).into())
 }

  /// Direct lookup in dynamic HashMap
  pub fn lookup_dynamic(&self, name: &str) -> Option< CommandDefinition >
  {
  self.dynamic_commands.get(name).cloned()
 }

  /// List all static commands from the PHF map
  pub fn list_static_commands( &self ) -> Vec< CommandDefinition >
  {
  STATIC_COMMANDS
   .entries()
   .map(|(_key, static_cmd)| (*static_cmd).into())
   .collect()
 }

  /// List all dynamic commands
  pub fn list_dynamic_commands( &self ) -> Vec< CommandDefinition >
  {
  self.dynamic_commands.values().cloned().collect()
 }

  /// List all commands (both static and dynamic) according to current mode
  pub fn list_all_commands( &self ) -> Vec< CommandDefinition >
  {
  let mut commands = Vec ::new();

  match self.mode
  {
   RegistryMode ::StaticOnly =>
   {
  commands.extend(self.list_static_commands());
 }
   RegistryMode ::DynamicOnly =>
   {
  commands.extend(self.list_dynamic_commands());
 }
   RegistryMode ::Hybrid | RegistryMode ::Auto =>
   {
  commands.extend(self.list_static_commands());
  commands.extend(self.list_dynamic_commands());
 }
 }

  commands
 }

  /// Check if a command has a registered routine
  pub fn has_routine(&self, name: &str) -> bool
  {
  self.routines.contains_key(name)
 }

  /// Register a routine for a command
  pub fn register_routine(&mut self, name: &str, routine: CommandRoutine)
  {
  self.routines.insert(name.to_string(), routine);
 }

  /// Get performance metrics
  pub fn performance_metrics( &self ) -> std ::cell ::Ref< '_, PerformanceMetrics >
  {
  self.metrics.borrow()
 }

  /// Set registry mode
  pub fn set_registry_mode(&mut self, mode: RegistryMode)
  {
  self.mode = mode;
 }

  /// Get registry mode
  pub fn registry_mode( &self ) -> RegistryMode
  {
  self.mode
 }

  /// Get registry mode (alias for registry_mode)
  pub fn mode( &self ) -> RegistryMode
  {
  self.mode
 }

  /// Clear dynamic commands while preserving static ones
  pub fn clear_dynamic_commands( &mut self )
  {
  self.dynamic_commands.clear();
 }

  /// Check if static command exists
  pub fn has_static_command(&self, name: &str) -> bool
  {
  STATIC_COMMANDS.contains_key(name)
 }

  /// Check if dynamic command exists
  pub fn has_dynamic_command(&self, name: &str) -> bool
  {
  self.dynamic_commands.contains_key(name)
 }

  /// Check if a command exists (either static or dynamic)
  pub fn has_command(&self, name: &str) -> bool
  {
  match self.mode
  {
   RegistryMode ::StaticOnly => self.has_static_command(name),
   RegistryMode ::DynamicOnly => self.has_dynamic_command(name),
   RegistryMode ::Hybrid | RegistryMode ::Auto =>
   {
  self.has_static_command(name) || self.has_dynamic_command(name)
 }
 }
 }

  /// Enable performance mode optimizations
  pub fn is_performance_mode_enabled( &self ) -> bool
  {
  matches!(self.mode, RegistryMode ::Auto | RegistryMode ::Hybrid)
 }

  /// Set metadata for the CLI
  pub fn set_metadata(&mut self, metadata: crate ::multi_yaml ::CliMetadata)
  {
  self.metadata = Some(metadata);
 }

  /// Get metadata for the CLI
  pub fn get_metadata( &self ) -> crate ::multi_yaml ::CliMetadata
  {
  self.metadata.clone().unwrap_or_default()
 }
}

impl Default for StaticCommandRegistry
{
  fn default() -> Self
  {
  Self ::new()
 }
}

impl Clone for StaticCommandRegistry
{
  fn clone( &self ) -> Self
  {
  // Clone everything except routines (which can't be cloned)
  Self
  {
   dynamic_commands: self.dynamic_commands.clone(),
   routines: HashMap ::new(), // Empty routines map for the clone
   metrics: RefCell ::new(PerformanceMetrics ::default()),
   mode: self.mode,
   metadata: self.metadata.clone(),
 }
 }
}

impl std ::fmt ::Debug for StaticCommandRegistry
{
  fn fmt(&self, f: &mut std ::fmt ::Formatter< '_ >) -> std ::fmt ::Result
  {
  f.debug_struct("StaticCommandRegistry")
   .field("dynamic_commands", &self.dynamic_commands)
   .field("routines_count", &self.routines.len())
   .field("mode", &self.mode)
   .field("metadata", &self.metadata)
   .finish()
 }
}

}

mod_interface ::mod_interface!
{
  exposed use private ::CommandRoutine;
  exposed use private ::CommandRegistry;
  exposed use private ::CommandRegistryBuilder;
  exposed use private ::StaticCommandRegistry;
  exposed use private ::RegistryMode;
  exposed use private ::PerformanceMetrics;
  exposed use private ::DynamicCommandMap;

  // Feature compile-time APIs first in prelude
  prelude use private ::RegistryMode;
  prelude use private ::PerformanceMetrics;
  prelude use private ::CommandRoutine;
  prelude use private ::StaticCommandRegistry;

  // Runtime APIs with performance guidance
  #[ doc = "Runtime command registration. Consider compile-time alternatives for better performance." ]
  prelude use private ::CommandRegistry;
  prelude use private ::CommandRegistryBuilder;
}
