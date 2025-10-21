//! genfile - CLI for template archive management
//!
//! This crate provides a command-line interface to genfile_core, enabling users to:
//! - Create and manage template archives
//! - Add files and define parameters
//! - Materialize templates with custom values
//! - Pack/unpack portable archives
//!
//! # Quick Start
//!
//! ```bash
//! # Create portable archive from directory
//! genfile .pack input::"./my-template" output::"template.json"
//!
//! # Load and materialize
//! genfile .archive.load path::"template.json"
//! genfile .value.set name::"project_name" value::"my-project"
//! genfile .materialize destination::"./output"
//! ```

#![deny( missing_docs )]
#![allow( unused_imports )]

#[ cfg( feature = "enabled" ) ]
pub mod commands;
#[ cfg( feature = "enabled" ) ]
pub mod handlers;
#[ cfg( feature = "enabled" ) ]
pub mod state;
#[ cfg( feature = "enabled" ) ]
pub mod error;
#[ cfg( feature = "enabled" ) ]
pub mod repl;

/// Prelude with commonly used items
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{
  #[ doc( inline ) ]
  pub use super::
  {
    commands,
    handlers,
    state,
    error,
    repl,
  };
}
