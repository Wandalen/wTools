#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
// Purpose: Tests the `#[ derive( former::Former ) ]` macro's generation of subformer starter methods for an enum
// with multiple single-field tuple variants, where the inner types also derive `former::Former`. This file
// focuses on verifying the derive-based implementation.
//
// Coverage:
// - Rule 3d (Tuple + Single-Field + Default): Verifies that for single-field tuple variants without specific attributes, the derived constructor is a subformer starter method.
// - Rule 4b (Option 2 Logic): Demonstrates the usage of the subformer mechanism for multiple variants, allowing nested building of inner types.
//
// Test Relevance/Acceptance Criteria:
// - Defines an enum `FunctionStep` with multiple single-field tuple variants (`Prompt`, `Break`, `InstructionsApplyToFiles`, `Run`).
// - The inner types (`Prompt`, `Break`, etc.) also derive `former::Former`.
// - Applies `#[ derive( former::Former ) ]` to the `FunctionStep` enum.
// - Includes shared test logic from `usecase1_only_test.rs`.
// - The included tests call the derived static methods (e.g., `FunctionStep::prompt()`, `FunctionStep::r#break()`), use the returned subformers to set fields of the inner types, and call `.form()` on the subformers to get the final `FunctionStep` enum instance.
// - Asserts that the resulting enum instances match manually constructed expected values. This verifies that the derived subformer starters correctly integrate with the inner types' formers.

#[ allow( unused_imports ) ]
use super::*;
use former::Former;
use former::FormerBegin;

// Define the inner structs that the enum variants will hold.
// These need to derive Former themselves if you want to build them easily.
// Re-enabled Former derive - trailing comma issue appears to be fixed
#[ derive( Debug, Clone, PartialEq, Default, former::Former ) ]
pub struct Prompt { pub content: String }

#[ derive( Debug, Clone, PartialEq, Default, former::Former ) ]
pub struct Break { pub condition: bool }

// Re-enabled Former derive - trailing comma issue appears to be fixed

#[ derive( Debug, Clone, PartialEq, Default, former::Former ) ]
pub struct InstructionsApplyToFiles { pub instruction: String }

#[ derive( Debug, Clone, PartialEq, Default, former::Former ) ]
pub struct Run { pub command: String }

// Derive former::Former on the enum.
// By default, this should generate subformer starter methods for each variant.
// Re-enabled Former derive - trailing comma issue appears to be fixed
#[ derive( Debug, Clone, PartialEq, former::Former ) ]
// #[ debug ]
pub enum FunctionStep
{
  Prompt(Prompt),
  Break(Break),
  InstructionsApplyToFiles(InstructionsApplyToFiles),
  Run(Run),
}

include!("usecase1_only_test.rs");