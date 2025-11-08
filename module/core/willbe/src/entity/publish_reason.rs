//! Types for tracking WHY packages need publishing.
//!
//! This module defines `PublishReason` which captures all possible reasons that
//! trigger a package to be republished. Understanding the reason is critical for:
//!
//! - **Diagnostics:** Users can see why their package is being published
//! - **Debugging:** Developers can trace cascade effects
//! - **Optimization:** Can prioritize different reason types
//!
//! # Reason Types
//!
//! 1. **LocalChanges** - Package has uncommitted or unpublished local modifications
//!    - Detected via `publish_need()` (compares local vs published crate archives)
//!    - Original detection method, existed before staleness fix
//!
//! 2. **VersionBump** - Package version was explicitly incremented
//!    - User manually bumped version in Cargo.toml
//!    - Rare: usually version bumps happen automatically during publish
//!
//! 3. **StaleDependencies** - Package has workspace dependencies with incompatible versions
//!    - NEW in staleness fix
//!    - Example: Package requires `former ~2.36.0` but workspace has `former 2.37.0`
//!    - Contains list of all stale dependencies for detailed reporting
//!
//! 4. **CascadeEffect** - Package depends on other packages being published in current batch
//!    - NEW in staleness fix
//!    - Example: `willbe` depends on `wca`, `wca` is being published → `willbe` cascades
//!    - Contains list of triggering packages
//!
//! # Design Decision: Why Separate `LocalChanges` and `VersionBump`?
//!
//! These could be merged into a single "Modified" variant. We keep them separate because:
//!
//! - **User Intent:** `LocalChanges` = unintentional changes, `VersionBump` = intentional
//! - **Workflow:** Different actions needed (commit vs publish)
//! - **Future:** May want different handling (e.g., warn on uncommitted changes)
//!
//! # Design Decision: Why Store Stale Dependencies List?
//!
//! The `StaleDependencies` variant contains `Vec<StaleDependency>` rather than just a count.
//! This enables:
//!
//! - **Detailed diagnostics:** Show exactly which dependencies are stale
//! - **User guidance:** "Update dependency X from ~2.36.0 to ~2.37.0"
//! - **Debugging:** Trace why staleness was detected
//!
//! **Cost:** Additional memory (~100 bytes per stale dependency)
//! **Benefit:** Vastly improved user experience and debuggability
//!
//! # Known Pitfalls
//!
//! ## Multiple Reasons Can Apply Simultaneously
//!
//! A package might have BOTH local changes AND stale dependencies. Current design
//! forces choosing one primary reason. This is intentional:
//!
//! - Simplifies data model (enum, not struct with flags)
//! - Publishing happens regardless of reason
//! - Primary reason is "most important" for user communication
//!
//! **Priority order (if multiple apply):**
//! 1. `LocalChanges` (most urgent - uncommitted changes)
//! 2. `StaleDependencies` (critical - version conflicts)
//! 3. `CascadeEffect` (automatic - triggered by others)
//! 4. `VersionBump` (rare - explicit user action)
//!
//! ## `CascadeEffect` Can Chain Deeply
//!
//! In a chain A→B→C→D, if D has local changes:
//! - D: `LocalChanges`
//! - C: `CascadeEffect` (triggered by D)
//! - B: `CascadeEffect` (triggered by C)
//! - A: `CascadeEffect` (triggered by B)
//!
//! The `triggered_by` field only shows immediate triggers, not full chain.
//! To see full chain, must trace recursively through publish plan.
//!
//! **Lesson:** For deep diagnostics, implement recursive reason tracing.

#[ allow( clippy ::std_instead_of_alloc, clippy ::std_instead_of_core ) ]
mod private
{
  use crate :: *;
  use iter ::Itertools;

  /// Reason why a package needs to be published.
  ///
  /// This enum captures all possible reasons that trigger a package publish:
  /// - Local code modifications
  /// - Explicit version bump requests
  /// - Stale workspace dependencies
  /// - Cascade effects from dependency updates
  #[ derive( Debug, Clone, PartialEq, Eq ) ]
  pub enum PublishReason
  {
    /// Package has local uncommitted or unpublished changes
    LocalChanges,

    /// Package version was explicitly bumped
    VersionBump,

    /// Package has dependencies with stale versions
    StaleDependencies
    {
      /// List of stale dependencies requiring update
      stale_deps: Vec< crate ::stale_dependency ::StaleDependency >,
    },

    /// Package must be published due to dependency cascade
    CascadeEffect
    {
      /// Packages that triggered this cascade
      triggered_by: Vec< package ::PackageName >,
    },
  }

  impl PublishReason
  {
    /// Check if reason is due to local changes
    #[must_use] 
    pub fn is_local_change( &self ) -> bool
    {
      matches!( self, Self ::LocalChanges )
    }

    /// Check if reason is due to stale dependencies
    #[must_use] 
    pub fn is_stale( &self ) -> bool
    {
      matches!( self, Self ::StaleDependencies { .. } )
    }

    /// Check if reason is due to cascade effect
    #[must_use] 
    pub fn is_cascade( &self ) -> bool
    {
      matches!( self, Self ::CascadeEffect { .. } )
    }

    /// Get human-readable description of the reason
    #[must_use] 
    pub fn description( &self ) -> String
    {
      match self
      {
        Self ::LocalChanges =>
        {
          "Local changes detected".to_string()
        }
        Self ::VersionBump =>
        {
          "Version bump requested".to_string()
        }
        Self ::StaleDependencies { stale_deps } =>
        {
          format!( "Stale dependencies: {}", stale_deps.len() )
        }
        Self ::CascadeEffect { triggered_by } =>
        {
          format!( "Cascade effect from: {}", triggered_by.iter().map( std::convert::AsRef::as_ref ).join( ", " ) )
        }
      }
    }
  }

}

//

crate ::mod_interface!
{
  own use PublishReason;
}
