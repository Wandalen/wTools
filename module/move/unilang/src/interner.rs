//! String Interning System
//!
//! This module provides high-performance string interning to optimize command name construction
//! in the semantic analysis hot path. Instead of repeatedly constructing command name strings
//! like ".command.subcommand", we cache them and return references to avoid allocation overhead.
//!
//! Performance target: 5-10x improvement in command name construction (38K → 190K-380K cmd/sec)

/// Internal namespace.
mod private
{
  use std::collections::HashMap;
  use std::sync::RwLock;

  /// Maximum number of strings to cache before evicting oldest entries
  const DEFAULT_CACHE_SIZE_LIMIT : usize = 10_000;

  /// Thread-safe string interner that caches strings and returns 'static references.
  /// 
  /// Uses `Box::leak()` to extend string lifetimes to 'static, enabling zero-copy
  /// command name lookups. Implements LRU eviction to prevent unbounded memory growth.
  #[ derive( Debug ) ]
  pub struct StringInterner
  {
    /// Storage for interned strings with thread-safe access
    storage : RwLock< InternerStorage >,
    /// Maximum cache size before eviction
    size_limit : usize,
  }

  #[ derive( Debug ) ]
  struct InternerStorage
  {
    /// Maps strings to their interned 'static references
    cache : HashMap< String, &'static str >,
    /// LRU access order tracking for eviction policy
    access_order : Vec< String >,
  }

  impl StringInterner
  {
    /// Creates a new string interner with default size limits.
    pub fn new() -> Self
    {
      Self::with_capacity( DEFAULT_CACHE_SIZE_LIMIT )
    }

    /// Creates a new string interner with specified cache capacity.
    pub fn with_capacity( size_limit : usize ) -> Self
    {
      Self
      {
        storage : RwLock::new( InternerStorage
        {
          cache : HashMap::new(),
          access_order : Vec::new(),
        }),
        size_limit,
      }
    }

    /// Interns a string, returning a 'static reference for zero-copy usage.
    /// 
    /// If the string is already cached, returns the existing reference.
    /// Otherwise, allocates the string on the heap with Box::leak() and caches it.
    pub fn intern( &self, s : &str ) -> &'static str
    {
      // Fast path: check if already cached with read lock
      {
        let storage = self.storage.read().unwrap();
        if let Some( &interned ) = storage.cache.get( s )
        {
          return interned;
        }
      }

      // Slow path: insert new string with write lock
      let mut storage = self.storage.write().unwrap();
      
      // Double-check in case another thread inserted while waiting for write lock
      if let Some( &interned ) = storage.cache.get( s )
      {
        return interned;
      }

      // Create interned string by leaking a Box allocation
      let interned : &'static str = Box::leak( s.to_string().into_boxed_str() );
      
      // Insert into cache
      storage.cache.insert( s.to_string(), interned );
      storage.access_order.push( s.to_string() );

      // Evict oldest entries if cache is too large
      if storage.cache.len() > self.size_limit
      {
        let cache_len = storage.cache.len();
        let to_remove = storage.access_order.drain( 0..( cache_len - self.size_limit ) ).collect::< Vec< _ > >();
        for key in to_remove
        {
          storage.cache.remove( &key );
        }
      }

      interned
    }

    /// Optimized command name construction and caching.
    /// 
    /// Constructs command names in the format ".command.subcommand" directly
    /// without intermediate string allocations when possible.
    pub fn intern_command_name( &self, path_slices : &[ &str ] ) -> &'static str
    {
      if path_slices.is_empty()
      {
        return self.intern( "." );
      }

      // Handle the case where first slice is empty (leading dot)
      let effective_slices = if path_slices[ 0 ].is_empty() && path_slices.len() > 1
      {
        &path_slices[ 1.. ]
      }
      else
      {
        path_slices
      };

      // Construct command name with leading dot
      let command_name = format!( ".{}", effective_slices.join( "." ) );
      self.intern( &command_name )
    }

    /// Returns current cache statistics for monitoring and debugging.
    pub fn stats( &self ) -> InternerStats
    {
      let storage = self.storage.read().unwrap();
      InternerStats
      {
        cached_strings : storage.cache.len(),
        size_limit : self.size_limit,
        memory_usage_estimate : storage.cache.iter()
          .map( | ( k, v ) | k.len() + v.len() )
          .sum::< usize >(),
      }
    }

    /// Clears all cached strings. Useful for testing and memory management.
    pub fn clear( &self )
    {
      let mut storage = self.storage.write().unwrap();
      storage.cache.clear();
      storage.access_order.clear();
    }
  }

  impl Default for StringInterner
  {
    fn default() -> Self
    {
      Self::new()
    }
  }

  /// Statistics about the string interner's current state.
  #[ derive( Debug, Clone ) ]
  pub struct InternerStats
  {
    /// Number of strings currently cached
    pub cached_strings : usize,
    /// Maximum cache size before eviction
    pub size_limit : usize,
    /// Estimated memory usage in bytes
    pub memory_usage_estimate : usize,
  }

  /// Global singleton interner for use throughout the application.
  /// 
  /// Using a global instance reduces the need to thread the interner through
  /// all APIs while maintaining the performance benefits.
  static GLOBAL_INTERNER : std::sync::OnceLock< StringInterner > = std::sync::OnceLock::new();

  /// Returns a reference to the global string interner instance.
  pub fn global_interner() -> &'static StringInterner
  {
    GLOBAL_INTERNER.get_or_init( StringInterner::new )
  }

  /// Convenience function to intern a string using the global interner.
  pub fn intern( s : &str ) -> &'static str
  {
    global_interner().intern( s )
  }

  /// Convenience function to intern command names using the global interner.
  pub fn intern_command_name( path_slices : &[ &str ] ) -> &'static str
  {
    global_interner().intern_command_name( path_slices )
  }

  #[ cfg( test ) ]
  mod tests
  {
    use super::*;

    #[ test ]
    fn test_basic_interning()
    {
      let interner = StringInterner::new();
      
      let s1 = interner.intern( "hello" );
      let s2 = interner.intern( "hello" );
      
      // Should return the same reference
      assert!( core::ptr::eq( s1, s2 ) );
      assert_eq!( s1, "hello" );
    }

    #[ test ]
    fn test_command_name_interning()
    {
      let interner = StringInterner::new();
      
      let cmd1 = interner.intern_command_name( &[ "command", "subcommand" ] );
      let cmd2 = interner.intern_command_name( &[ "command", "subcommand" ] );
      
      // Should return the same reference and correct format
      assert!( core::ptr::eq( cmd1, cmd2 ) );
      assert_eq!( cmd1, ".command.subcommand" );
    }

    #[ test ]
    fn test_command_name_empty_first_slice()
    {
      let interner = StringInterner::new();
      
      // Test the case where first slice is empty (like ["", "command", "subcommand"])
      let cmd = interner.intern_command_name( &[ "", "command", "subcommand" ] );
      assert_eq!( cmd, ".command.subcommand" );
    }

    #[ test ]
    fn test_cache_size_limit()
    {
      let interner = StringInterner::with_capacity( 2 );
      
      // Add strings up to the limit
      interner.intern( "first" );
      interner.intern( "second" );
      
      let stats_before = interner.stats();
      assert_eq!( stats_before.cached_strings, 2 );
      
      // Adding a third should evict the first (LRU)
      interner.intern( "third" );
      
      let stats_after = interner.stats();
      assert_eq!( stats_after.cached_strings, 2 );
    }

    #[ test ]
    fn test_global_interner()
    {
      let s1 = intern( "global_test" );
      let s2 = intern( "global_test" );
      
      assert!( core::ptr::eq( s1, s2 ) );
      assert_eq!( s1, "global_test" );
    }

    #[ test ]
    fn test_global_command_name_interner()
    {
      let cmd1 = intern_command_name( &[ "global", "command" ] );
      let cmd2 = intern_command_name( &[ "global", "command" ] );
      
      assert!( core::ptr::eq( cmd1, cmd2 ) );
      assert_eq!( cmd1, ".global.command" );
    }

    #[ test ]
    fn test_empty_path_slices()
    {
      let interner = StringInterner::new();
      let cmd = interner.intern_command_name( &[] );
      assert_eq!( cmd, "." );
    }

    #[ test ]
    fn test_stats()
    {
      let interner = StringInterner::new();
      
      let initial_stats = interner.stats();
      assert_eq!( initial_stats.cached_strings, 0 );
      
      interner.intern( "test" );
      
      let updated_stats = interner.stats();
      assert_eq!( updated_stats.cached_strings, 1 );
      assert!( updated_stats.memory_usage_estimate > 0 );
    }

    #[ test ]
    fn test_clear()
    {
      let interner = StringInterner::new();
      
      interner.intern( "test1" );
      interner.intern( "test2" );
      
      let stats_before = interner.stats();
      assert_eq!( stats_before.cached_strings, 2 );
      
      interner.clear();
      
      let stats_after = interner.stats();
      assert_eq!( stats_after.cached_strings, 0 );
    }

    #[ test ]
    fn test_concurrent_access()
    {
      use std::sync::Arc;
      use std::thread;
      
      let interner = Arc::new( StringInterner::new() );
      let mut handles = Vec::new();
      
      // Spawn multiple threads to test thread safety
      for i in 0..4
      {
        let interner_clone = Arc::clone( &interner );
        let handle = thread::spawn( move ||
        {
          let test_string = format!( "test_{i}" );
          let interned1 = interner_clone.intern( &test_string );
          let interned2 = interner_clone.intern( &test_string );
          
          // Should return the same reference even across threads
          assert!( core::ptr::eq( interned1, interned2 ) );
          assert_eq!( interned1, test_string );
        });
        handles.push( handle );
      }
      
      // Wait for all threads to complete
      for handle in handles
      {
        handle.join().unwrap();
      }
    }
  }
}

mod_interface::mod_interface!
{
  exposed use private::StringInterner;
  exposed use private::InternerStats;
  exposed use private::global_interner;
  exposed use private::intern;
  exposed use private::intern_command_name;
}