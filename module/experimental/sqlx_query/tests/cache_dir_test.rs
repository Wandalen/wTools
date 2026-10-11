//! Where `sqlx_query` looks for a query's cache entry, and where a live build
//! writes one -- both following sqlx's own resolution: `SQLX_OFFLINE_DIR`
//! (relative to the workspace root), then `<crate>/.sqlx`, then
//! `<workspace root>/.sqlx`, searched per query, with the workspace root
//! as Cargo itself determines it.
//!
//! The write target is observed through `SQLX_QUERY_SOFT_WARN=1`, whose
//! `cargo:rustc-env=SQLX_OFFLINE_DIR=` names it.
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Input | Expected | Status |
//! |-----------|----------|-------|----------|--------|
//! | `missing_cache_dir_never_forces_offline` | Crate never ran `cargo sqlx prepare` | no `.sqlx/` anywhere | no `rustc-env` | ✅ |
//! | `missing_cache_dir_is_reported` | Same | no `.sqlx/` anywhere | one warning naming the fix | ✅ |
//! | `local_cache_dir_is_preferred_over_the_workspace_one` | Both exist | member and root `.sqlx/` | writes into the member's | ✅ |
//! | `workspace_cache_dir_is_used_without_a_local_one` | Workspace-shared cache | root `.sqlx/` only | writes into the root's | ✅ |
//! | `workspace_entry_mre_counts_as_fresh_beside_a_local_cache_dir` | Per-query fallback (MRE) | entry in the root's, empty member `.sqlx/` | fresh | ✅ |
//! | `explicit_offline_dir_is_searched` | Absolute `SQLX_OFFLINE_DIR` | entry there, empty `.sqlx/` | fresh | ✅ |
//! | `relative_offline_dir_mre_resolves_from_workspace_root` | Relative `SQLX_OFFLINE_DIR` in a member (MRE) | `cache` with the entry at the root | fresh | ✅ |
//! | `nested_workspace_mre_resolves_the_nearest_root` | Workspace inside another workspace's directory (MRE) | outer `.sqlx/` only | outer cache never used | ✅ |
//! | `unreadable_manifest_still_uses_the_crate_cache_dir` | Cargo cannot tell the workspace root | invalid `Cargo.toml`, entry in `.sqlx/` | fresh | ✅ |

#![ cfg( feature = "enabled" ) ]

mod support;

use support::{ directives_for, manifest, rustc_env, site_warnings, warnings, PackageDir };

/// The warning printed when no cache directory exists anywhere.
const NO_CACHE_DIR_WARNING : &str = "cargo:warning=sqlx_query: no .sqlx directory found, so SQLX_OFFLINE is not forced; run `cargo sqlx prepare` once to create one";

/// A workspace whose only member, `member/`, has one query at
/// `member/src/lib.rs:1`; no cache directory yet.
fn workspace_with_member() -> PackageDir
{
  let tree = PackageDir::empty();
  tree.write( "Cargo.toml", "[workspace]\nmembers = [ \"member\" ]\nresolver = \"2\"\n" );
  tree.write( "member/Cargo.toml", &manifest( "member", "" ) );
  tree.write( "member/src/lib.rs", "fn f() { let _ = sqlx::query!( \"SELECT 1\" ); }\n" );
  tree
}

/// A crate that never ran `cargo sqlx prepare` keeps building as before:
/// forcing offline mode would fail every query.
#[ test ]
fn missing_cache_dir_never_forces_offline()
{
  let package = PackageDir::new( "fn f() { let _ = sqlx::query!( \"SELECT 1\" ); }\n" );

  let directives = package.directives( &[] );
  assert_eq!( rustc_env( &directives ), Vec::< &str >::new(), "{directives:#?}" );
}

/// A crate without a cache directory is told why nothing is enforced, and how
/// to start.
#[ test ]
fn missing_cache_dir_is_reported()
{
  let package = PackageDir::new( "fn f() { let _ = sqlx::query!( \"SELECT 1\" ); }\n" );

  let directives = package.directives( &[] );
  assert_eq!( warnings( &directives ), [ NO_CACHE_DIR_WARNING ], "{directives:#?}" );
}

/// sqlx searches the crate's own `.sqlx/` before the workspace's, so a live
/// build writes there.
#[ test ]
fn local_cache_dir_is_preferred_over_the_workspace_one()
{
  let tree = workspace_with_member();
  tree.mkdir( ".sqlx" );
  let local = tree.mkdir( "member/.sqlx" );

  let directives = directives_for( &tree.root().join( "member" ), &[ ( "SQLX_QUERY_SOFT_WARN", "1" ) ] );
  let expected = format!( "SQLX_OFFLINE_DIR={}", local.display() );
  assert_eq!( rustc_env( &directives ), [ expected.as_str() ], "{directives:#?}" );
}

/// Without a crate-local `.sqlx/`, the workspace root's is the cache.
#[ test ]
fn workspace_cache_dir_is_used_without_a_local_one()
{
  let tree = workspace_with_member();
  let shared = tree.mkdir( ".sqlx" );

  let directives = directives_for( &tree.root().join( "member" ), &[ ( "SQLX_QUERY_SOFT_WARN", "1" ) ] );
  let expected = format!( "SQLX_OFFLINE_DIR={}", shared.display() );
  assert_eq!( rustc_env( &directives ), [ expected.as_str() ], "{directives:#?}" );
}

/// sqlx resolves each query file independently: an entry in the workspace's
/// `.sqlx/` serves a crate that also has a `.sqlx/` of its own.
///
/// ## Root Cause
///
/// `resolve_cache_dir` in `src/cache_dir.rs` returned the first existing
/// cache directory, and `report::diff` checked every site against that one
/// directory only -- so with a crate-local `.sqlx/` present, entries living in
/// the workspace root's `.sqlx/` were reported stale, although sqlx finds
/// them and the build compiles.
///
/// ## Why Not Caught
///
/// Tests created one cache directory at a time; a crate with two never
/// appeared.
///
/// ## Fix Applied
///
/// `cache_dir::resolve` returns sqlx's whole lookup list (`CacheDirs::lookup`),
/// and `report::stale_sites` counts a site fresh when any of them holds its
/// entry.
///
/// ## Prevention
///
/// Model a resolution the way the consumer of the data performs it -- here
/// per query, not per crate.
///
/// ## Pitfall
///
/// Where a live build *writes* is a separate question with a single answer
/// (`CacheDirs::primary`); only lookup is per query.
// test_kind: mre
#[ test ]
fn workspace_entry_mre_counts_as_fresh_beside_a_local_cache_dir()
{
  let tree = workspace_with_member();
  tree.cache_entry( ".sqlx", "SELECT 1" );
  tree.mkdir( "member/.sqlx" );

  let directives = directives_for( &tree.root().join( "member" ), &[] );
  assert_eq!( site_warnings( &directives ), Vec::< String >::new(), "{directives:#?}" );
}

/// An absolute `SQLX_OFFLINE_DIR` is searched (first) for every query.
#[ test ]
fn explicit_offline_dir_is_searched()
{
  let package = PackageDir::new( "fn f() { let _ = sqlx::query!( \"SELECT 1\" ); }\n" );
  package.mkdir( ".sqlx" );
  let cache = package.mkdir( "cache" );
  package.cache_entry( "cache", "SELECT 1" );
  let cache = cache.to_str().expect( "a UTF-8 temporary path" );

  let directives = package.directives( &[ ( "SQLX_OFFLINE_DIR", cache ) ] );
  assert_eq!( site_warnings( &directives ), Vec::< String >::new(), "{directives:#?}" );
}

/// A relative `SQLX_OFFLINE_DIR` names a directory under the workspace root,
/// where Cargo runs rustc -- not under the member crate.
///
/// ## Root Cause
///
/// `src/lib.rs` took `SQLX_OFFLINE_DIR` as given, so a relative value resolved
/// against the build script's working directory, the crate's own root. sqlx
/// runs inside rustc, which Cargo starts from the workspace root: for a member
/// crate the two disagreed, and every entry sqlx found was reported stale.
///
/// ## Why Not Caught
///
/// Tests set `SQLX_OFFLINE_DIR` only to absolute paths, and only in
/// single-package workspaces, where both directories coincide.
///
/// ## Fix Applied
///
/// `cache_dir::resolve` joins a relative `SQLX_OFFLINE_DIR` to the workspace
/// root (falling back to the crate root when Cargo cannot tell).
///
/// ## Prevention
///
/// Resolve every relative path against the working directory of the process
/// that will use it -- here rustc's, not the build script's.
///
/// ## Pitfall
///
/// Manual scenario S8 confirmed rustc's working directory empirically; it
/// changes only if Cargo changes how it invokes rustc.
// test_kind: mre
#[ test ]
fn relative_offline_dir_mre_resolves_from_workspace_root()
{
  let tree = workspace_with_member();
  tree.cache_entry( "cache", "SELECT 1" );

  let directives = directives_for( &tree.root().join( "member" ), &[ ( "SQLX_OFFLINE_DIR", "cache" ) ] );
  assert_eq!( site_warnings( &directives ), Vec::< String >::new(), "{directives:#?}" );
}

/// A workspace nested inside another workspace's directory is its own root;
/// the outer workspace's `.sqlx/` belongs to another project.
///
/// ## Root Cause
///
/// `src/cache_dir.rs` found the workspace root by walking up from the crate
/// and taking the outermost ancestor whose `Cargo.toml` mentions
/// `[workspace]`. A crate whose own workspace sits inside another -- an
/// example project, a vendored repository -- resolved to the enclosing one, so
/// live builds wrote into another project's `.sqlx/`, and sites were checked
/// against the wrong entries.
///
/// ## Why Not Caught
///
/// Tests never placed one workspace inside another.
///
/// ## Fix Applied
///
/// `workspace_root` in `src/cache_dir.rs` runs `cargo locate-project
/// --workspace`, the answer sqlx's own `cargo metadata` call gives too.
///
/// ## Prevention
///
/// Ask Cargo for anything Cargo decides; manifest-text heuristics miss
/// `package.workspace`, `exclude` and nested roots.
///
/// ## Pitfall
///
/// `__CARGO_FIX_PLZ` is removed before calling Cargo, as sqlx does, so
/// `cargo fix` runs resolve the same root.
// test_kind: mre
#[ test ]
fn nested_workspace_mre_resolves_the_nearest_root()
{
  let tree = PackageDir::empty();
  tree.write( "Cargo.toml", "[workspace]\nresolver = \"2\"\n" );
  tree.cache_entry( ".sqlx", "SELECT 1" );
  tree.write( "inner/Cargo.toml", &manifest( "inner", "[workspace]\n" ) );
  tree.write( "inner/src/lib.rs", "fn f() { let _ = sqlx::query!( \"SELECT 1\" ); }\n" );

  let directives = directives_for( &tree.root().join( "inner" ), &[] );
  assert_eq!( warnings( &directives ), [ NO_CACHE_DIR_WARNING ], "{directives:#?}" );
}

/// When Cargo cannot report the workspace root, the crate's own `.sqlx/` is
/// still used.
#[ test ]
fn unreadable_manifest_still_uses_the_crate_cache_dir()
{
  let package = PackageDir::new( "fn f() { let _ = sqlx::query!( \"SELECT 1\" ); }\n" );
  package.write( "Cargo.toml", "[package\n" );
  package.cache_entry( ".sqlx", "SELECT 1" );

  let directives = package.directives( &[] );
  assert_eq!( warnings( &directives ), Vec::< &str >::new(), "{directives:#?}" );
}
