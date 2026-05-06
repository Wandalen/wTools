//! Metadata Correctness Verification Tests
//!
//! These tests verify that project metadata (Cargo.toml) is accurate and
//! synchronized with the actual implementation state.
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Expected | Status |
//! |-----------|----------|----------|--------|
//! | `test_cargo_toml_repository_url_correct` | Cargo.toml repository field | Points to `module/alias/proper_tools` | ✅ |
//!
//! ## Lessons Learned (Bugs Fixed)
//!
//! - **2026-01-05 (issue-manual-testing-3):** Cargo.toml repository URL pointed to wrong module path.
//!   Root cause: Copy-paste from template referencing module/core instead of module/alias.
//!   Prevention: Validate repository URLs match actual crate location.

use std::fs;

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
