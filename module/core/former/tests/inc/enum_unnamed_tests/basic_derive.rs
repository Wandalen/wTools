//! Purpose: Tests the `#[derive(Former)]` macro's generation of constructors for unnamed (tuple)
//! variants that return subformers, including with `#[subform_scalar]` and `#[standalone_constructors]`.
//! This file focuses on verifying the derive-based implementation.
//!
//! Coverage:
//! - Rule 3d (Tuple + Default -> Subform): Tests static method `FunctionStep::run()`.
//! - Rule 2d (Tuple + `#[subform_scalar]` -> InnerFormer): Tests static method `FunctionStep::r#break()`.
//! - Rule 4a (#[standalone_constructors]): Verifies generation of top-level constructor functions.
//! - Rule 4b (Option 2 Logic): Implicitly covered by the standalone constructor returning a subformer.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines an enum `FunctionStep` with two single-field tuple variants: `Break(Break)` and `Run(Run)`.
//! - `Break` is annotated with `#[subform_scalar]`. The enum has `#[derive(Former)]` and `#[standalone_constructors]`.
//! - Relies on the derived static methods (`FunctionStep::r#break()`, `FunctionStep::run()`) and
//!   standalone constructor (`FunctionStep::break_variant()`) defined in `basic_only_test.rs`.
//! - Asserts that these constructors return the expected subformers and that using the subformers
//!   to set fields and call `.form()` results in the correct `FunctionStep` enum instances.

use super::*;

// Define the inner structs
#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Break { pub condition : bool }

#[derive(Debug, Clone, PartialEq, former::Former)]
pub struct Run { pub command : String }

// Derive Former on the simplified enum - This should generate static methods
#[ derive( Debug, Clone, PartialEq, former::Former ) ]
// #[ debug ]
#[ former( standalone_constructors ) ]
enum FunctionStep
{
  #[ subform_scalar ]
  Break( Break ),
  Run( Run ),
}

// Include the test logic
include!( "basic_only_test.rs" );