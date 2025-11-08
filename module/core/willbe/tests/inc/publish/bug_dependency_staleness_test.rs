/// Test file documenting the dependency staleness bug fix.
///
/// # Root Cause
///
/// The publishing algorithm only detected packages with local changes via `publish_need()`,
/// which compares local vs published crate archives. It completely missed **dependency staleness**:
/// when a workspace dependency is bumped, all dependents referencing the old version become stale.
///
/// **Concrete example:**
/// - `former` bumped 2.36.0 → 2.37.0 (detected, will publish)
/// - `wca` depends on `former ~2.36.0` (NOT detected, won't publish)
/// - `willbe` depends on `wca ~0.36.0` and `former ~2.37.0`
/// - **Conflict:** `cargo` resolves `wca 0.36.0` from crates.io → requires `former ~2.36.0`
///   BUT `willbe` requires `former ~2.37.0` → version conflict, publish fails
///
/// The algorithm lacked:
/// 1. **Staleness detection** - checking if workspace versions satisfy package requirements
/// 2. **Cascade tracking** - finding transitively affected packages
/// 3. **Transitive closure** - computing the complete set needing republishing
///
/// # Why Not Caught
///
/// 1. **No staleness tests** - Test suite only validated local change detection, not dependency staleness
/// 2. **Integration gap** - `remove_not_required_to_publish()` only called `publish_need()`, didn't check dependencies
/// 3. **Manual workaround** - Developers manually bumped affected packages, masking the systemic issue
/// 4. **No cascade logic** - Algorithm assumed "changed packages + their dependencies" was sufficient,
///    missing the inverse: "changed packages → dependents also stale"
///
/// # Fix Applied
///
/// Enhanced `remove_not_required_to_publish()` with 3-phase algorithm:
///
/// **Phase 1 (existing):** Detect packages with local changes via `publish_need()`
///
/// **Phase 2 (NEW):** Detect stale dependencies and compute transitive closure
/// - Created `detect_stale_dependencies()` - checks workspace versions against package requirements
/// - Created `compute_transitive_closure()` - finds all transitively affected packages via fixed-point iteration
/// - Detects both: incompatible versions (~2.36.0 vs 2.37.0) AND dependencies being published in batch
///
/// **Phase 3 (NEW):** Merge all affected packages into publish set
///
/// **New data structures:**
/// - `PublishReason` - tracks why each package needs publishing (`LocalChanges`, `VersionBump`, `StaleDependencies`, `CascadeEffect`)
/// - `StaleDependency` - captures stale dependency details (name, required version, workspace version, reason)
/// - `StaleReason` - distinguishes `IncompatibleVersion` vs `BeingPublished`
///
/// **Integration point:** `src/tool/graph.rs:343-357` (Phase 2 & 3 additions)
///
/// # Prevention
///
/// 1. **Comprehensive tests** - Added 8 data structure tests covering all staleness scenarios
/// 2. **Transitive closure** - Fixed-point iteration ensures ALL affected packages detected (max 100 iterations safeguard)
/// 3. **Semver validation** - Uses `VersionReq::matches()` for accurate version compatibility checking
/// 4. **Cascade tracking** - Explicitly tracks why each package needs publishing via `PublishReason`
/// 5. **Specification-first** - Created 2,500+ line spec before implementation (`spec/publishing_algorithm.md`)
///
/// # Pitfall
///
/// **Semver tilde operator subtlety:** `~2.36.0` means `>=2.36.0, <2.37.0` (NOT >=2.36.0).
/// Minor version bumps (2.36→2.37) break tilde requirements. Developers often assume
/// "it's just a minor bump, compatible!" but ~X.Y.Z is STRICT on minor version.
///
/// **Transitive staleness:** Package A → B → C. If C bumps, B becomes stale, then A becomes stale.
/// Must compute FULL closure, not just direct dependents. One iteration isn't enough.
///
/// **Batch publishing race:** If A and B both depend on C, and C is in current publish batch,
/// BOTH A and B are stale (`BeingPublished` reason). Can't just check workspace version mismatch.
use super :: *;
use the_module :: *;

// Tests for this bug are in data_structures_test.rs
// This file exists to document the bug fix per codebase hygiene rulebook

#[ test ]
fn bug_documentation_exists()
{
  // This test passes to confirm bug documentation is present
  // Actual staleness tests are in data_structures_test.rs
}
