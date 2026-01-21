//! Metadata Correctness Verification Tests
//!
//! These tests verify that project metadata (Cargo.toml, spec.md, readme.md)
//! is accurate and synchronized with the actual implementation state.
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Expected | Status |
//! |-----------|----------|----------|--------|
//! | `test_cargo_toml_repository_url_correct` | Cargo.toml repository field | Points to `module/alias/proper_tools` | ✅ |
//! | `test_spec_examples_status_accurate` | spec.md examples section | Accurately reflects existing examples | ✅ |
//! | `test_spec_readme_claims_accurate` | spec.md readme claims | Doesn't claim sections are commented out when active | ✅ |
//!
//! ## Lessons Learned (Bugs Fixed)
//!
//! - **2026-01-05 (issue-manual-testing-1):** spec.md claimed examples don't exist when they do.
//!   Root cause: spec.md written before examples were created, never updated after implementation.
//!   Prevention: Add metadata verification tests to catch stale documentation claims.
//!
//! - **2026-01-05 (issue-manual-testing-2):** spec.md claimed readme examples section was commented out.
//!   Root cause: spec.md documented old readme state, never synchronized after readme updates.
//!   Prevention: Verify spec.md claims against actual file states in tests.
//!
//! - **2026-01-05 (issue-manual-testing-3):** Cargo.toml repository URL pointed to wrong module path.
//!   Root cause: Copy-paste from template referencing module/core instead of module/alias.
//!   Prevention: Validate repository URLs match actual crate location.

use std::fs;
use std::path::Path;

/// Verifies Cargo.toml repository field points to correct module path
///
/// # Root Cause
///
/// In `Cargo.toml:11`, the `repository` field contained
/// `"https://github.com/Wandalen/wTools/tree/master/module/core/proper_tools"`,
/// pointing to `module/core/` instead of `module/alias/` where the crate actually exists.
/// This occurred because the Cargo.toml was likely created from a template that referenced
/// a core module, and the path wasn't updated when creating this alias module.
///
/// # Why Not Caught
///
/// No existing tests validated Cargo.toml metadata correctness. The repository field
/// is metadata-only and doesn't affect compilation, so it wasn't caught by cargo build,
/// clippy, or existing integration tests. Manual inspection during code review would
/// catch this, but no automated verification existed.
///
/// # Fix Applied
///
/// Changed `Cargo.toml:11` from:
/// ```toml
/// repository = "https://github.com/Wandalen/wTools/tree/master/module/core/proper_tools"
/// ```
/// to:
/// ```toml
/// repository = "https://github.com/Wandalen/wTools/tree/master/module/alias/proper_tools"
/// ```
///
/// This aligns the repository URL with the actual crate location in the wTools repository.
///
/// # Prevention
///
/// 1. **Metadata verification tests**: Add tests validating Cargo.toml fields match reality
/// 2. **Template checklist**: When creating new crates from templates, verify all path references
/// 3. **CI check**: Add linter rule to verify repository URLs match actual crate locations
/// 4. **Code review**: Check repository/homepage/documentation URLs in Cargo.toml reviews
///
/// # Pitfall
///
/// Similar pattern may exist in other alias modules if created from the same template.
/// Audit all module/alias/*/Cargo.toml files for repository URLs pointing to module/core.
/// Watch for copy-paste errors when creating new modules from templates.
#[ test ]
fn test_cargo_toml_repository_url_correct()
{
  let cargo_toml_path = concat!( env!( "CARGO_MANIFEST_DIR" ), "/Cargo.toml" );
  let cargo_toml_content = fs::read_to_string( cargo_toml_path )
    .expect( "Failed to read Cargo.toml" );

  // Verify repository URL points to module/alias/proper_tools, not module/core
  assert!(
    cargo_toml_content.contains( "module/alias/proper_tools" ),
    "Cargo.toml repository field should point to module/alias/proper_tools, not module/core"
  );

  // Ensure it doesn't incorrectly point to module/core
  assert!(
    !cargo_toml_content.contains( "module/core/proper_tools" ),
    "Cargo.toml repository field incorrectly points to module/core/proper_tools"
  );
}

/// Verifies spec.md accurately reflects that examples DO exist
///
/// # Root Cause
///
/// In `spec.md` around lines 151-158, the specification claimed:
/// ```
/// **Status:** No examples exist
/// **Issues:**
/// - ❌ readme.md:32 references non-existent examples/proper_tools_trivial.rs
/// - ❌ No examples/ directory
/// - ❌ No usage demonstrations
/// ```
///
/// This was accurate when spec.md was originally written (before examples were created),
/// but became stale after `examples/proper_tools_trivial.rs` was implemented. The spec
/// was never updated to reflect that examples now exist and work correctly.
///
/// # Why Not Caught
///
/// No tests validated spec.md accuracy against actual implementation state. Specification
/// documents are typically documentation-only and don't affect compilation. The disconnect
/// between spec claims and reality went unnoticed because no automated verification existed.
///
/// # Fix Applied
///
/// Updated `spec.md` lines 151-183 to accurately reflect current state:
/// - Changed status from "No examples exist" to "Examples implemented"
/// - Removed outdated issue markers claiming examples don't exist
/// - Updated to reflect examples/ directory exists and contains working example
/// - Documented example demonstrates placeholder functionality correctly
///
/// # Prevention
///
/// 1. **Spec verification tests**: Add tests checking spec.md claims match implementation
/// 2. **Documentation review**: When adding features, update spec.md in same PR/commit
/// 3. **Automated checks**: Parse spec.md and verify claims against actual files
/// 4. **Spec timestamps**: Add "Last Updated" markers to spec sections
///
/// # Pitfall
///
/// Any time new functionality is added (examples, tests, features), spec.md must be
/// updated simultaneously. Don't leave spec updates for "later" - they get forgotten.
/// Similar staleness likely exists in other spec.md sections (check Implementation
/// Roadmap checkboxes, Open Questions section).
#[ test ]
fn test_spec_examples_status_accurate()
{
  let spec_path = concat!( env!( "CARGO_MANIFEST_DIR" ), "/spec.md" );
  let spec_content = fs::read_to_string( spec_path )
    .expect( "Failed to read spec.md" );

  // Verify spec doesn't claim examples don't exist (they DO exist)
  assert!(
    !spec_content.contains( "**Status:** No examples exist" ),
    "spec.md incorrectly claims no examples exist - examples/proper_tools_trivial.rs exists!"
  );

  // Verify spec doesn't claim examples/ directory is missing
  assert!(
    !spec_content.contains( "No `examples/` directory" ) ||
    spec_content.contains( "examples/ directory exists" ),
    "spec.md should not claim examples/ directory is missing"
  );

  // Verify the example file actually exists
  let example_path = concat!( env!( "CARGO_MANIFEST_DIR" ), "/examples/proper_tools_trivial.rs" );
  assert!(
    Path::new( example_path ).exists(),
    "examples/proper_tools_trivial.rs should exist"
  );
}

/// Verifies spec.md doesn't falsely claim readme sections are commented out
///
/// # Root Cause
///
/// In `spec.md:234`, the specification claimed:
/// ```
/// - Examples section commented out (lines 10-33)
/// ```
///
/// This referred to an old state of `readme.md` where the examples section was initially
/// commented out or placeholder. After readme.md was updated to include an active examples
/// section (lines 45-47 with link to `proper_tools_trivial`), spec.md was not synchronized
/// to reflect this change.
///
/// # Why Not Caught
///
/// No tests cross-referenced spec.md claims against actual readme.md state. The spec
/// documented historical issues that were subsequently fixed, but the spec itself wasn't
/// updated to mark those issues as resolved or remove outdated claims.
///
/// # Fix Applied
///
/// Updated `spec.md` around line 234 to remove or correct the outdated claim about
/// commented-out sections. Modified the documentation section to accurately reflect
/// that readme.md now has an active Examples section linking to `proper_tools_trivial`.
///
/// # Prevention
///
/// 1. **Cross-reference validation**: Test spec.md claims against actual file content
/// 2. **Issue tracking**: Mark spec.md issues as [RESOLVED] when fixed rather than deleting
/// 3. **Synchronization policy**: Update spec.md when fixing issues it documents
/// 4. **Automated parsing**: Parse spec.md issue markers and verify against reality
///
/// # Pitfall
///
/// Specifications often document problems/issues. When those problems are fixed in code,
/// the spec must be updated to reflect resolution. Leaving stale issue markers in spec
/// creates confusion about project state. Audit spec.md for other outdated issue claims.
#[ test ]
fn test_spec_readme_claims_accurate()
{
  let spec_path = concat!( env!( "CARGO_MANIFEST_DIR" ), "/spec.md" );
  let spec_content = fs::read_to_string( spec_path )
    .expect( "Failed to read spec.md" );

  let readme_path = concat!( env!( "CARGO_MANIFEST_DIR" ), "/readme.md" );
  let readme_content = fs::read_to_string( readme_path )
    .expect( "Failed to read readme.md" );

  // Verify spec doesn't incorrectly claim readme examples section is commented out
  if spec_content.contains( "Examples section commented out" )
  {
    // If spec makes this claim, verify it's actually true (it shouldn't be)
    assert!(
      !readme_content.contains( "### Examples" ) ||
      readme_content.contains( "<!--" ) && readme_content.contains( "Examples" ) &&
      readme_content.find( "<!--" ).unwrap() < readme_content.find( "### Examples" ).unwrap(),
      "spec.md claims Examples section is commented out, but readme.md has active Examples section!"
    );
  }

  // Verify readme actually has examples section (should be active, not commented)
  assert!(
    readme_content.contains( "### Examples" ),
    "readme.md should have Examples section"
  );

  // Verify examples section references the actual example file
  assert!(
    readme_content.contains( "proper_tools_trivial" ),
    "readme.md Examples section should reference proper_tools_trivial example"
  );
}
