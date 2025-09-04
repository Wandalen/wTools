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
}