#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc
(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https://docs.rs/unilang/latest/unilang/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ]
#![ allow( clippy::mod_module_files ) ]

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
}