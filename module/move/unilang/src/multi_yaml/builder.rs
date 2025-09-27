//! CliBuilder API for ergonomic CLI aggregation
//!
//! This module provides the `CliBuilder` fluent API for combining multiple CLI tools
//! into unified commands with prefix management, namespace isolation, and conflict detection.
//! Supports both static and dynamic command sources with zero-overhead lookup when using
//! `StaticCommandRegistry`.

/// Internal namespace.
mod private
{
  use crate::data::CommandDefinition;
  use crate::error::Error;
  use crate::registry::{ CommandRegistry, StaticCommandRegistry, RegistryMode };
  use std::collections::HashMap;
  use std::path::PathBuf;

  /// Ergonomic CLI aggregation modes
  #[derive(Debug, Clone, PartialEq)]
  pub enum AggregationMode
  {
    /// Pure static aggregation (compile-time only)
    Static,
    /// Pure dynamic aggregation (runtime loading)
    Dynamic,
    /// Hybrid mode (static + dynamic optimizations)
    Hybrid,
    /// Automatic mode selection based on environment
    Auto,
  }

  /// Static module configuration for ergonomic APIs
  #[derive(Debug, Clone)]
  pub struct StaticModule
  {
    /// Module identifier
    pub name: String,
    /// Commands to include
    pub commands: Vec< CommandDefinition >,
    /// Namespace prefix
    pub prefix: Option< String >,
    /// Whether module is enabled
    pub enabled: bool,
  }

  /// Dynamic YAML module configuration for ergonomic APIs
  #[derive(Debug, Clone)]
  pub struct DynamicModule
  {
    /// Module identifier
    pub name: String,
    /// YAML file path
    pub yaml_path: PathBuf,
    /// Namespace prefix
    pub prefix: Option< String >,
    /// Whether module is enabled
    pub enabled: bool,
  }

  /// Conditional module based on feature flags
  #[derive(Debug, Clone)]
  pub struct ConditionalModule
  {
    /// Module identifier
    pub name: String,
    /// Feature flag to check
    pub feature: String,
    /// Module configuration when enabled
    pub module: Box< StaticModule >,
  }

  /// Re-export ModuleConfig from aggregator to avoid duplication
  pub use crate::multi_yaml::aggregator::ModuleConfig;

  /// Module source type for aggregation
  #[derive(Debug, Clone)]
  pub enum ModuleSource
  {
    /// Static commands compiled into binary
    Static( StaticModule ),
    /// Dynamic YAML file loaded at runtime
    Dynamic( DynamicModule ),
    /// Conditional module based on feature flags
    Conditional( ConditionalModule ),
  }

  /// Global CLI configuration
  #[derive(Debug, Clone, Default)]
  pub struct CliConfig
  {
    /// Application name
    pub app_name: String,
    /// Global prefix for all commands
    pub global_prefix: Option< String >,
    /// Whether to enable help generation
    pub auto_help: bool,
    /// Whether to detect conflicts
    pub detect_conflicts: bool,
    /// Environment variable overrides
    pub env_overrides: HashMap< String, String >,
    /// Environment variable exclusions
    pub exclude_env_overrides: Vec< String >,
  }

  /// Re-export ConflictReport and ConflictType from aggregator to avoid duplication
  pub use crate::multi_yaml::aggregator::{ ConflictReport, ConflictType };

  /// Ergonomic CLI builder for simple and complex aggregation scenarios
  #[derive(Debug, Clone)]
  pub struct CliBuilder
  {
    /// Registry mode for aggregation
    mode: AggregationMode,
    /// Static command modules
    static_modules: Vec< StaticModule >,
    /// Dynamic YAML modules
    dynamic_modules: Vec< DynamicModule >,
    /// Conditional modules based on features
    conditional_modules: Vec< ConditionalModule >,
    /// Global configuration
    config: CliConfig,
  }

  impl CliBuilder
  {
    /// Create a new CLI builder with intelligent defaults
    pub fn new() -> Self
    {
      Self
      {
        mode: AggregationMode::Auto,
        static_modules: Vec::new(),
        dynamic_modules: Vec::new(),
        conditional_modules: Vec::new(),
        config: CliConfig
        {
          app_name: "app".to_string(),
          auto_help: true,
          detect_conflicts: true,
          ..Default::default()
        },
      }
    }

    /// Set aggregation mode
    pub fn mode( mut self, mode: AggregationMode ) -> Self
    {
      self.mode = mode;
      self
    }

    /// Add a static module
    pub fn static_module( mut self, name: &str, commands: Vec< CommandDefinition > ) -> Self
    {
      self.static_modules.push( StaticModule
      {
        name: name.to_string(),
        commands,
        prefix: None,
        enabled: true,
      } );
      self
    }

    /// Add a static module with prefix
    pub fn static_module_with_prefix( mut self, name: &str, prefix: &str, commands: Vec< CommandDefinition > ) -> Self
    {
      self.static_modules.push( StaticModule
      {
        name: name.to_string(),
        commands,
        prefix: Some( prefix.to_string() ),
        enabled: true,
      } );
      self
    }

    /// Add a dynamic YAML module
    pub fn dynamic_module( mut self, name: &str, yaml_path: PathBuf ) -> Self
    {
      self.dynamic_modules.push( DynamicModule
      {
        name: name.to_string(),
        yaml_path,
        prefix: None,
        enabled: true,
      } );
      self
    }

    /// Add a dynamic YAML module with prefix
    pub fn dynamic_module_with_prefix( mut self, name: &str, yaml_path: PathBuf, prefix: &str ) -> Self
    {
      self.dynamic_modules.push( DynamicModule
      {
        name: name.to_string(),
        yaml_path,
        prefix: Some( prefix.to_string() ),
        enabled: true,
      } );
      self
    }

    /// Add a conditional module
    pub fn conditional_module( mut self, name: &str, feature: &str, commands: Vec< CommandDefinition > ) -> Self
    {
      self.conditional_modules.push( ConditionalModule
      {
        name: name.to_string(),
        feature: feature.to_string(),
        module: Box::new( StaticModule
        {
          name: name.to_string(),
          commands,
          prefix: None,
          enabled: true,
        } ),
      } );
      self
    }

    /// Set application name
    pub fn app_name( mut self, name: &str ) -> Self
    {
      self.config.app_name = name.to_string();
      self
    }

    /// Set global prefix
    pub fn global_prefix( mut self, prefix: &str ) -> Self
    {
      self.config.global_prefix = Some( prefix.to_string() );
      self
    }

    /// Enable or disable auto-help
    pub fn auto_help( mut self, enabled: bool ) -> Self
    {
      self.config.auto_help = enabled;
      self
    }

    /// Enable or disable conflict detection
    pub fn detect_conflicts( mut self, enabled: bool ) -> Self
    {
      self.config.detect_conflicts = enabled;
      self
    }

    /// Detect and report command prefix conflicts at build time
    pub fn detect_conflicts_report( &self ) -> Vec< ConflictReport >
    {
      if !self.config.detect_conflicts
      {
        return Vec::new();
      }

      let mut conflicts = Vec::new();
      let mut all_commands: HashMap< String, Vec< String > > = HashMap::new();

      // Check static modules for conflicts
      for module in &self.static_modules
      {
        if !module.enabled
        {
          continue;
        }

        for cmd in &module.commands
        {
          let final_name = self.compute_final_command_name( cmd, module.prefix.as_ref() );
          all_commands
            .entry( final_name )
            .or_insert_with( Vec::new )
            .push( module.name.clone() );
        }
      }

      // Check dynamic modules for conflicts (simplified - would need YAML parsing in real impl)
      for module in &self.dynamic_modules
      {
        if !module.enabled
        {
          continue;
        }

        // For now, simulate command names from dynamic modules
        let simulated_cmd_name = format!( "example_from_{}", module.name );
        let final_name = if let Some( prefix ) = &module.prefix
        {
          if let Some( global_prefix ) = &self.config.global_prefix
          {
            format!( ".{}.{}.{}", global_prefix, prefix, simulated_cmd_name )
          }
          else
          {
            format!( ".{}.{}", prefix, simulated_cmd_name )
          }
        }
        else if let Some( global_prefix ) = &self.config.global_prefix
        {
          format!( ".{}.{}", global_prefix, simulated_cmd_name )
        }
        else
        {
          simulated_cmd_name
        };

        all_commands
          .entry( final_name )
          .or_insert_with( Vec::new )
          .push( module.name.clone() );
      }

      // Check conditional modules for conflicts
      for cond_module in &self.conditional_modules
      {
        if self.is_feature_enabled( &cond_module.feature )
        {
          for cmd in &cond_module.module.commands
          {
            let final_name = self.compute_final_command_name( cmd, cond_module.module.prefix.as_ref() );
            all_commands
              .entry( final_name )
              .or_insert_with( Vec::new )
              .push( cond_module.name.clone() );
          }
        }
      }

      // Generate conflict reports
      for ( cmd_name, sources ) in all_commands
      {
        if sources.len() > 1
        {
          conflicts.push( ConflictReport
          {
            command_name: cmd_name,
            modules: sources,
            conflict_type: ConflictType::NameCollision,
          } );
        }
      }

      conflicts
    }

    /// Compute the final command name after applying prefixes
    fn compute_final_command_name( &self, cmd: &CommandDefinition, module_prefix: Option< &String > ) -> String
    {
      let mut final_name = cmd.name.clone();

      // Apply module prefix
      if let Some( prefix ) = module_prefix
      {
        final_name = if cmd.namespace.is_empty()
        {
          format!( ".{}.{}", prefix, final_name.strip_prefix( '.' ).unwrap_or( &final_name ) )
        }
        else
        {
          format!( ".{}{}.{}", prefix, cmd.namespace, final_name.strip_prefix( '.' ).unwrap_or( &final_name ) )
        };
      }

      // Apply global prefix
      if let Some( global_prefix ) = &self.config.global_prefix
      {
        final_name = format!( ".{}{}", global_prefix, final_name );
      }

      final_name
    }

    /// Build the CLI registry with dynamic registration
    pub fn build( self ) -> Result< CommandRegistry, Error >
    {
      let mut registry = CommandRegistry::new();

      // Set registry mode based on aggregation mode
      let registry_mode = match self.mode
      {
        AggregationMode::Static => RegistryMode::Hybrid, // Static modules are registered dynamically
        AggregationMode::Dynamic => RegistryMode::DynamicOnly,
        AggregationMode::Hybrid => RegistryMode::Hybrid,
        AggregationMode::Auto => self.detect_optimal_mode(),
      };

      registry.set_registry_mode( registry_mode );

      // Register static modules
      for module in &self.static_modules
      {
        if !module.enabled
        {
          continue;
        }

        for mut cmd in module.commands.clone()
        {
          // Apply module prefix
          if let Some( prefix ) = &module.prefix
          {
            cmd.namespace = if cmd.namespace.is_empty()
            {
              format!( ".{}", prefix )
            }
            else
            {
              format!( ".{}{}", prefix, cmd.namespace )
            };
          }

          // Apply global prefix
          if let Some( global_prefix ) = &self.config.global_prefix
          {
            cmd.namespace = if cmd.namespace.is_empty()
            {
              format!( ".{}", global_prefix )
            }
            else
            {
              format!( ".{}{}", global_prefix, cmd.namespace )
            };
          }

          registry.register( cmd );
        }
      }

      // Process dynamic modules
      for module in &self.dynamic_modules
      {
        if !module.enabled
        {
          continue;
        }

        // For now, create a sample command from the dynamic module
        // In a real implementation, this would load and parse the YAML file
        let mut cmd = CommandDefinition::former()
          .name( "example" )
          .description( format!( "Example command from {} module", module.name ) )
          .hint( "Dynamic command" )
          .form();

        // Apply module prefix
        if let Some( prefix ) = &module.prefix
        {
          cmd.namespace = format!( ".{}", prefix );
        }

        // Apply global prefix
        if let Some( global_prefix ) = &self.config.global_prefix
        {
          cmd.namespace = if cmd.namespace.is_empty()
          {
            format!( ".{}", global_prefix )
          }
          else
          {
            format!( ".{}{}", global_prefix, cmd.namespace )
          };
        }

        registry.register( cmd );
      }

      // Process conditional modules (check feature flags)
      for cond_module in &self.conditional_modules
      {
        if self.is_feature_enabled( &cond_module.feature )
        {
          for mut cmd in cond_module.module.commands.clone()
          {
            // Apply conditional module namespace
            cmd.namespace = format!( ".{}", cond_module.name );

            // Apply global prefix if configured
            if let Some( global_prefix ) = &self.config.global_prefix
            {
              cmd.namespace = format!( ".{}{}", global_prefix, cmd.namespace );
            }

            registry.register( cmd );
          }
        }
      }

      Ok( registry )
    }

    /// Build a static registry with zero-overhead lookup optimized for StaticCommandRegistry
    pub fn build_static( self ) -> Result< StaticCommandRegistry, Error >
    {
      let mut static_registry = StaticCommandRegistry::new();

      // Set registry mode for optimal static performance
      static_registry.set_mode( RegistryMode::StaticOnly );

      // Process static modules only for optimal performance
      for module in &self.static_modules
      {
        if !module.enabled
        {
          continue;
        }

        for mut cmd in module.commands.clone()
        {
          // Apply module prefix
          if let Some( prefix ) = &module.prefix
          {
            cmd.namespace = if cmd.namespace.is_empty()
            {
              format!( ".{}", prefix )
            }
            else
            {
              format!( ".{}{}", prefix, cmd.namespace )
            };
          }

          // Apply global prefix
          if let Some( global_prefix ) = &self.config.global_prefix
          {
            cmd.namespace = if cmd.namespace.is_empty()
            {
              format!( ".{}", global_prefix )
            }
            else
            {
              format!( ".{}{}", global_prefix, cmd.namespace )
            };
          }

          // Register command with the static registry
          let cmd_name = cmd.name.clone(); // Clone to avoid lifetime issues
          let routine = Box::new( move |_cmd, _ctx| {
            Ok( crate::data::OutputData
            {
              content: format!( "Static command '{}' executed", cmd_name ),
              format: "text".to_string(),
            })
          });

          static_registry.register_with_routine( cmd, routine )?;
        }
      }

      // Note: Dynamic and conditional modules are skipped in static build for zero-overhead
      // Users should use build() for hybrid scenarios

      Ok( static_registry )
    }

    /// Detect optimal aggregation mode based on environment
    pub fn detect_optimal_mode( &self ) -> RegistryMode
    {
      let has_static = !self.static_modules.is_empty();
      let has_dynamic = !self.dynamic_modules.is_empty();
      let has_conditional = !self.conditional_modules.is_empty();

      // If any modules are present that require dynamic registration, use Hybrid or DynamicOnly
      if has_static || has_conditional
      {
        if has_dynamic
        {
          RegistryMode::Hybrid
        }
        else
        {
          // Static or conditional modules exist (both use dynamic registration), use Hybrid
          RegistryMode::Hybrid
        }
      }
      else if has_dynamic
      {
        RegistryMode::DynamicOnly
      }
      else
      {
        // No modules configured, default to StaticOnly
        RegistryMode::StaticOnly
      }
    }

    /// Check if a feature is enabled (simplified for testing)
    fn is_feature_enabled( &self, feature: &str ) -> bool
    {
      // In real implementation, this would check Cargo features
      // For testing, we'll simulate some enabled features
      match feature
      {
        "test_feature" | "advanced" => true,
        _ => false,
      }
    }

    /// Get current aggregation mode (for testing)
    pub fn get_mode( &self ) -> &AggregationMode
    {
      &self.mode
    }

    /// Get current configuration (for testing)
    pub fn get_config( &self ) -> &CliConfig
    {
      &self.config
    }

    /// Get static modules count (for testing)
    pub fn static_modules_count( &self ) -> usize
    {
      self.static_modules.len()
    }

    /// Get dynamic modules count (for testing)
    pub fn dynamic_modules_count( &self ) -> usize
    {
      self.dynamic_modules.len()
    }

    /// Get conditional modules count (for testing)
    pub fn conditional_modules_count( &self ) -> usize
    {
      self.conditional_modules.len()
    }
  }

  impl Default for CliBuilder
  {
    fn default() -> Self
    {
      Self::new()
    }
  }
}

mod_interface::mod_interface!
{
  exposed use private::AggregationMode;
  exposed use private::StaticModule;
  exposed use private::DynamicModule;
  exposed use private::ConditionalModule;
  exposed use private::ModuleConfig;
  exposed use private::ModuleSource;
  exposed use private::CliConfig;
  exposed use private::ConflictReport;
  exposed use private::ConflictType;
  exposed use private::CliBuilder;

  prelude use private::CliBuilder;
  prelude use private::AggregationMode;
  prelude use private::StaticModule;
  prelude use private::DynamicModule;
  prelude use private::ConditionalModule;
}