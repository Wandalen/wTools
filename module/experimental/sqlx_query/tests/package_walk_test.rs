//! Which of a package's files `sqlx_query` scans for query sites, and which
//! paths it asks Cargo to watch (`cargo:rerun-if-changed=`).
//!
//! The walk mirrors Cargo's own package listing: what Cargo would not package
//! -- target directories, nested packages, hidden and git-ignored entries -- is
//! neither scanned nor watched. A directory holding such an entry is watched
//! entry by entry instead of as a whole, because Cargo watches a directory
//! recursively and an excluded entry may change during every build.
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Input | Expected | Status |
//! |-----------|----------|-------|----------|--------|
//! | `nested_package_mre_is_not_scanned` | Subdirectory with its own `Cargo.toml` (MRE) | `tools/helper/` package with a query | no site | ✅ |
//! | `nested_package_is_not_watched` | Same layout | `tools/helper/` | nothing under `tools/` watched | ✅ |
//! | `target_dir_with_cachedir_tag_mre_is_skipped` | `CARGO_TARGET_DIR` inside the package, not named `target` (MRE) | `out/CACHEDIR.TAG` + generated `.rs` with a query | no site, nothing under `out/` watched | ✅ |
//! | `root_target_dir_is_skipped_without_cachedir_tag` | Default `target/`, before Cargo tags it | `target/gen.rs` with a query | no site, `target/` not watched | ✅ |
//! | `nested_dir_named_target_is_scanned` | `target` below the root is ordinary source | `src/target/mod.rs` with a query | site found | ✅ |
//! | `directory_holding_an_excluded_entry_is_watched_entry_by_entry` | Excluded entry one level down | `tools/out/CACHEDIR.TAG` + `tools/notes.md` | `tools/notes.md` watched, `tools/` not | ✅ |
//! | `hidden_entries_are_skipped` | Dot-prefixed file and directory | `.scratch/q.rs`, `src/.q.rs` with queries | no site | ✅ |
//! | `git_ignored_entries_are_skipped` | Logs and generated code git ignores | `-0001_build.log`, `generated/q.rs` | no site, neither watched | ✅ |
//! | `query_file_outside_the_package_is_watched` | `query_file!( "../shared/q.sql" )` | SQL file beside the package | the SQL file watched | ✅ |
//! | Package sources and cache directories watched | — | — | `build_script_directives_test.rs`, `rerun_triggers_mre_cover_every_input` | N/A here |
//! | Cache directories inside the package never walked | — | — | `build_script_directives_test.rs`, `standing_aside_never_watches_a_cache_dir` | N/A here |

#![ cfg( feature = "enabled" ) ]

mod support;

use std::process::Command;

use support::{ directives_for, manifest, site_warning, site_warnings, watched_paths, PackageDir };

/// A query in a nested package is that package's concern: Cargo builds and
/// caches it on its own.
///
/// ## Root Cause
///
/// The walk in `src/scan.rs` (`walkdir` over the whole crate directory)
/// descended into every subdirectory, so a nested package's queries -- e.g. a
/// helper crate under `tools/`, or an example workspace -- were reported as
/// this crate's stale entries, and fail-fast mode named sites this build never
/// compiles.
///
/// ## Why Not Caught
///
/// Test packages never contained a second `Cargo.toml`.
///
/// ## Fix Applied
///
/// `walk_dir` in `src/scan.rs` excludes any subdirectory holding a
/// `Cargo.toml`, as Cargo's own package listing does.
///
/// ## Prevention
///
/// Mirror Cargo's package listing rules whenever deciding what belongs to a
/// package.
///
/// ## Pitfall
///
/// A nested package also must not be watched through its parent directory --
/// see `nested_package_is_not_watched`.
// test_kind: mre
#[ test ]
fn nested_package_mre_is_not_scanned()
{
  let package = PackageDir::new( "" );
  package.write( "tools/helper/Cargo.toml", &manifest( "helper", "" ) );
  package.write( "tools/helper/src/lib.rs", "fn f() { let _ = sqlx::query!( \"SELECT 1\" ); }\n" );
  package.mkdir( ".sqlx" );

  let directives = package.directives( &[] );
  assert_eq!( site_warnings( &directives ), Vec::< String >::new(), "{directives:#?}" );
}

/// A nested package's files -- and its own `target/`, once built on its own --
/// change without affecting this package, so nothing in it is watched.
#[ test ]
fn nested_package_is_not_watched()
{
  let package = PackageDir::new( "" );
  package.write( "tools/helper/Cargo.toml", &manifest( "helper", "" ) );
  package.write( "tools/helper/src/lib.rs", "" );

  let watched = watched_paths( &package.directives( &[] ) );
  let tools = package.root().join( "tools" );
  assert!( !watched.iter().any( | path | path.starts_with( &tools ) ), "nothing under tools/ may be watched: {watched:#?}" );
}

/// A Cargo target directory is recognized by the `CACHEDIR.TAG` Cargo writes
/// into it, whatever its name.
///
/// ## Root Cause
///
/// `src/scan.rs` skipped only a directory literally named `target`. With
/// `CARGO_TARGET_DIR` (or `--target-dir`) pointing elsewhere inside the
/// package, the walk scanned build output -- generated `.rs` files with queries
/// -- and, once paths were watched, would have re-run the build script after
/// every build, since a target directory changes during each one.
///
/// ## Why Not Caught
///
/// Every test build used the default target directory location.
///
/// ## Fix Applied
///
/// `is_target_dir` in `src/scan.rs` also matches any directory holding
/// `CACHEDIR.TAG`, and `walk_dir` excludes it from both scanning and watching.
///
/// ## Prevention
///
/// Never identify Cargo-owned directories by name alone.
///
/// ## Pitfall
///
/// Excluding the directory is not enough: its parent must not be watched as a
/// whole either -- see `directory_holding_an_excluded_entry_is_watched_entry_by_entry`.
// test_kind: mre
#[ test ]
fn target_dir_with_cachedir_tag_mre_is_skipped()
{
  let package = PackageDir::new( "" );
  package.write( "out/CACHEDIR.TAG", "Signature: 8a477f597d28d172789f06886806bc55\n" );
  package.write( "out/debug/build/gen/out/queries.rs", "fn g() { let _ = sqlx::query!( \"SELECT 1\" ); }\n" );
  package.mkdir( ".sqlx" );

  let directives = package.directives( &[] );
  assert_eq!( site_warnings( &directives ), Vec::< String >::new(), "{directives:#?}" );
  let out = package.root().join( "out" );
  assert!( !watched_paths( &directives ).iter().any( | path | path.starts_with( &out ) ), "{directives:#?}" );
}

/// The package root's `target/` is Cargo's default target directory even
/// before Cargo has written its `CACHEDIR.TAG`.
#[ test ]
fn root_target_dir_is_skipped_without_cachedir_tag()
{
  let package = PackageDir::new( "" );
  package.write( "target/gen.rs", "fn g() { let _ = sqlx::query!( \"SELECT 1\" ); }\n" );
  package.mkdir( ".sqlx" );

  let directives = package.directives( &[] );
  assert_eq!( site_warnings( &directives ), Vec::< String >::new(), "{directives:#?}" );
  let target = package.root().join( "target" );
  assert!( !watched_paths( &directives ).iter().any( | path | path.starts_with( &target ) ), "{directives:#?}" );
}

/// Below the root, a directory named `target` without a `CACHEDIR.TAG` is
/// ordinary source -- a module, say -- and is scanned.
#[ test ]
fn nested_dir_named_target_is_scanned()
{
  let package = PackageDir::new( "mod target;\n" );
  package.write( "src/target/mod.rs", "fn g() { let _ = sqlx::query!( \"SELECT 1\" ); }\n" );
  package.mkdir( ".sqlx" );

  let directives = package.directives( &[] );
  assert_eq!( site_warnings( &directives ), [ site_warning( "src/target/mod.rs", 1 ) ], "{directives:#?}" );
}

/// A directory with an excluded entry below it is replaced by its other
/// entries in the watch list: watching it whole would include the excluded
/// entry, since Cargo watches a directory recursively.
#[ test ]
fn directory_holding_an_excluded_entry_is_watched_entry_by_entry()
{
  let package = PackageDir::new( "" );
  package.write( "tools/out/CACHEDIR.TAG", "Signature: 8a477f597d28d172789f06886806bc55\n" );
  let notes = package.write( "tools/notes.md", "notes" );

  let watched = watched_paths( &package.directives( &[] ) );
  assert!( watched.contains( &notes ), "tools/notes.md should be watched: {watched:#?}" );
  assert!( !watched.contains( &package.root().join( "tools" ) ), "tools/ must not be watched whole: {watched:#?}" );
}

/// Hidden entries -- dot-prefixed files and directories -- are not package
/// source, and their queries are not sites.
#[ test ]
fn hidden_entries_are_skipped()
{
  let package = PackageDir::new( "" );
  package.write( ".scratch/q.rs", "fn g() { let _ = sqlx::query!( \"SELECT 1\" ); }\n" );
  package.write( "src/.q.rs", "fn g() { let _ = sqlx::query!( \"SELECT 2\" ); }\n" );
  package.mkdir( ".sqlx" );

  let directives = package.directives( &[] );
  assert_eq!( site_warnings( &directives ), Vec::< String >::new(), "{directives:#?}" );
}

/// What git ignores -- logs written during a build, generated code -- Cargo
/// leaves out of its package listing, so it never triggered a rerun before
/// `sqlx_query` emitted explicit rerun-if directives; it is neither scanned nor
/// watched. Uses the `git` on `PATH`, as `sqlx_query` itself does.
#[ test ]
fn git_ignored_entries_are_skipped()
{
  let package = PackageDir::new( "" );
  package.write( ".gitignore", "-*\ngenerated/\n" );
  let log = package.write( "-0001_build.log", "log" );
  package.write( "generated/q.rs", "fn g() { let _ = sqlx::query!( \"SELECT 1\" ); }\n" );
  package.mkdir( ".sqlx" );
  let status = Command::new( "git" ).args( [ "init", "--quiet" ] ).arg( package.root() ).status().expect( "run git init" );
  assert!( status.success(), "git init failed: {status}" );

  let directives = package.directives( &[] );
  assert_eq!( site_warnings( &directives ), Vec::< String >::new(), "{directives:#?}" );
  let watched = watched_paths( &directives );
  let generated = package.root().join( "generated" );
  assert!( !watched.iter().any( | path | *path == log || path.starts_with( &generated ) ), "{watched:#?}" );
}

/// A `query_file*!` SQL file outside the package is not reached by the walk,
/// so it is watched by its own path.
#[ test ]
fn query_file_outside_the_package_is_watched()
{
  let tree = PackageDir::empty();
  tree.write( "pkg/Cargo.toml", &manifest( "pkg", "[workspace]\n" ) );
  tree.write( "pkg/src/lib.rs", "fn f() { let _ = sqlx::query_file!( \"../shared/q.sql\" ); }\n" );
  tree.write( "shared/q.sql", "SELECT 1" );
  tree.mkdir( "pkg/.sqlx" );

  let manifest_dir = tree.root().join( "pkg" );
  let watched = watched_paths( &directives_for( &manifest_dir, &[] ) );
  assert!( watched.contains( &manifest_dir.join( "../shared/q.sql" ) ), "{watched:#?}" );
}
