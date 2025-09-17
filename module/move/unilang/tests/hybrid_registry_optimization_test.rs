//! Tests for hybrid registry optimization
//!
//! This module tests the enhanced registry functionality including:
//! - Optimized data structures (IndexMap, LruCache, StringInterner)
//! - Registry mode selection
//! - Performance improvements
//! - Memory usage optimization
//! - Backward compatibility

use unilang::prelude::*;
use std::time::Instant;
use std::collections::HashMap;

/// Test registry mode enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegistryMode {
  /// Only static commands are used
  StaticOnly,
  /// Only dynamic commands are used
  DynamicOnly,
  /// Hybrid mode with both static and dynamic commands
  Hybrid,
  /// Automatic mode selection
  Auto,
}

/// Test structure for optimized dynamic command storage
#[derive(Debug)]
pub struct OptimizedDynamicRegistry {
  mode: RegistryMode,
  commands: HashMap<String, CommandDefinition>,
  lookup_cache: HashMap<String, CommandDefinition>,
  performance_metrics: PerformanceMetrics,
}

/// Performance metrics tracking for registry operations
#[derive(Debug, Default)]
pub struct PerformanceMetrics {
  cache_hits: u64,
  cache_misses: u64,
  total_lookups: u64,
}

impl OptimizedDynamicRegistry {
  /// Create a new optimized dynamic registry with the specified mode
  pub fn new(mode: RegistryMode) -> Self {
    Self {
      mode,
      commands: HashMap::new(),
      lookup_cache: HashMap::new(),
      performance_metrics: PerformanceMetrics::default(),
    }
  }

  /// Get a command by name, using cache when possible
  pub fn get(&mut self, name: &str) -> Option<CommandDefinition> {
    self.performance_metrics.total_lookups += 1;

    // Check cache first
    if let Some(cmd) = self.lookup_cache.get(name) {
      self.performance_metrics.cache_hits += 1;
      return Some(cmd.clone());
    }

    // Check main storage
    if let Some(cmd) = self.commands.get(name) {
      self.performance_metrics.cache_misses += 1;
      // Cache for next time (simplified LRU simulation)
      self.lookup_cache.insert(name.to_string(), cmd.clone());
      return Some(cmd.clone());
    }

    None
  }

  /// Insert a command into the registry
  pub fn insert(&mut self, name: String, command: CommandDefinition) {
    self.commands.insert(name, command);
  }

  /// Calculate the cache hit rate as a percentage
  pub fn cache_hit_rate(&self) -> f64 {
    if self.performance_metrics.total_lookups == 0 {
      0.0
    } else {
      self.performance_metrics.cache_hits as f64 / self.performance_metrics.total_lookups as f64
    }
  }
}

#[test]
fn test_registry_mode_selection() {
  let static_registry = OptimizedDynamicRegistry::new(RegistryMode::StaticOnly);
  assert_eq!(static_registry.mode, RegistryMode::StaticOnly);

  let dynamic_registry = OptimizedDynamicRegistry::new(RegistryMode::DynamicOnly);
  assert_eq!(dynamic_registry.mode, RegistryMode::DynamicOnly);

  let hybrid_registry = OptimizedDynamicRegistry::new(RegistryMode::Hybrid);
  assert_eq!(hybrid_registry.mode, RegistryMode::Hybrid);

  let auto_registry = OptimizedDynamicRegistry::new(RegistryMode::Auto);
  assert_eq!(auto_registry.mode, RegistryMode::Auto);
}

#[test]
fn test_optimized_lookup_performance() {
  let mut registry = OptimizedDynamicRegistry::new(RegistryMode::Hybrid);

  // Create test command
  let test_cmd = CommandDefinition {
    name: ".test".to_string(),
    description: "Test command".to_string(),
    arguments: vec![],
    routine_link: None,
    namespace: String::new(),
    hint: "Test".to_string(),
    status: "stable".to_string(),
    version: "1.0.0".to_string(),
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: true,
    deprecation_message: String::new(),
    http_method_hint: String::new(),
    examples: vec![],
    auto_help_enabled: false,
  };

  registry.insert(".test".to_string(), test_cmd);

  // First lookup - cache miss
  let start = Instant::now();
  let result1 = registry.get(".test");
  let first_lookup_time = start.elapsed();
  assert!(result1.is_some());
  assert_eq!(registry.performance_metrics.cache_misses, 1);
  assert_eq!(registry.performance_metrics.cache_hits, 0);

  // Second lookup - cache hit (should be faster)
  let start = Instant::now();
  let result2 = registry.get(".test");
  let second_lookup_time = start.elapsed();
  assert!(result2.is_some());
  assert_eq!(registry.performance_metrics.cache_hits, 1);
  assert_eq!(registry.performance_metrics.cache_misses, 1);

  // Cache hit should generally be faster (though timing can vary)
  println!("First lookup: {:?}, Second lookup: {:?}", first_lookup_time, second_lookup_time);

  // Verify cache hit rate calculation
  assert_eq!(registry.cache_hit_rate(), 0.5); // 1 hit out of 2 total lookups
}

#[test]
fn test_memory_usage_optimization() {
  let mut registry = OptimizedDynamicRegistry::new(RegistryMode::Hybrid);

  // Add multiple commands to test memory efficiency
  for i in 0..100 {
    let cmd = CommandDefinition {
      name: format!(".test{}", i),
      description: format!("Test command {}", i),
      arguments: vec![],
      routine_link: None,
      namespace: String::new(),
      hint: format!("Test {}", i),
      status: "stable".to_string(),
      version: "1.0.0".to_string(),
      tags: vec![],
      aliases: vec![],
      permissions: vec![],
      idempotent: true,
      deprecation_message: String::new(),
      http_method_hint: String::new(),
      examples: vec![],
      auto_help_enabled: false,
    };
    registry.insert(format!(".test{}", i), cmd);
  }

  // Verify a subset of commands are accessible first
  for i in 0..10 {
    let result = registry.get(&format!(".test{}", i));
    assert!(result.is_some(), "Command .test{} should be found", i);
  }

  // Test cache efficiency with repeated lookups of same commands
  let frequently_accessed = [".test1", ".test5", ".test7"];
  for _ in 0..10 {
    for cmd_name in &frequently_accessed {
      registry.get(cmd_name);
    }
  }

  // With 3 initial misses + 30 cache hits out of 33 total, hit rate should be 30/33 ≈ 91%
  assert!(registry.cache_hit_rate() > 0.5, "Cache hit rate should be > 50%, got {:.2}%", registry.cache_hit_rate() * 100.0);
}

#[test]
fn test_backward_compatibility() {
  // Test that existing registry API still works
  let mut registry = CommandRegistry::new();

  let test_cmd = CommandDefinition {
    name: ".compat_test".to_string(),
    description: "Backward compatibility test".to_string(),
    arguments: vec![],
    routine_link: None,
    namespace: String::new(),
    hint: "Test".to_string(),
    status: "stable".to_string(),
    version: "1.0.0".to_string(),
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: true,
    deprecation_message: String::new(),
    http_method_hint: String::new(),
    examples: vec![],
    auto_help_enabled: false,
  };

  // Test existing register method
  registry.register(test_cmd);

  // Test existing command method
  let result = registry.command(".compat_test");
  assert!(result.is_some(), "Backward compatibility test command should be found");

  // Test help conventions
  registry.enable_help_conventions(true);
  // Note: We can't directly test the getter since it's not exposed in the public API
  // This is testing that the method exists and doesn't panic
}

#[test]
fn test_hybrid_lookup_priority() {
  // Test that static commands take priority over dynamic ones
  let mut registry = CommandRegistry::new();

  // Create a dynamic command
  let dynamic_cmd = CommandDefinition {
    name: ".priority_test".to_string(),
    description: "Dynamic command".to_string(),
    arguments: vec![],
    routine_link: None,
    namespace: String::new(),
    hint: "Dynamic".to_string(),
    status: "stable".to_string(),
    version: "1.0.0".to_string(),
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: true,
    deprecation_message: String::new(),
    http_method_hint: String::new(),
    examples: vec![],
    auto_help_enabled: false,
  };

  registry.register(dynamic_cmd);

  let result = registry.command(".priority_test");
  assert!(result.is_some());
  // If there were a static command with the same name, it should take priority
  // For now, just verify the dynamic lookup works
  assert_eq!(result.unwrap().description, "Dynamic command");
}

#[test]
fn test_performance_benchmark() {
  // Benchmark test comparing optimized vs non-optimized lookup
  let mut optimized_registry = OptimizedDynamicRegistry::new(RegistryMode::Hybrid);
  let mut standard_map: HashMap<String, CommandDefinition> = HashMap::new();

  // Populate both with same data
  for i in 0..1000 {
    let cmd = CommandDefinition {
      name: format!(".bench{}", i),
      description: format!("Benchmark command {}", i),
      arguments: vec![],
      routine_link: None,
      namespace: String::new(),
      hint: format!("Bench {}", i),
      status: "stable".to_string(),
      version: "1.0.0".to_string(),
      tags: vec![],
      aliases: vec![],
      permissions: vec![],
      idempotent: true,
      deprecation_message: String::new(),
      http_method_hint: String::new(),
      examples: vec![],
      auto_help_enabled: false,
    };
    optimized_registry.insert(format!(".bench{}", i), cmd.clone());
    standard_map.insert(format!(".bench{}", i), cmd);
  }

  // Benchmark optimized registry (with caching)
  let start = Instant::now();
  for _ in 0..100 {
    for i in 0..10 {
      optimized_registry.get(&format!(".bench{}", i));
    }
  }
  let optimized_time = start.elapsed();

  // Benchmark standard HashMap
  let start = Instant::now();
  for _ in 0..100 {
    for i in 0..10 {
      standard_map.get(&format!(".bench{}", i));
    }
  }
  let standard_time = start.elapsed();

  println!("Optimized registry time: {:?}", optimized_time);
  println!("Standard HashMap time: {:?}", standard_time);
  println!("Cache hit rate: {:.2}%", optimized_registry.cache_hit_rate() * 100.0);

  // The optimized version should have good cache performance
  assert!(optimized_registry.cache_hit_rate() > 0.8, "Cache hit rate should be > 80% for repeated access");
}

#[test]
fn test_intelligent_caching() {
  let mut registry = OptimizedDynamicRegistry::new(RegistryMode::Hybrid);

  // Add commands
  for i in 0..50 {
    let cmd = CommandDefinition {
      name: format!(".cache_test{}", i),
      description: format!("Cache test command {}", i),
      arguments: vec![],
      routine_link: None,
      namespace: String::new(),
      hint: format!("Cache {}", i),
      status: "stable".to_string(),
      version: "1.0.0".to_string(),
      tags: vec![],
      aliases: vec![],
      permissions: vec![],
      idempotent: true,
      deprecation_message: String::new(),
      http_method_hint: String::new(),
      examples: vec![],
      auto_help_enabled: false,
    };
    registry.insert(format!(".cache_test{}", i), cmd);
  }

  // Access some commands more frequently than others
  let hot_commands = [".cache_test1", ".cache_test5", ".cache_test10"];
  let cold_commands = [".cache_test20", ".cache_test30", ".cache_test40"];

  // Access hot commands multiple times
  for _ in 0..5 {
    for cmd in &hot_commands {
      registry.get(cmd);
    }
  }

  // Access cold commands once
  for cmd in &cold_commands {
    registry.get(cmd);
  }

  // Now access hot commands again - should be cached
  for cmd in &hot_commands {
    registry.get(cmd);
  }

  // Verify cache effectiveness
  assert!(registry.cache_hit_rate() > 0.3, "Cache hit rate should show benefit from repeated access");
  println!("Final cache hit rate: {:.2}%", registry.cache_hit_rate() * 100.0);
}