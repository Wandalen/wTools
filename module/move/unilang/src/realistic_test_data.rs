//! Realistic test data generation for unilang benchmarks
//!
//! Implements benchkit usage.md "Generate Realistic Test Data" section
//! with production-like data patterns and fixed seeding for reproducible results.

/// Internal namespace.
mod private
{
  #[ cfg( feature = "benchmarks" ) ]
  use rand::{ Rng, SeedableRng };
  #[ cfg( feature = "benchmarks" ) ]
  use rand::rngs::StdRng;
  #[ cfg( feature = "benchmarks" ) ]
  use crate::benchmark_data_sizes::BenchmarkDataSize;

  /// Realistic test data generator for unilang scenarios
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug ) ]
  pub struct RealisticDataGenerator
  {
    rng : StdRng,
  }

  #[ cfg( feature = "benchmarks" ) ]
  impl RealisticDataGenerator
  {
    /// Create new generator with fixed seed for reproducible results
    pub fn new() -> Self
    {
      Self::with_seed( 12345 )
    }

    /// Create generator with custom seed
    pub fn with_seed( seed : u64 ) -> Self
    {
      Self
      {
        rng : StdRng::seed_from_u64( seed ),
      }
    }

    /// Generate realistic command names based on common patterns
    pub fn generate_command_names( &mut self, count : usize ) -> Vec< String >
    {
      let common_commands = vec![
        "config", "deploy", "build", "test", "run", "start", "stop", "restart",
        "install", "update", "remove", "list", "show", "get", "set", "add",
        "delete", "create", "edit", "copy", "move", "backup", "restore",
        "connect", "disconnect", "login", "logout", "auth", "validate",
        "generate", "compile", "package", "publish", "download", "upload",
        "sync", "merge", "commit", "push", "pull", "clone", "branch", "tag"
      ];

      let namespaces = vec![
        "system", "user", "config", "network", "database", "service", "api",
        "auth", "security", "monitoring", "backup", "deployment", "docker",
        "kubernetes", "aws", "azure", "gcp", "git", "ci", "cd", "test"
      ];

      let subcommands = vec![
        "create", "update", "delete", "list", "show", "get", "set", "reset",
        "enable", "disable", "start", "stop", "restart", "status", "info",
        "logs", "history", "backup", "restore", "validate", "check", "fix"
      ];

      ( 0..count ).map( | i | {
        if i < common_commands.len() {
          // First use common single commands
          format!( ".{}", common_commands[ i ] )
        } else {
          // Then generate realistic namespace.command.subcommand patterns
          let namespace = namespaces[ self.rng.gen_range( 0..namespaces.len() ) ];
          let command = common_commands[ self.rng.gen_range( 0..common_commands.len() ) ];
          let subcommand = subcommands[ self.rng.gen_range( 0..subcommands.len() ) ];
          
          match i % 4 {
            0 => format!( ".{}", command ),
            1 => format!( ".{}.{}", namespace, command ),
            2 => format!( ".{}.{}.{}", namespace, command, subcommand ),
            _ => format!( ".{}.{}", command, subcommand ),
          }
        }
      } ).collect()
    }

    /// Generate realistic argument patterns
    pub fn generate_realistic_args( &mut self, command : &str, count : usize ) -> Vec< String >
    {
      let mut args = Vec::new();
      
      for i in 0..count {
        match i % 6 {
          0 => args.push( format!( "{} --verbose", command ) ),
          1 => args.push( format!( "{} --config /etc/app/config.yml", command ) ),
          2 => args.push( format!( "{} --output-format json", command ) ),
          3 => args.push( format!( "{} --environment production", command ) ),
          4 => args.push( format!( "{} --timeout {}s", command, self.rng.gen_range( 30..300 ) ) ),
          _ => args.push( format!( "{} --user user{} --force", command, self.rng.gen_range( 1..1000 ) ) ),
        }
      }
      
      args
    }

    /// Generate realistic user data for parsing benchmarks
    pub fn generate_user_data( &mut self, count : usize ) -> Vec< String >
    {
      let domains = [ "example.com", "test.org", "company.net", "service.io", "app.dev" ];
      let first_names = [ "John", "Jane", "Bob", "Alice", "Charlie", "Diana", "Eva", "Frank" ];
      let last_names = [ "Smith", "Johnson", "Brown", "Davis", "Wilson", "Miller", "Taylor", "Anderson" ];
      
      ( 0..count ).map( | i | {
        let first = first_names[ i % first_names.len() ];
        let last = last_names[ ( i / first_names.len() ) % last_names.len() ];
        let domain = domains[ self.rng.gen_range( 0..domains.len() ) ];
        let id = self.rng.gen_range( 1000..99999 );
        
        format!( r#"{{"id": {}, "name": "{} {}", "email": "{}.{}@{}", "active": {}, "department": "{}"}}"#,
          id, first, last, 
          first.to_lowercase(), last.to_lowercase(), domain,
          self.rng.gen_bool( 0.85 ), // 85% active users
          if i % 5 == 0 { "engineering" } else if i % 3 == 0 { "marketing" } else { "operations" }
        )
      } ).collect()
    }

    /// Generate realistic JSON payloads for different scenarios
    pub fn generate_json_scenarios( &mut self, size : BenchmarkDataSize ) -> String
    {
      match size {
        BenchmarkDataSize::Small => self.generate_small_json_payload(),
        BenchmarkDataSize::Medium => self.generate_medium_json_payload(),
        BenchmarkDataSize::Large => self.generate_large_json_payload(),
        BenchmarkDataSize::Huge => self.generate_huge_json_payload(),
      }
    }

    fn generate_small_json_payload( &mut self ) -> String
    {
      // Typical API response
      format!( r#"{{
        "status": "success",
        "timestamp": "2024-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        "request_id": "req_{}",
        "data": {{
          "user_id": {},
          "session_token": "st_{}",
          "permissions": ["read", "write"],
          "preferences": {{
            "theme": "{}",
            "language": "{}",
            "notifications": {}
          }}
        }}
      }}"#, 
        self.rng.gen_range( 1..13 ), self.rng.gen_range( 1..29 ), 
        self.rng.gen_range( 0..24 ), self.rng.gen_range( 0..60 ), self.rng.gen_range( 0..60 ),
        self.rng.gen_range( 100000..999999 ),
        self.rng.gen_range( 1000..99999 ),
        self.rng.gen_range( 100000..999999 ),
        if self.rng.gen_bool( 0.5 ) { "dark" } else { "light" },
        if self.rng.gen_bool( 0.7 ) { "en" } else { "es" },
        self.rng.gen_bool( 0.8 )
      )
    }

    fn generate_medium_json_payload( &mut self ) -> String
    {
      let users : Vec< String > = ( 0..20 ).map( | i | {
        format!( r#"{{
          "id": {},
          "username": "user_{}",
          "email": "user{}@domain{}.com",
          "last_login": "2024-{:02}-{:02}T{:02}:{:02}:00Z",
          "role": "{}"
        }}"#,
          1000 + i,
          self.rng.gen_range( 10000..99999 ),
          i,
          self.rng.gen_range( 1..10 ),
          self.rng.gen_range( 1..13 ), self.rng.gen_range( 1..29 ),
          self.rng.gen_range( 0..24 ), self.rng.gen_range( 0..60 ),
          match i % 4 { 0 => "admin", 1 => "editor", 2 => "viewer", _ => "guest" }
        )
      } ).collect();

      format!( r#"{{
        "page": {},
        "per_page": 20,
        "total": {},
        "users": [{}]
      }}"#,
        self.rng.gen_range( 1..10 ),
        self.rng.gen_range( 100..1000 ),
        users.join( ",\n  " )
      )
    }

    fn generate_large_json_payload( &mut self ) -> String
    {
      // Realistic log aggregation response
      let logs : Vec< String > = ( 0..100 ).map( | i | {
        format!( r#"{{
          "timestamp": "2024-{:02}-{:02}T{:02}:{:02}:{:02}.{}Z",
          "level": "{}",
          "service": "{}",
          "message": "{}",
          "request_id": "{}",
          "duration_ms": {}
        }}"#,
          self.rng.gen_range( 1..13 ), self.rng.gen_range( 1..29 ),
          self.rng.gen_range( 0..24 ), self.rng.gen_range( 0..60 ), self.rng.gen_range( 0..60 ),
          self.rng.gen_range( 100..999 ),
          match i % 5 { 0 => "ERROR", 1 => "WARN", 2 => "INFO", 3 => "DEBUG", _ => "TRACE" },
          match i % 4 { 0 => "api", 1 => "database", 2 => "auth", _ => "scheduler" },
          match i % 6 {
            0 => "Request processed successfully",
            1 => "Database connection established", 
            2 => "User authentication completed",
            3 => "Cache miss, fetching from database",
            4 => "Rate limit check passed",
            _ => "Health check completed"
          },
          format!( "req_{}", self.rng.gen_range( 100000..999999 ) ),
          self.rng.gen_range( 1..500 )
        )
      } ).collect();

      format!( r#"{{
        "query": {{
          "start_time": "2024-01-01T00:00:00Z",
          "end_time": "2024-01-01T23:59:59Z",
          "service": "all",
          "level": "info"
        }},
        "results": {},
        "logs": [{}]
      }}"#,
        logs.len(),
        logs.join( ",\n    " )
      )
    }

    fn generate_huge_json_payload( &mut self ) -> String
    {
      // Large dataset with metrics
      let metrics : Vec< String > = ( 0..500 ).map( | i | {
        format!( r#"{{
          "timestamp": {},
          "metric": "{}",
          "value": {:.2},
          "tags": {{
            "host": "server-{:02}",
            "region": "{}",
            "environment": "{}"
          }}
        }}"#,
          1640995200 + ( i * 60 ), // Unix timestamp with 1-minute intervals
          match i % 8 {
            0 => "cpu.usage.percent",
            1 => "memory.used.bytes",
            2 => "disk.io.read.ops",
            3 => "network.bytes.sent",
            4 => "requests.per.second", 
            5 => "response.time.ms",
            6 => "errors.per.minute",
            _ => "database.connections.active"
          },
          match i % 8 {
            0 => self.rng.gen_range( 10.0..95.0 ),
            1 => self.rng.gen_range( 1000000.0..8000000000.0 ),
            2 => self.rng.gen_range( 100.0..10000.0 ),
            3 => self.rng.gen_range( 1000.0..1000000.0 ),
            4 => self.rng.gen_range( 10.0..1000.0 ),
            5 => self.rng.gen_range( 50.0..2000.0 ),
            6 => self.rng.gen_range( 0.0..50.0 ),
            _ => self.rng.gen_range( 5.0..200.0 )
          },
          ( i % 20 ) + 1,
          match i % 3 { 0 => "us-east-1", 1 => "us-west-2", _ => "eu-west-1" },
          match i % 3 { 0 => "production", 1 => "staging", _ => "development" }
        )
      } ).collect();

      format!( r#"{{
        "metadata": {{
          "query_time_ms": {},
          "total_metrics": {},
          "data_points": {},
          "time_range": {{
            "start": "2024-01-01T00:00:00Z",
            "end": "2024-01-01T08:20:00Z"
          }}
        }},
        "metrics": [{}]
      }}"#,
        self.rng.gen_range( 50..500 ),
        metrics.len(),
        metrics.len() * 10,
        metrics.join( ",\n    " )
      )
    }
  }

  #[ cfg( feature = "benchmarks" ) ]
  impl Default for RealisticDataGenerator
  {
    fn default() -> Self
    {
      Self::new()
    }
  }

  /// Pre-generated realistic data cache for performance
  #[ cfg( feature = "benchmarks" ) ]
  #[ derive( Debug ) ]
  pub struct RealisticDataCache
  {
    command_names : std::collections::HashMap< usize, Vec< String > >,
    user_data : std::collections::HashMap< usize, Vec< String > >,
    json_scenarios : std::collections::HashMap< String, String >,
  }

  #[ cfg( feature = "benchmarks" ) ]
  impl RealisticDataCache
  {
    /// Create new cache and pre-generate common data sizes
    pub fn new() -> Self
    {
      let mut cache = Self
      {
        command_names : std::collections::HashMap::new(),
        user_data : std::collections::HashMap::new(), 
        json_scenarios : std::collections::HashMap::new(),
      };
      
      cache.pregenerate_all();
      cache
    }

    /// Pre-generate data for standard sizes to avoid generation during benchmarks
    pub fn pregenerate_all( &mut self )
    {
      let mut generator = RealisticDataGenerator::new();
      
      // Pre-generate for all standard sizes
      for size in BenchmarkDataSize::all() {
        let count = size.value();
        self.command_names.insert( count, generator.generate_command_names( count ) );
        self.user_data.insert( count, generator.generate_user_data( count ) );
        self.json_scenarios.insert( size.name().to_string(), generator.generate_json_scenarios( size ) );
      }
    }

    /// Get pre-generated command names for specific count
    pub fn get_command_names( &self, count : usize ) -> Option< &Vec< String > >
    {
      self.command_names.get( &count )
    }

    /// Get pre-generated user data for specific count
    pub fn get_user_data( &self, count : usize ) -> Option< &Vec< String > >
    {
      self.user_data.get( &count )
    }

    /// Get pre-generated JSON scenario for specific size
    pub fn get_json_scenario( &self, size : BenchmarkDataSize ) -> Option< &String >
    {
      self.json_scenarios.get( size.name() )
    }
  }

  #[ cfg( feature = "benchmarks" ) ]
  impl Default for RealisticDataCache
  {
    fn default() -> Self
    {
      Self::new()
    }
  }
}

mod_interface::mod_interface!
{
  #[ cfg( feature = "benchmarks" ) ]
  orphan use RealisticDataGenerator;
  #[ cfg( feature = "benchmarks" ) ]
  orphan use RealisticDataCache;
}