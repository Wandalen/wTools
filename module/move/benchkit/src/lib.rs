#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https://docs.rs/benchkit/latest/benchkit/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Lightweight benchmarking toolkit focused on practical performance analysis and report generation" ) ]

//! # benchkit
//!
//! Lightweight benchmarking toolkit focused on practical performance analysis and report generation.
//! **benchkit** is a **toolkit, not a framework** - it provides flexible building blocks for creating 
//! custom benchmarking solutions without imposing rigid workflows.
//!
//! ## Quick Start
//!
//! ```rust
//! use benchkit::prelude::*;
//! 
//! // Simple timing measurement
//! let result = bench_function("my_operation", || {
//!     // Your code here
//!     std::hint::black_box(42 + 42);
//! });
//! 
//! println!("Average time: {:?}", result.mean_time());
//! ```
//!
//! ## Features
//!
//! - **Toolkit Philosophy** - Building blocks, not rigid framework
//! - **Markdown Integration** - Generate documentation-ready reports  
//! - **Statistical Analysis** - Proper confidence intervals and outlier detection
//! - **Comparative Benchmarking** - Before/after and A/B testing
//! - **Zero Setup** - Works in any test file or binary

#[ cfg( feature = "enabled" ) ]
pub mod measurement;

#[ cfg( feature = "enabled" ) ]
pub mod analysis;

#[ cfg( feature = "enabled" ) ]
pub mod suite;

#[ cfg( feature = "markdown_reports" ) ]
pub mod reporting;

#[ cfg( feature = "data_generators" ) ]
pub mod generators;

/// Prelude module for convenient imports
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{
  pub use crate::measurement::*;
  pub use crate::analysis::*;
  pub use crate::suite::*;
  pub use std::time::{Duration, Instant};

  #[ cfg( feature = "markdown_reports" ) ]
  pub use crate::reporting::*;

  #[ cfg( feature = "data_generators" ) ]
  pub use crate::generators::*;
}