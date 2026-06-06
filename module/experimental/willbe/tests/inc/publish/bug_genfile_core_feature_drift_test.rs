//! Bug documentation for `genfile_core` feature drift between local and published versions.
//!
//! # Root Cause
//!
//! `genfile_core` v0.10.0 was published to crates.io (commit `f0d5e9ee`) without
//! the `enabled` feature. The `enabled` feature was added locally later
//! (commit `bb2374a2 chore: enable default features and consolidate experimental modules`)
//! as part of the workspace-wide `enabled`/`full` feature convention adoption.
//!
//! However, the version was NOT bumped when `enabled` was added, creating a silent
//! divergence between the local source and the published registry version.
//!
//! `willbe/Cargo.toml:90` references `genfile_core = { workspace = true, features = ["enabled"] }`.
//! Local builds resolve this via the `path = "module/core/genfile_core"` field (works).
//! `cargo package` strips the `path` field and resolves from the registry — finds
//! `genfile_core` 0.10.0 which has no `enabled` feature → packaging fails.
//!
//! **Error observed:**
//! ```text
//! package `willbe` depends on `genfile_core` with feature `enabled` but `genfile_core`
//! does not have that feature.
//! available features: archive, binary, default, external_content, filesystem, full, json,
//!   parameter_discovery, renderer, serialization, template, yaml
//! ```
//!
//! # Why Not Caught
//!
//! 1. **Local builds always succeeded** — `path` resolution bypasses the registry version,
//!    so `cargo build` and `cargo test` worked fine. The divergence was invisible locally.
//! 2. **No feature-sync validation** — there was no check that feature changes in a local
//!    crate are accompanied by a version bump before consumers reference the new features.
//! 3. **Single affected consumer** — only `willbe` used `features = ["enabled"]`; `genfile`
//!    used `features = ["full"]` which existed in both versions, masking the scope.
//! 4. **Publish-path testing gap** — test suite ran against the local workspace, not the
//!    registry-resolved paths that `cargo package` uses for publication readiness.
//!
//! # Fix Applied
//!
//! Bumped `genfile_core` version `0.10.0 → 0.11.0` in two files:
//! - `module/core/genfile_core/Cargo.toml:3` — crate version
//! - `Cargo.toml:334` — workspace dep constraint `~0.10.0 → ~0.11.0`
//!
//! After publishing `genfile_core` 0.11.0 (which includes `enabled`), `willbe`'s
//! `cargo package` will resolve `~0.11.0` from the registry, find the feature, and succeed.
//!
//! **Publication order:** `genfile_core` 0.11.0 must be published before `willbe`.
//!
//! # Prevention
//!
//! 1. **Version bump discipline** — any change to a crate's feature set MUST be accompanied
//!    by a version bump. Feature additions are breaking for dependents that reference them.
//! 2. **`cargo package` in CI** — running `cargo package --no-verify` on the affected crates
//!    in CI catches registry-resolution failures before they reach the publish step.
//! 3. **Feature-change checklist** — when adopting workspace feature conventions across
//!    multiple crates, bump all affected crate versions in the SAME commit that adds features.
//!
//! # Pitfall
//!
//! **Local `path` dep masks registry divergence.** When a workspace dep declares both
//! `path = "..."` and `version = "..."`, local builds use `path` (bypassing the registry).
//! `cargo package` strips `path`, exposing registry-only resolution. Feature or API changes
//! that work locally are invisible mismatches until packaging is attempted.
//!
//! **Publication ordering cascade.** Bumping `genfile_core` to `~0.11.0` in the workspace
//! dep means ALL crates inheriting that dep (including `genfile`) must also resolve `0.11.0`.
//! Publish `genfile_core` first; only then can downstream crates be packaged successfully.

use super :: *;

// test_kind: bug_reproducer(issue-genfile-core-feature-drift)

/// Verifies that `genfile_core` has the `enabled` feature declared and that its
/// version was bumped past 0.10.0 (the last published version without `enabled`).
///
/// Before fix: `genfile_core` was at 0.10.0 with no `enabled` feature — this test
/// would fail at the version assertion.
/// After fix: `genfile_core` is at 0.11.0 with `enabled` — both assertions pass.
#[ test ]
fn genfile_core_enabled_feature_exists_with_correct_version()
{
  let willbe_manifest_dir = std ::path ::Path ::new( env!( "CARGO_MANIFEST_DIR" ) );
  let workspace_root = willbe_manifest_dir
  .parent().expect( "experimental dir" )
  .parent().expect( "module dir" )
  .parent().expect( "workspace root" );

  let genfile_core_cargo = workspace_root
  .join( "module/core/genfile_core/Cargo.toml" );

  let content = std ::fs ::read_to_string( &genfile_core_cargo )
  .expect( "genfile_core/Cargo.toml must be readable" );

  // The `enabled` feature must exist in local source — registry version must match.
  assert!
  (
  content.contains( "enabled" ),
  "genfile_core must declare `enabled` feature; found none in {genfile_core_cargo:?}"
 );

  // Version must be past 0.10.0 — the last version published without `enabled`.
  // If this fails, the version was not bumped and the registry still has no `enabled`.
  assert!
  (
  !content.contains( r#"version = "0.10.0""# ),
  "genfile_core version must be bumped past 0.10.0 — 0.10.0 was published without `enabled`"
 );
}

/// Verifies that the workspace dependency constraint for `genfile_core` is past the
/// broken version (0.10.0), ensuring `cargo package` for downstream crates resolves
/// a version that has the `enabled` feature.
///
/// Before fix: workspace constraint was `~0.10.0` → resolves to 0.10.0 (no `enabled`).
/// After fix: workspace constraint is past ~0.10.0 (currently ~0.12.0, has `enabled`).
#[ test ]
fn workspace_genfile_core_dep_constraint_matches_bumped_version()
{
  let willbe_manifest_dir = std ::path ::Path ::new( env!( "CARGO_MANIFEST_DIR" ) );
  let workspace_root = willbe_manifest_dir
  .parent().expect( "experimental dir" )
  .parent().expect( "module dir" )
  .parent().expect( "workspace root" );

  let workspace_cargo = workspace_root.join( "Cargo.toml" );

  let content = std ::fs ::read_to_string( &workspace_cargo )
  .expect( "workspace Cargo.toml must be readable" );

  // Extract the genfile_core section from the workspace Cargo.toml.
  // Locate the section header and take everything up to the next section.
  let section_start = content.find( "[workspace.dependencies.genfile_core]" )
  .expect( "workspace Cargo.toml must have [workspace.dependencies.genfile_core]" );
  let section = &content[ section_start.. ];
  let section_end = section[ 1.. ]
  .find( "\n[" )
  .map_or( section.len(), | j | j + 1 );
  let genfile_section = &section[ ..section_end ];

  // Constraint must not still point at 0.10.0 (registry version without `enabled`).
  assert!
  (
  !genfile_section.contains( r#"version = "~0.10.0""# ),
  "workspace genfile_core dep must be updated past ~0.10.0 (has no `enabled`)"
 );

  // Constraint must reference some version that includes `enabled` (any version > 0.10.0).
  // The `enabled` feature was added in 0.11.0; subsequent bumps (0.12.0, etc.) retain it.
  assert!
  (
  genfile_section.contains( "version = \"~0." ),
  "workspace genfile_core dep must use a tilde version constraint"
 );
}
