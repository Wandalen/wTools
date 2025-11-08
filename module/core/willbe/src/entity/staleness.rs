//! Dependency staleness detection and transitive closure computation.
//!
//! This module implements the core fix for the dependency staleness bug that caused
//! version conflicts during publishing. When a workspace dependency is bumped, all
//! packages referencing the old version become "stale" and must be republished.
//!
//! # Problem
//!
//! The original publishing algorithm only detected packages with local changes
//! (via `publish_need()` which compares local vs published crate archives). It
//! completely missed packages whose dependencies were bumped but had no local changes.
//!
//! **Example failure:**
//! ```text
//! former: 2.36.0 → 2.37.0 (detected: version bump)
//! wca: requires former ~2.36.0 (NOT detected: no local changes)
//! willbe: requires wca ~0.36.0, former ~2.37.0
//!
//! Result: cargo resolves wca 0.36.0 from crates.io → requires former ~2.36.0
//!         BUT former 2.37.0 already published
//!         → VERSION CONFLICT ❌
//! ```
//!
//! # Solution
//!
//! Two-phase detection algorithm:
//!
//! **Phase 1: Staleness Detection** - `detect_stale_dependencies()`
//! - For each package, check all workspace dependencies
//! - Mark as stale if: workspace version doesn't satisfy package requirement
//! - Mark as stale if: dependency is in current publishing batch
//!
//! **Phase 2: Transitive Closure** - `compute_transitive_closure()`
//! - Start with initial set (packages with local changes)
//! - Iterate: find packages with stale dependencies, add to set
//! - Repeat until fixed point (no new packages added)
//! - Safeguard: MAX_ITERATIONS=100 to prevent infinite loops
//!
//! # Known Pitfalls
//!
//! ## Semver Tilde Operator Strictness
//!
//! The tilde operator `~X.Y.Z` is STRICT on minor version, not lenient like caret `^X.Y.Z`.
//!
//! - `~2.36.0` means `>=2.36.0, <2.37.0` (NOT just >=2.36.0)
//! - Minor version bumps (2.36→2.37) BREAK tilde requirements
//! - Many developers assume tilde is lenient, causing surprise when dependencies break
//!
//! **Lesson:** After ANY minor version bump, check ALL dependents using tilde requirements.
//!
//! ## Transitive Staleness Requires Iteration
//!
//! One pass over packages is insufficient. Consider the chain A→B→C:
//!
//! - Iteration 1: C bumped → B becomes stale (direct dependency)
//! - Iteration 2: B being published → A becomes stale (depends on B)
//! - Convergence: Both B and A must be republished
//!
//! **Lesson:** Must iterate to fixed point. Single-pass algorithms WILL miss packages.
//!
//! ## Batch Publishing Creates Additional Staleness
//!
//! During batch publishing, packages depending on ANYTHING in the batch become stale,
//! even if versions match:
//!
//! ```text
//! Scenario: A and B both depend on C, C being published
//! Result: BOTH A and B are stale (BeingPublished reason)
//! ```
//!
//! **Why:** If A publishes with old C (from crates.io), but C is about to publish
//! new version, dependency resolution fails. Must use fresh versions.
//!
//! **Lesson:** Check for BeingPublished staleness SEPARATELY from version incompatibility.
//!
//! ## Fixed-Point Iteration Can Be Slow
//!
//! In pathological cases (deep dependency chains, large workspaces), convergence
//! may take many iterations:
//!
//! - Typical case (5-10 packages): 2-3 iterations
//! - Large workspace (100+ packages): 5-10 iterations
//! - Pathological (deep chains): 20+ iterations
//!
//! **Mitigation:** MAX_ITERATIONS=100 safeguard prevents infinite loops but allows
//! even worst-case scenarios to complete.
//!
//! # Architecture Decision: Why Two Separate Functions?
//!
//! `detect_stale_dependencies()` and `compute_transitive_closure()` could be merged
//! into a single function. We keep them separate because:
//!
//! 1. **Testability:** Can test staleness detection independently from closure computation
//! 2. **Clarity:** Each function has single, clear responsibility
//! 3. **Reusability:** Staleness detection useful for diagnostics without full closure
//! 4. **Performance:** Can optimize each phase independently
//!
//! # Integration Point
//!
//! This module is called from `tool::graph::remove_not_required_to_publish()`:
//!
//! ```rust,ignore
//! // Phase 1: Detect local changes (existing)
//! let packages_with_changes = detect_via_publish_need();
//!
//! // Phase 2: Compute transitive closure (NEW - this module)
//! let packages_to_publish = compute_transitive_closure(workspace, &packages_with_changes);
//!
//! // Phase 3: Add all to publish set (existing)
//! for package in packages_to_publish { ... }
//! ```

#[ allow( clippy ::std_instead_of_alloc, clippy ::std_instead_of_core ) ]
mod private
{
  use crate :: *;
  use semver ::{ Version, VersionReq };
  use std ::str ::FromStr;

  /// Compute the transitive closure of packages that need publishing.
  ///
  /// Given an initial set of packages to publish, find all packages that
  /// transitively depend on them. This implements the cascade effect where
  /// package A must be republished if its dependency B is being published.
  ///
  /// Uses fixed-point iteration with a maximum iteration limit to ensure termination.
  ///
  /// # Arguments
  ///
  /// * `workspace` - The cargo workspace to analyze
  /// * `initial_set` - Initial set of package names to publish
  ///
  /// # Returns
  ///
  /// Complete set of package names that need publishing (including cascades)
  #[must_use] 
  pub fn compute_transitive_closure
  (
    workspace: &Workspace,
    initial_set: &collection ::HashSet< String >,
  )
  -> collection ::HashSet< String >
  {
    const MAX_ITERATIONS: usize = 100;
    let mut publishing = initial_set.clone();
    let mut iteration = 0;

    loop
    {
      iteration += 1;
      if iteration > MAX_ITERATIONS
      {
        // Safety guard against infinite loops
        break;
      }

      let before_size = publishing.len();

      // Find packages with stale dependencies due to current publish set
      let stale_map = detect_stale_dependencies( workspace, &publishing );

      // Add all packages with stale dependencies to publish set
      for package_name in stale_map.keys()
      {
        publishing.insert( package_name.clone() );
      }

      // Fixed point reached - no new packages added
      if publishing.len() == before_size
      {
        break;
      }
    }

    publishing
  }

  /// Detect packages with stale workspace dependencies.
  ///
  /// A dependency is stale if:
  /// - Its workspace version doesn't satisfy the package's version requirement
  /// - It's being published in the current publishing batch
  ///
  /// # Arguments
  ///
  /// * `workspace` - The cargo workspace to analyze
  /// * `publishing` - Set of package names currently being published
  ///
  /// # Returns
  ///
  /// `HashMap` mapping package names to their list of stale dependencies
  #[must_use] 
  pub fn detect_stale_dependencies
  (
    workspace: &Workspace,
    publishing: &collection ::HashSet< String >,
  )
  -> collection ::HashMap< String, Vec< stale_dependency ::StaleDependency > >
  {
    let mut stale_map = collection ::HashMap ::new();

    // Get all workspace packages and their versions
    let workspace_versions: collection ::HashMap< String, Version > = workspace
      .packages()
      .map
      (
        | p |
        {
          let name = p.name().to_string();
          let version = p.version();
          ( name, version )
        }
      )
      .collect();

    // Check each package for stale dependencies
    for package in workspace.packages()
    {
      let package_name = package.name().to_string();
      let mut stale_deps = Vec ::new();

      // Check all dependency types
      for dep in package.dependencies()
      {
        let dep_name = dep.name().clone();

        // Only check workspace dependencies
        if let Some( workspace_version ) = workspace_versions.get( &dep_name )
        {
          let req_str = dep.req().to_string();
          let required = match VersionReq ::from_str( &req_str )
          {
            Ok( req ) => req,
            Err( _ ) => continue, // Skip invalid version requirements
          };

          // Check if dependency is being published
          if publishing.contains( &dep_name )
          {
            stale_deps.push
            (
              stale_dependency ::StaleDependency
              {
                name: dep_name.clone().into(),
                required: required.clone(),
                workspace_version: workspace_version.clone(),
                reason: stale_dependency ::StaleReason ::BeingPublished,
              }
            );
          }
          // Check if workspace version satisfies requirement
          else if !required.matches( workspace_version )
          {
            stale_deps.push
            (
              stale_dependency ::StaleDependency
              {
                name: dep_name.clone().into(),
                required,
                workspace_version: workspace_version.clone(),
                reason: stale_dependency ::StaleReason ::IncompatibleVersion,
              }
            );
          }
        }
      }

      if !stale_deps.is_empty()
      {
        stale_map.insert( package_name, stale_deps );
      }
    }

    stale_map
  }
}

//

crate ::mod_interface!
{
  own use compute_transitive_closure;
  own use detect_stale_dependencies;
}
