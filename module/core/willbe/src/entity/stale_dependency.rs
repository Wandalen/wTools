//! Types for representing stale workspace dependencies.
//!
//! A dependency becomes "stale" when the workspace version no longer satisfies
//! the package's version requirement, or when the dependency is being published
//! in the current batch.
//!
//! # Core Types
//!
//! - `StaleDependency` - Captures all information about a stale dependency
//! - `StaleReason` - Distinguishes WHY the dependency is stale
//!
//! # Staleness Scenarios
//!
//! ## Scenario 1: Incompatible Version
//!
//! Package `wca` requires `former ~2.36.0` (meaning >=2.36.0, <2.37.0).
//! Workspace has `former 2.37.0`.
//!
//! ```text
//! StaleDependency {
//!   name: "former",
//!   required: ~2.36.0,
//!   workspace_version: 2.37.0,
//!   reason: IncompatibleVersion,
//! }
//! ```
//!
//! `is_compatible()` returns `false` because 2.37.0 doesn't satisfy ~2.36.0.
//!
//! ## Scenario 2: Being Published
//!
//! Package `willbe` requires `wca ~0.37.0`.
//! Workspace has `wca 0.37.0` (versions match!).
//! BUT `wca` is in current publish batch.
//!
//! ```text
//! StaleDependency {
//!   name: "wca",
//!   required: ~0.37.0,
//!   workspace_version: 0.37.0,
//!   reason: BeingPublished,
//! }
//! ```
//!
//! `is_compatible()` returns `true` (version matches), but dependency is STILL stale
//! because `wca` is being republished. `willbe` must also be republished to reference
//! the fresh `wca` version, not the old one from crates.io.
//!
//! # Design Decision: Why Two Separate Staleness Reasons?
//!
//! We could use a single `Stale` reason. Separating into IncompatibleVersion and
//! BeingPublished enables:
//!
//! - **Different User Messages:**
//!   - IncompatibleVersion: "Update your dependency requirement"
//!   - BeingPublished: "Dependency being published, cascade required"
//!
//! - **Different Handling:**
//!   - IncompatibleVersion: May want to auto-fix requirement
//!   - BeingPublished: Always requires republishing
//!
//! - **Debugging:**
//!   - IncompatibleVersion: Semver issue
//!   - BeingPublished: Batch publishing issue
//!
//! # Known Pitfalls
//!
//! ## BeingPublished Can Have Compatible Versions
//!
//! Confusing scenario: `is_compatible()` returns `true` but dependency is still stale.
//!
//! ```rust,ignore
//! let stale = StaleDependency {
//!   required: VersionReq::parse("~2.37.0").unwrap(),
//!   workspace_version: Version::parse("2.37.0").unwrap(),
//!   reason: StaleReason::BeingPublished,
//! };
//!
//! assert!(stale.is_compatible());  // true!
//! // But still need to republish because dependency in publish batch
//! ```
//!
//! **Lesson:** `is_compatible()` only checks version matching, NOT whether republishing
//! is required. Always check `reason` field.
//!
//! ## Requirement vs Workspace Version Confusion
//!
//! Which is "newer"?
//! - `required: ~2.36.0` (package's Cargo.toml)
//! - `workspace_version: 2.37.0` (workspace's current version)
//!
//! Answer: Workspace version (2.37.0) is newer. Package's requirement (~2.36.0) is
//! STALE because it references an old version.
//!
//! **Common mistake:** Thinking requirement is what package needs, so it's "correct".
//! Actually, requirement is STALE when it doesn't match current workspace reality.
//!
//! ## String Conversion Pitfalls
//!
//! Converting between `PackageName` (newtype) and `String`:
//!
//! ```rust,ignore
//! let dep_name: String = dep.name().to_string();  // String
//! let pkg_name: PackageName = dep_name.into();    // PackageName
//! ```
//!
//! In `StaleDependency`, `name` is `PackageName` (newtype), not `String`.
//! When comparing with HashMap keys (often `String`), must convert properly.
//!
//! **Lesson:** Be explicit about `String` vs `PackageName` conversions.

#[ allow( clippy ::std_instead_of_alloc, clippy ::std_instead_of_core ) ]
mod private
{
  use crate :: *;
  use semver :: { Version, VersionReq };

  /// Reason why a dependency is considered stale.
  #[ derive( Debug, Clone, PartialEq, Eq ) ]
  pub enum StaleReason
  {
    /// Workspace version doesn't satisfy package's version requirement
    IncompatibleVersion,

    /// Dependency is being published in current batch
    BeingPublished,
  }

  /// Represents a workspace dependency that has become stale.
  ///
  /// A dependency is stale when:
  /// - Its workspace version doesn't satisfy the package's version requirement, OR
  /// - It's being published in the current batch
  #[ derive( Debug, Clone, PartialEq, Eq ) ]
  pub struct StaleDependency
  {
    /// Name of the stale dependency
    pub name: package ::PackageName,

    /// Version requirement from package's Cargo.toml
    pub required: VersionReq,

    /// Current version in workspace
    pub workspace_version: Version,

    /// Reason for staleness
    pub reason: StaleReason,
  }

  impl StaleDependency
  {
    /// Check if workspace version satisfies the requirement.
    ///
    /// Returns `true` if the version requirement is compatible with the workspace version.
    /// For example:
    /// - `^2.36.0` matches `2.36.1` → compatible
    /// - `~2.36.0` doesn't match `2.37.0` → incompatible
    #[must_use] 
    pub fn is_compatible( &self ) -> bool
    {
      self.required.matches( &self.workspace_version )
    }

    /// Get human-readable description of the staleness.
    #[must_use] 
    pub fn description( &self ) -> String
    {
      match self.reason
      {
        StaleReason ::IncompatibleVersion =>
        {
          format!
          (
            "{}: required {} but workspace has {}",
            self.name.as_ref(),
            self.required,
            self.workspace_version
          )
        }
        StaleReason ::BeingPublished =>
        {
          format!
          (
            "{} is being published in current batch",
            self.name.as_ref()
          )
        }
      }
    }
  }
}

//

crate ::mod_interface!
{
  own use StaleReason;
  own use StaleDependency;
}
