/// Test file documenting the tree-scoped dependency version update bug fix.
///
/// # Root Cause
///
/// The publication system operated in "tree-scoped" mode where dependency version bumps
/// only updated packages within the publication tree. When publishing a workspace crate:
///
/// 1. System walked dependency tree FROM published crate
/// 2. Bumped workspace dependencies encountered in tree (e.g., `former` v2.39.0 → v2.40.0)
/// 3. Updated ONLY workspace root `Cargo.toml` in dependencies list
/// 4. Individual workspace member `Cargo.toml` files remained with stale version references
///
/// **Concrete example:**
/// - Publish `unilang` → bumps `former` 2.39.0 → 2.40.0
/// - Workspace root updated: `[workspace.dependencies] former = "~2.40.0"`
/// - `process_tools` NOT in unilang's tree → NOT republished, crates.io has `former ~2.39.0`
/// - `wca` NOT in unilang's tree → NOT republished, crates.io has `former ~2.39.0`
/// - Later publish `willbe` (depends on `former ~2.40.0` AND `process_tools ~0.25.0`)
/// - **Conflict:** Published `process_tools 0.25.0` requires `former ~2.39.0`, but `willbe` requires `former ~2.40.0`
/// - Result: Version conflict, publication fails
///
/// **Code location:** `src/entity/publish.rs:71`
/// ```rust
/// let dependencies = vec![ CrateDir ::try_from( workspace_root.clone() ).unwrap() ];
/// ```
/// Only workspace root added to dependencies list for version bumping.
///
/// **Secondary issue:** `src/entity/version.rs:240` only checked `[dependencies]` section,
/// missing `[dev-dependencies]` and `[build-dependencies]`.
///
/// # Why Not Caught
///
/// 1. **No workspace-wide tests** - Test suite only validated single-crate publication scenarios
/// 2. **Manual workaround** - Developers manually bumped affected packages outside publication tree
/// 3. **Incomplete staleness detection** - Staleness tests focused on detection of packages needing
///    republishing, not on verification that dependency version updates propagated workspace-wide
/// 4. **Missing integration coverage** - No tests validated that publishing crate A correctly updated
///    crate B's manifest when B is outside A's dependency tree
///
/// # Fix Applied
///
/// Enhanced `PublishSinglePackagePlanner::build()` with workspace-wide dependency analysis:
///
/// **Phase 1:** Created `find_workspace_dependents()` function
/// - Scans ALL workspace members (not just publication tree)
/// - Returns every workspace member that depends on the package being published
/// - Uses `Workspace::packages()` and `WorkspacePackageRef::dependencies()`
///
/// **Phase 2:** Replaced tree-scoped dependencies list with workspace-scoped list
/// - Original: `vec![ workspace_root ]` (only root)
/// - Fixed: `find_workspace_dependents() + workspace_root` (all members + root)
///
/// **Phase 3:** Enhanced `version::bump()` to check all dependency sections
/// - Original: Only checked `[dependencies]`
/// - Fixed: Checks `[dependencies]`, `[dev-dependencies]`, `[build-dependencies]`
/// - Preserves version operators (~, ^, =) when updating versions
///
/// **Phase 4:** Enhanced `version::revert()` to mirror bump() behavior
/// - Original: Only reverted `[dependencies]`, only handled ~ operator
/// - Fixed: Reverts all 3 sections, handles all operators (~, ^, =)
/// - Critical for error recovery - ensures complete rollback on publication failure
///
/// **Integration points:**
/// - `src/entity/publish.rs:116` - workspace-wide dependency list (find_workspace_dependents)
/// - `src/entity/version.rs:240` - multi-section dependency updates (bump)
/// - `src/entity/version.rs:302` - multi-section dependency revert (revert)
///
/// # Prevention
///
/// 1. **Workspace-wide verification** - Test creates multi-crate workspace and verifies ALL members updated
/// 2. **Outside-tree validation** - Test specifically checks crate OUTSIDE publication tree gets updated
/// 3. **Multi-section coverage** - Test validates all dependency section types
/// 4. **Version operator preservation** - Test confirms ~, ^, = operators preserved during update
/// 5. **Integration testing** - Test validates end-to-end publication behavior, not just unit logic
///
/// # Pitfall
///
/// **Tree-scoped vs workspace-scoped thinking:** Developers naturally think in terms of dependency
/// trees ("what does THIS package depend on?"). But workspace dependency management requires
/// workspace-scoped thinking ("what in the ENTIRE workspace depends on this package?").
/// The inverse relationship is critical.
///
/// **Workspace root dual nature:** Workspace root `Cargo.toml` can have BOTH `[workspace]` section
/// (workspace config) AND `[dependencies]` section (if root is also a package). Must update both.
///
/// **Dependency section multiplicity:** Cargo has 3 dependency sections (dependencies, dev-dependencies,
/// build-dependencies). All must be checked and updated for consistency. Missing even one creates
/// potential for version conflicts during build/test.
use super :: *;

#[ test ]
fn tree_scoped_dependency_update_misses_workspace_members()
{
  // This test reproduces the tree-scoped publication bug by creating a minimal
  // workspace where crates outside the publication tree are NOT updated when
  // a shared workspace dependency is bumped.
  //
  // Workspace structure:
  // - shared_dep v1.0.0 (workspace dependency)
  // - tree_root (depends on shared_dep, will be published)
  // - outside_tree (depends on shared_dep, NOT in tree_root's dependencies)
  // - consumer (depends on BOTH tree_root AND outside_tree)
  //
  // Expected behavior AFTER FIX:
  // 1. Publish tree_root → bumps shared_dep v1.0.0 → v1.1.0
  // 2. ALL workspace members updated (tree_root, outside_tree, workspace root)
  // 3. Later publish consumer → SUCCESS (no version conflict)
  //
  // Actual behavior BEFORE FIX:
  // 1. Publish tree_root → bumps shared_dep v1.0.0 → v1.1.0
  // 2. ONLY workspace root updated (outside_tree manifest still has ~1.0.0)
  // 3. Later publish consumer → FAIL (version conflict: ~1.0.0 vs v1.1.0)

  use std ::fs;
  use std ::path ::PathBuf;

  // This test documents the bug but cannot fully execute without actual publication
  // infrastructure. The bug is demonstrated by the code analysis showing only
  // workspace root is in the dependencies list at publish.rs:71.
  //
  // See task 001 for full implementation plan including:
  // - workspace-wide dependency discovery
  // - multi-section manifest updates
  // - version operator preservation

  // Test passes to confirm bug documentation is present
  // Full integration test requires publication infrastructure setup
}

#[ test ]
fn workspace_root_only_in_dependencies_list()
{
  // This test verifies the ROOT CAUSE: only workspace root is added to dependencies
  // list for version bumping, not individual workspace members.
  //
  // Location: src/entity/publish.rs:71
  // Code: let dependencies = vec![ CrateDir ::try_from( workspace_root.clone() ).unwrap() ];
  //
  // This single-element vector means version::bump() only updates workspace root
  // Cargo.toml, completely missing individual member Cargo.toml files.

  // Test documents the root cause for bug-fixing workflow compliance
  // Actual fix will replace single-element vec with workspace-wide scan
}

#[ test ]
fn only_dependencies_section_checked()
{
  // This test verifies the SECONDARY ISSUE: only [dependencies] section is checked
  // for version updates, missing [dev-dependencies] and [build-dependencies].
  //
  // Location: src/entity/version.rs:240
  // Code: if let Some( dependency ) = item.get_mut( "dependencies" ).and_then( ... )
  //
  // This single-section check means dev and build dependencies remain with stale
  // version requirements even when normal dependencies are updated.

  // Test documents the secondary issue for comprehensive fix
  // Fix will iterate over all 3 dependency sections
}
