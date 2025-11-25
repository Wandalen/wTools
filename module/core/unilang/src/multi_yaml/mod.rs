//! Multi-YAML Build System and Ergonomic Aggregation APIs
//!
//! This module implements the enhanced build system that processes multiple YAML files
//! and combines them at compile-time with zero runtime overhead. It also provides
//! ergonomic aggregation APIs for simple and complex use cases.

/// Multi-YAML aggregation system for compile-time command processing
pub mod aggregator;

/// CLI builder for ergonomic command aggregation
pub mod builder;

mod private
{
  #[ allow( unused_imports ) ]
  use crate::*;

  /// Re-export key aggregator types
  pub use super::aggregator::{
    MultiYamlAggregator,
    AggregationConfig,
    ModuleConfig,
    ConflictReport,
    ConflictType,
    ConflictResolutionStrategy,
    NamespaceIsolation,
    EnvConfigParser,
    parse_cargo_metadata,
    create_aggregated_registry,
    aggregate_cli_simple,
    aggregate_cli_complex,
  };

  /// Re-export key builder types
  pub use super::builder::{
    CliBuilder,
    AggregationMode,
    StaticModule,
    DynamicModule,
    ConditionalModule,
    ModuleSource,
    CliConfig,
  };
}

// Direct exports from the private module
pub use private::{
  MultiYamlAggregator,
  AggregationConfig,
  ModuleConfig,
  ConflictReport,
  ConflictType,
  ConflictResolutionStrategy,
  NamespaceIsolation,
  EnvConfigParser,
  parse_cargo_metadata,
  create_aggregated_registry,
  aggregate_cli_simple,
  aggregate_cli_complex,
  CliBuilder,
  AggregationMode,
  StaticModule,
  DynamicModule,
  ConditionalModule,
  ModuleSource,
  CliConfig,
};

/// Orphaned stuff.
pub mod orphan
{
  pub use super::
  {
    aggregator,
    builder,
  };

  pub use super::private::
  {
    MultiYamlAggregator,
    AggregationConfig,
    ModuleConfig,
    ConflictReport,
    ConflictType,
    ConflictResolutionStrategy,
    NamespaceIsolation,
    EnvConfigParser,
    parse_cargo_metadata,
    create_aggregated_registry,
    aggregate_cli_simple,
    aggregate_cli_complex,
    CliBuilder,
    AggregationMode,
    StaticModule,
    DynamicModule,
    ConditionalModule,
    ModuleSource,
    CliConfig,
  };
}

/// Exposed stuff of module.
pub mod exposed
{
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::
  {
    MultiYamlAggregator,
    AggregationConfig,
    ModuleConfig,
    ConflictReport,
    ConflictType,
    ConflictResolutionStrategy,
    NamespaceIsolation,
    EnvConfigParser,
    parse_cargo_metadata,
    create_aggregated_registry,
    aggregate_cli_simple,
    aggregate_cli_complex,
    CliBuilder,
    AggregationMode,
    StaticModule,
    DynamicModule,
    ConditionalModule,
    ModuleSource,
    CliConfig,
  };
}