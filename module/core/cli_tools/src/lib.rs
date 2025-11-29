//! CLI application building blocks.
//!
//! This crate provides utilities for building command-line applications,
//! including output processing, formatting, and other CLI-specific helpers.
//!
//! # Modules
//!
//! - `cli_output` - Process command output (head/tail filtering, width truncation, stream merging)
//!
//! # Architecture
//!
//! This crate focuses on CLI-application-specific functionality. For general-purpose
//! string and ANSI manipulation, use `strs_tools` instead.
//!
//! **Distinction:**
//! - `strs_tools`: Generic string/ANSI utilities (any application)
//! - `cli_tools`: CLI-specific helpers (command-line tools only)
//!
//! # Examples
//!
//! ```rust
//! # #[ cfg( feature = "cli_output" ) ]
//! # {
//! use cli_tools::cli_output::*;
//!
//! let config = OutputConfig::default()
//!   .with_head( 10 )
//!   .with_width( 80 );
//!
//! let result = process_output( "stdout text", "stderr text", &config );
//! println!( "{}", result.content );
//! # }
//! ```

#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_docs ) ]

/// CLI output processing.
#[ cfg( feature = "cli_output" ) ]
pub mod cli_output;

/// Namespace with dependencies.
pub mod dependency
{
  pub use ::strs_tools;
}

/// Own namespace of the module.
pub mod own
{
  #[ cfg( feature = "cli_output" ) ]
  pub use super::cli_output::orphan::*;
}

/// Orphan namespace of the module.
pub mod orphan
{
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::own::*;
}

/// Prelude to use essentials: `use cli_tools::prelude::*`.
pub mod prelude
{
  #[ cfg( feature = "cli_output" ) ]
  pub use super::cli_output::orphan::*;
}
