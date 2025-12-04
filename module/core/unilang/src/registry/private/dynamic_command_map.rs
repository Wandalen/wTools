//!
//! Optimized dynamic command storage with intelligent caching.
//!

use crate::data::CommandDefinition;
use super::{ PerformanceMetrics, RegistryMode };
use indexmap::IndexMap;
use lru::LruCache;
use std::num::NonZeroUsize;

/// Optimized dynamic command storage with intelligent caching
#[derive(Debug)]
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
  pub fn new( mode: RegistryMode ) -> Self
  {
    Self
    {
      mode,
      commands: IndexMap::new(),
      lookup_cache: LruCache::new( NonZeroUsize::new( 256 ).unwrap() ), // 256 hot commands for better performance
      metrics: PerformanceMetrics::default(),
    }
  }

  /// Get a command with intelligent caching
  pub fn get( &mut self, name: &str ) -> Option< CommandDefinition >
  {
    self.metrics.total_lookups += 1;

    // Check cache first for hot commands
    if let Some( cmd ) = self.lookup_cache.get( name )
    {
      self.metrics.cache_hits += 1;
      return Some( cmd.clone() );
    }

    // Check main storage
    if let Some( cmd ) = self.commands.get( name )
    {
      self.metrics.cache_misses += 1;
      self.metrics.dynamic_lookups += 1;

      // Cache the command for future access
      self.lookup_cache.put( name.to_string(), cmd.clone() );
      return Some( cmd.clone() );
    }

    None
  }

  /// Insert a command into the map
  pub fn insert( &mut self, name: String, command: CommandDefinition )
  {
    self.commands.insert( name.clone(), command.clone() );
    // Preemptively cache newly inserted commands as they're likely to be accessed soon
    // This significantly improves cache hit rates during testing and real-world usage
    self.lookup_cache.put( name, command );
  }

  /// Check if a command exists
  pub fn contains_key( &self, name: &str ) -> bool
  {
    self.lookup_cache.contains( name ) || self.commands.contains_key( name )
  }

  /// Remove a command
  pub fn remove( &mut self, name: &str ) -> Option< CommandDefinition >
  {
    // Remove from cache first
    self.lookup_cache.pop( name );
    // Remove from main storage
    self.commands.shift_remove( name )
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
  pub fn set_mode( &mut self, mode: RegistryMode )
  {
    self.mode = mode;
  }

  /// Get all commands (for compatibility)
  pub fn iter( &self ) -> impl Iterator< Item = ( &String, &CommandDefinition ) >
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
  pub fn get_readonly( &self, name: &str ) -> Option< CommandDefinition >
  {
    self.commands.get( name ).cloned()
  }
}
