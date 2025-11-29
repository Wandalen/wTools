#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc
(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https://docs.rs/unilang/latest/unilang/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Universal language processing" ) ]

// TEMPORARY: Suppress clippy pedantic lints pending systematic cleanup
// TODO(cleanup): Remove these allows and fix underlying issues
// Tracking: 205 clippy errors exist as of 2025-10-25
#![ allow( clippy::format_push_string ) ]           // 73 occurrences - use write! instead
#![ allow( clippy::missing_errors_doc ) ]           // 19 occurrences - add # Errors sections
#![ allow( clippy::unused_self ) ]                  // 10 occurrences - remove unused self params
#![ allow( clippy::match_same_arms ) ]              // 8 occurrences - consolidate identical arms
#![ allow( clippy::doc_markdown ) ]                 // 35 occurrences - add backticks to identifiers
#![ allow( clippy::must_use_candidate ) ]           // 3 occurrences - add #[must_use] messages
#![ allow( clippy::needless_pass_by_value ) ]       // 1 occurrence - use references where possible
#![ allow( clippy::missing_panics_doc ) ]           // 1 occurrence - document panic conditions

//!
//! ## Design Rules Compliance Notice
//!
//! **CRITICAL: This codebase must follow strict design rules. Before making changes, review:**
//! - `$PRO/genai/code/rules/code_design.rulebook.md` - Core design patterns and architecture rules
//! - `$PRO/genai/code/rules/code_style.rulebook.md` - Code formatting and style requirements
//!
//! **Key Rules Summary:**
//! - **Testing:** All tests MUST be in `tests/` directory, NOT in `src/` as `mod tests`
//! - **Benchmarking:** Use `benchkit` framework ONLY - no custom timing code in tests
//! - **Performance Tests:** NEVER mix benchmarks with unit tests - separate concerns
//! - **Test Documentation:** Every test file MUST have Test Matrix documentation
//! - **Directory Structure:** `tests/` for tests, `benches/` for benchmarks (if using benchkit)
//!
//! **Common Violations to Avoid:**
//! ❌ Custom `std::time::Instant` timing code in test files
//! ❌ Performance/benchmark tests in `tests/` directory
//! ❌ Missing file-level documentation with Test Matrix in test files
//! ❌ Using anything other than `benchkit` for performance measurement
//!
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
//!   - **Dependencies**: `rustyline`, `std::io::IsTerminal`
//!
//! ### Performance Features
//! - **`simd`** - SIMD optimizations for parsing and JSON processing
//!   - **Enables**: `simd-json` (4-25x faster JSON), SIMD string operations
//!   - **Automatic**: Included in `default` for maximum performance
//!   - **Disable with**: `cargo build --no-default-features --features enabled`
//!
//! ### Optional Features
//! - `on_unknown_suggest` - Fuzzy command suggestions (requires `textdistance`)
//!
//! **Note**: Benchmarking tools are available in the separate `unilang_benchmarks` workspace crate
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
  /// **Requires feature**: `static_registry`
  #[ cfg( feature = "static_registry" ) ]
  layer static_data;

  /// Error handling utilities.
  layer error;

  /// Configuration loading from YAML/JSON.
  /// Functions gated by `yaml_parser` and `json_parser` features.
  layer loader;

  /// Value types and type system.
  layer types;

  /// Help generation system.
  layer help;

  /// Command execution interpreter.
  layer interpreter;

  /// Command registry management.
  /// Some functions gated by approach features.
  layer registry;

  /// Command validation utilities.
  layer command_validation;

  /// Core validation logic shared between runtime and build.rs.
  /// This module can be included in build.rs via include!() since it has no dependencies.
  layer validation_core;

  /// Semantic analysis and validation.
  layer semantic;

  /// High-level pipeline API.
  layer pipeline;

  /// Multi-YAML build system for compile-time aggregation.
  /// **Requires feature**: `multi_file`
  #[ cfg( feature = "multi_file" ) ]
  layer multi_yaml;

  /// String interning system for performance optimization.
  layer interner;

  /// SIMD-optimized JSON parsing for 4-25x performance improvements.
  /// **Requires features**: `simd-json` AND `json_parser`
  #[ cfg( all( feature = "simd-json", feature = "json_parser" ) ) ]
  layer simd_json_parser;

  /// SIMD-optimized tokenization for 3-6x performance improvements.
  layer simd_tokenizer;

  /// Build-time helper utilities for type analysis and hint generation.
  /// Provides tools for detecting type issues in YAML command definitions during build.
  /// **Requires feature**: `yaml_parser`
  #[ cfg( feature = "yaml_parser" ) ]
  layer build_helpers;

  /// Config value extraction utilities.
  /// Generic extractors for `HashMap<String, (JsonValue, S)>` config maps.
  /// **Requires feature**: `json_parser`
  #[ cfg( feature = "json_parser" ) ]
  layer config_extraction;

  /// Output processing utilities for CLI applications (DEPRECATED in 0.31.0).
  /// **Use `strs_tools::cli_output` instead** - this module will be removed in 0.32.0.
  /// Includes deprecated re-exports for backward compatibility.
  #[ cfg( feature = "output_processing" ) ]
  layer output;

  // NOTE: Benchmark modules have been moved to unilang_benchmarks workspace crate
  // to avoid polluting production dependencies. Use unilang_benchmarks for all
  // benchmarking needs.
}

/// Re-export unilang_parser crate as parser module.
///
/// Provides full access to the parser infrastructure including:
/// - `Parser` and `UnilangParserOptions` for parsing
/// - `GenericInstruction` and `Argument` for results
/// - `cli_parser` module for CLI parameter parsing
/// - `prelude` for convenient imports
///
/// # Example
///
/// ```rust,ignore
/// use unilang::parser::{Parser, UnilangParserOptions};
/// use unilang::parser::cli_parser::{parse_cli_args, CliParams};
/// ```
pub use unilang_parser as parser;