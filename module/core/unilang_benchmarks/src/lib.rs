//! Performance benchmarks for the unilang framework
//!
//! This crate contains all performance benchmarking infrastructure for unilang,
//! separated from the main crate to avoid polluting production dependencies.
//!
//! ## Running Benchmarks
//!
//! From the workspace root:
//!
//! ```sh
//! # Run all benchmarks
//! cargo bench -p unilang_benchmarks
//!
//! # Run specific benchmark
//! cargo bench -p unilang_benchmarks --bench throughput_benchmark
//!
//! # Run with specific features
//! cargo bench -p unilang_benchmarks --features "simd"
//! ```
//!
//! ## Available Benchmarks
//!
//! - `throughput_benchmark` - Overall framework throughput
//! - `string_interning_benchmark` - String interning performance
//! - `simd_json_benchmark` - SIMD JSON parsing performance
//! - `integrated_string_interning_benchmark` - Integrated interning tests
//! - `throughput_benchmark_original` - Original baseline benchmarks
//! - `strs_tools_benchmark` - String tools performance
//!
//! ## Benchmark Configuration
//!
//! Benchmarks adapt to the execution environment via `BENCHMARK_ENV`:
//!
//! - `development` (default) - Fast feedback, relaxed accuracy (CV < 15%)
//! - `ci` / `staging` - Regression detection (CV < 10%)
//! - `production` - High accuracy analysis (CV < 5%)
//!
//! Example:
//! ```sh
//! BENCHMARK_ENV=production cargo bench -p unilang_benchmarks
//! ```

//! Benchmarks always require std library
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/unilang_benchmarks/latest/unilang_benchmarks/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// Re-export main crate for benchmark access
pub use unilang;
pub use unilang_parser;

// Benchmark support modules
pub mod benchmark_config;
pub mod benchmark_data_sizes;
pub mod realistic_test_data;
pub mod comparative_benchmark_structure;

/// Prelude for benchmarks
#[ allow( unused_imports ) ]
pub mod prelude
{
  pub use crate::benchmark_config::*;
  pub use crate::benchmark_data_sizes::*;
  pub use crate::realistic_test_data::*;
  pub use crate::comparative_benchmark_structure::*;
  pub use unilang::prelude::*;
}
