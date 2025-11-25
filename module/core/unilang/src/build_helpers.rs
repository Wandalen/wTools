//! Build-time helper utilities for static registry generation
//!
//! This module provides build-time analysis and validation for command definitions,
//! including type hint suggestions and YAML validation.

/// Type analyzer for detecting type issues in arguments
pub mod type_analyzer;

/// Hint generator for formatting type hint messages
pub mod hint_generator;

mod private
{
  #[ allow( unused_imports ) ]
  use crate::*;

  pub use super::type_analyzer::{ TypeAnalyzer, TypeHint, Severity };
  pub use super::hint_generator::HintGenerator;
}

// Direct exports
pub use private::
{
  TypeAnalyzer,
  TypeHint,
  Severity,
  HintGenerator,
};

/// Orphaned stuff.
pub mod orphan
{
  pub use super::
  {
    type_analyzer,
    hint_generator,
  };

  pub use super::private::
  {
    TypeAnalyzer,
    TypeHint,
    Severity,
    HintGenerator,
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
    TypeAnalyzer,
    TypeHint,
    Severity,
    HintGenerator,
  };
}
