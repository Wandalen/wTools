#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc
(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https://docs.rs/unilang/latest/unilang/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Universal language processing" ) ]

//! ## Feature Flags
//!
//! Unilang supports multiple feature flags to customize functionality and dependencies:
//!
//! ### Core Features
//! - `enabled` - Core functionality (included in `default`)
//! - `full` - All features enabled for maximum functionality
//!
//! ### REPL Features  
//! - **`repl`** - Basic REPL functionality with standard I/O
//!   - Provides interactive command execution
//!   - Basic command history tracking
//!   - Cross-platform compatibility
//!   - No additional dependencies
//!
//! - **`enhanced_repl`** ⭐ **Enabled by Default** - Advanced REPL with rustyline integration
//!   - **Enables**: All features from `repl` plus:
//!   - **Arrow Key Navigation**: ↑/↓ for command history browsing
//!   - **Tab Auto-completion**: Command and argument completion
//!   - **Interactive Prompts**: Secure password input with masking
//!   - **Session Persistence**: History saved across sessions
//!   - **Terminal Detection**: Auto-fallback to basic REPL in non-interactive environments
//!   - **Dependencies**: `rustyline`, `atty`
//!
//! ### Performance Features
//! - **`simd`** - SIMD optimizations for parsing and JSON processing
//!   - **Enables**: `simd-json` (4-25x faster JSON), SIMD string operations
//!   - **Automatic**: Included in `default` for maximum performance
//!   - **Disable with**: `cargo build --no-default-features --features enabled`
//!
//! ### Optional Features
//! - `on_unknown_suggest` - Fuzzy command suggestions (requires `textdistance`)
//! - `benchmarks` - Development benchmarking tools (dev-only)
//!
//! ### Usage Examples
//!
//! **Basic REPL (minimal dependencies):**
//! ```toml
//! [dependencies]
//! unilang = { version = "0.10", features = ["repl"] }
//! ```
//!
//! **Default (Enhanced REPL included):**
//! ```toml
//! [dependencies]
//! unilang = "0.10"  # Enhanced REPL enabled by default
//! ```
//!
//! **Performance-optimized CLI:**
//! ```toml
//! [dependencies]
//! unilang = { version = "0.10", features = ["enhanced_repl", "simd", "on_unknown_suggest"] }
//! ```
//!
//! **Embedded/minimal:**
//! ```toml
//! [dependencies]
//! unilang = { version = "0.10", default-features = false, features = ["enabled"] }
//! ```
//!
//! ### Feature Compatibility
//!
//! - `enhanced_repl` automatically includes `repl`
//! - `full` includes all features except development-only ones
//! - All features work together without conflicts
//! - Enhanced REPL gracefully falls back to basic REPL when needed
#![ allow( clippy::mod_module_files ) ]
#![ allow( clippy::format_push_string ) ]
#![ allow( clippy::used_underscore_binding ) ]
#![ allow( clippy::match_same_arms ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::semicolon_if_nothing_returned ) ]
#![ allow( clippy::redundant_closure ) ]
#![ allow( clippy::unreadable_literal ) ]
#![ allow( clippy::redundant_closure_for_method_calls ) ]
#![ allow( clippy::unused_self ) ]
#![ allow( clippy::useless_vec ) ]
#![ allow( clippy::missing_errors_doc ) ]
#![ allow( clippy::needless_pass_by_value ) ]
#![ allow( clippy::must_use_candidate ) ]
#![ allow( clippy::too_many_arguments ) ]
#![ allow( clippy::large_enum_variant ) ]
#![ allow( clippy::module_name_repetitions ) ]
#![ allow( clippy::writeln_empty_string ) ]
#![ allow( clippy::doc_markdown ) ]
#![ allow( clippy::struct_excessive_bools ) ]
#![ allow( clippy::fn_params_excessive_bools ) ]
#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::manual_let_else ) ]
#![ allow( clippy::cast_possible_truncation ) ]
#![ allow( clippy::missing_panics_doc ) ]
#![ allow( clippy::map_unwrap_or ) ]
#![ allow( clippy::unused_unit ) ]
#![ allow( clippy::similar_names ) ]
#![ allow( clippy::all ) ]
#![ allow( clippy::doc_link_with_quotes ) ]
#![ allow( clippy::cast_sign_loss ) ]
#![ allow( clippy::no_effect_underscore_binding ) ]
#![ allow( clippy::return_self_not_must_use ) ]

/// Internal namespace.
mod private
{
}

mod_interface::mod_interface!
{
  /// Core data structures and types.
  layer data;
  
  /// Static data structures for compile-time commands.
  layer static_data;
  
  /// Error handling utilities.
  layer error;
  
  /// Configuration loading from YAML/JSON.
  layer loader;
  
  /// Value types and type system.
  layer types;
  
  /// Help generation system.
  layer help;
  
  /// Command execution interpreter.
  layer interpreter;
  
  /// Command registry management.
  layer registry;
  
  /// Semantic analysis and validation.
  layer semantic;
  
  /// High-level pipeline API.
  layer pipeline;
  
  /// String interning system for performance optimization.
  layer interner;
  
  /// SIMD-optimized JSON parsing for 4-25x performance improvements.
  layer simd_json_parser;
  
  /// SIMD-optimized tokenization for 3-6x performance improvements.
  layer simd_tokenizer;
  
  /// Environment-specific benchmark configuration system.
  #[ cfg( feature = "benchmarks" ) ]
  layer benchmark_config;
  
  /// Coefficient of variation analysis for benchmark quality assessment.
  #[ cfg( feature = "benchmarks" ) ]
  layer cv_analysis;
  
  /// Automatic documentation updater for benchmark results.
  #[ cfg( feature = "benchmarks" ) ]
  layer documentation_updater;
  
  /// Standard benchmark data sizes for consistent performance comparison.
  #[ cfg( feature = "benchmarks" ) ]
  layer benchmark_data_sizes;
  
  /// Realistic test data generation for production-like benchmarks.
  #[ cfg( feature = "benchmarks" ) ]
  layer realistic_test_data;
  
  /// Comparative benchmark structure for side-by-side algorithm performance analysis.
  #[ cfg( feature = "benchmarks" ) ]
  layer comparative_benchmark_structure;
  
  /// Context-rich benchmark documentation generator for comprehensive reporting.
  #[ cfg( feature = "benchmarks" ) ]
  layer context_rich_documentation;
  
  /// Before/after optimization workflow system for systematic performance tracking.
  #[ cfg( feature = "benchmarks" ) ]
  layer optimization_workflow;
}