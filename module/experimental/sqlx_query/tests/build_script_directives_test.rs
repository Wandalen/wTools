//! What `sqlx_query::build_script_directives` tells Cargo: whether to force
//! `SQLX_OFFLINE=true`, what to report, and which inputs re-run the build script.
//!
//! Every test passes the build script's entire environment explicitly (see
//! `support::options_for`); nothing is read from this process's environment.
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Input | Expected | Status |
//! |-----------|----------|-------|----------|--------|
//! | `fresh_cache_forces_offline_silently` | Every site has its entry | entry present | no warning; `SQLX_OFFLINE=true` | ✅ |
//! | `stale_site_still_forces_offline` | Fail-fast default | entry missing | `SQLX_OFFLINE=true` | ✅ |
//! | `orphaned_cache_entries_are_ignored` | Entries no site uses (a shared `.sqlx`) | two extra entries | no warning | ✅ |
//! | `forced_offline_summary_mre_names_the_way_out` | Why the build is about to fail (MRE) | two stale sites | site warnings, then two summary lines | ✅ |
//! | `explicit_offline_false_mre_is_never_overridden` | `cargo sqlx prepare`'s environment (MRE) | `SQLX_OFFLINE=false`, stale | no `rustc-env` at all | ✅ |
//! | `standing_aside_reports_nothing` | Same | `SQLX_OFFLINE=false`, stale | no warning | ✅ |
//! | `standing_aside_never_watches_a_cache_dir` | sqlx rewrites cache files on every live compile | `SQLX_OFFLINE=false`, `.sqlx/` + `SQLX_OFFLINE_DIR=data/sqlx` | neither cache dir nor `data/` watched; `data/notes.txt` is | ✅ |
//! | `explicit_offline_true_is_never_reforced` | User already forces offline | `SQLX_OFFLINE=true`, stale | no `rustc-env` | ✅ |
//! | `explicit_offline_true_still_reports_stale_sites` | Same | `SQLX_OFFLINE=true`, stale | site warning, then a summary | ✅ |
//! | `soft_warn_mre_needs_no_database_url_in_process_env` | `DATABASE_URL` only in `.env` (MRE) | `SQLX_QUERY_SOFT_WARN=1`, stale | `SQLX_OFFLINE` not forced; `SQLX_OFFLINE_DIR` exported | ✅ |
//! | `soft_warn_summary_names_where_entries_are_written` | Opt-out explanation | `SQLX_QUERY_SOFT_WARN=1`, one stale site | summary names `.sqlx/` and "its missing entry" | ✅ |
//! | `soft_warn_summary_counts_several_missing_entries` | Same, plural | `SQLX_QUERY_SOFT_WARN=1`, two stale sites | "the 2 queries above ... their missing entries ... (commit them)" | ✅ |
//! | `test_only_query_mre_summaries_hold_for_builds_that_skip_it` | A query `cargo build` does not compile (MRE) | stale query in `tests/it.rs`; default, `SQLX_OFFLINE=true`, `SQLX_QUERY_SOFT_WARN=1` | every summary speaks of wherever the query is compiled | ✅ |
//! | `soft_warn_keeps_an_explicit_offline_dir` | User-chosen cache dir | `SQLX_QUERY_SOFT_WARN=1`, `SQLX_OFFLINE_DIR` set | no `rustc-env` | ✅ |
//! | `soft_warn_without_stale_sites_still_forces_offline` | Opt-out only matters when stale | `SQLX_QUERY_SOFT_WARN=1`, fresh | `SQLX_OFFLINE=true` | ✅ |
//! | `dotenv_offline_false_stands_aside` | `SQLX_OFFLINE=false` kept in `.env` | `.env`, stale | no `rustc-env`, no warning; `.env` watched | ✅ |
//! | `dotenv_offline_dir_is_searched` | `SQLX_OFFLINE_DIR` kept in `.env` | entry only in that directory | no warning; `SQLX_OFFLINE=true` | ✅ |
//! | `soft_warn_keeps_a_dotenv_offline_dir` | Same, with the opt-out | `SQLX_QUERY_SOFT_WARN=1`, stale | no `rustc-env`; summary names that directory | ✅ |
//! | `missing_offline_dir_mre_says_sqlx_cannot_write` | `SQLX_OFFLINE_DIR` names no directory (MRE) | `SQLX_QUERY_SOFT_WARN=1`, stale | no `rustc-env`; summary says sqlx fails until it is created | ✅ |
//! | `forced_offline_advice_names_a_missing_offline_dir` | Same, fail-fast | stale | `SQLX_OFFLINE=true`; advice names the directory to create | ✅ |
//! | `missing_offline_dir_is_watched_while_named` | Creating it must re-run the check | stale; then `SQLX_OFFLINE=true`; then fresh | declared only while a warning names it | ✅ |
//! | `rerun_triggers_mre_cover_every_input` | Cargo must re-run on any input change (MRE) | package with `.sqlx/` | five env vars, `Cargo.toml`, `src/`, `.sqlx/` | ✅ |
//! | `sqlx_toml_variable_is_declared_and_named` | `sqlx.toml` renames the database variable | `database-url-var = "APP_DATABASE_URL"`, stale | that variable declared and named; `sqlx.toml` watched | ✅ |
//! | `auto_regen_without_database_url_mre_reports_the_skip` | Nothing to regenerate against (`auto_regen` feature, MRE) | `SQLX_QUERY_AUTO_REGEN=1`, stale | no regeneration; one notice naming `DATABASE_URL` | ✅ |
//! | `auto_regen_with_explicit_offline_true_is_skipped` | User forbids live builds (`auto_regen` feature) | `SQLX_OFFLINE=true`, `DATABASE_URL`, `SQLX_QUERY_AUTO_REGEN=1` | no regeneration | ✅ |
//! | `auto_regen_finds_database_url_in_dotenv` | `DATABASE_URL` kept in `.env` (`auto_regen` feature) | `.env`, `SQLX_QUERY_AUTO_REGEN=1`, stale | an attempt, not the skip notice | ✅ |
//! | `auto_regen_mre_keeps_the_nested_check_output_captured` | The attempt's output in a passing `cargo test` (`auto_regen` feature, MRE) | that test, re-run under the harness's capture | none of the nested check's output printed | ✅ |
//! | `auto_regen_follows_the_sqlx_toml_variable` | `DATABASE_URL` is not sqlx's variable (`auto_regen` feature) | `sqlx.toml` variable unset, `DATABASE_URL` set | skip notice naming the configured variable | ✅ |
//! | `auto_regen_skips_a_missing_offline_dir` | Nowhere to regenerate into (`auto_regen` feature) | `SQLX_OFFLINE_DIR` names no directory, `DATABASE_URL` set | skip notice naming that directory | ✅ |
//! | No cache directory anywhere | — | — | `cache_dir_test.rs` | N/A here |
//! | Live regeneration itself | — | — | `offline_regen_deadlock_guard_test.rs` | N/A here |

#![ cfg( feature = "enabled" ) ]

mod support;

use std::path::PathBuf;

use support::{ rustc_env, site_warning, warnings, watched_paths, PackageDir };

/// A package with one query at `src/lib.rs:1` and an empty `.sqlx/`.
fn stale_package() -> PackageDir
{
  let package = PackageDir::new( "fn f() { let _ = sqlx::query!( \"SELECT 1\" ); }\n" );
  package.mkdir( ".sqlx" );
  package
}

/// With every entry present, the build is forced offline -- exactly what CI
/// does -- and nothing is reported.
#[ test ]
fn fresh_cache_forces_offline_silently()
{
  let package = stale_package();
  package.cache_entry( ".sqlx", "SELECT 1" );

  let directives = package.directives( &[] );
  assert_eq!( warnings( &directives ), Vec::< &str >::new(), "{directives:#?}" );
  assert_eq!( rustc_env( &directives ), [ "SQLX_OFFLINE=true" ], "{directives:#?}" );
}

/// The fail-fast default: a stale entry still forces offline mode, so this
/// build fails on it exactly as a build without a database would.
#[ test ]
fn stale_site_still_forces_offline()
{
  let directives = stale_package().directives( &[] );
  assert_eq!( rustc_env( &directives ), [ "SQLX_OFFLINE=true" ], "{directives:#?}" );
}

/// Entries no call site uses are never reported: a workspace-shared `.sqlx/`
/// legitimately holds sibling crates' entries.
#[ test ]
fn orphaned_cache_entries_are_ignored()
{
  let package = stale_package();
  for sql in [ "SELECT 1", "SELECT 2", "SELECT 3" ]
  {
    package.cache_entry( ".sqlx", sql );
  }

  let directives = package.directives( &[] );
  assert_eq!( warnings( &directives ), Vec::< &str >::new(), "{directives:#?}" );
}

/// The failing build itself explains who forced `SQLX_OFFLINE` and both ways
/// out -- sqlx's own error only says to unset it.
///
/// ## Root Cause
///
/// `ensure_offline_mode_with` in `src/lib.rs` printed only one warning per
/// stale site before forcing `SQLX_OFFLINE=true`. sqlx then failed with "run
/// `cargo sqlx prepare` ... or unset `SQLX_OFFLINE`" -- a variable the user
/// never set and cannot unset -- and the opt-out, `SQLX_QUERY_SOFT_WARN=1`, was
/// mentioned nowhere in the build output.
///
/// ## Why Not Caught
///
/// Tests asserted that the build failed, never that its output let a user
/// recover; the manual scenario reading the output end to end (S1) found it.
///
/// ## Fix Applied
///
/// `report::forced_offline_summary` adds two lines after the site warnings:
/// who forces offline mode and why, then `cargo sqlx prepare` (with the
/// `--all-targets --all-features` hint) and `SQLX_QUERY_SOFT_WARN=1`.
///
/// ## Prevention
///
/// Every build failure this crate causes must name its own way out in the
/// same output.
///
/// ## Pitfall
///
/// Plain `cargo sqlx prepare` checks only the default target: queries in
/// tests or behind features need `-- --all-targets --all-features`.
// test_kind: mre
#[ test ]
fn forced_offline_summary_mre_names_the_way_out()
{
  let package = PackageDir::new( "fn f()\n{\n  let _ = sqlx::query!( \"SELECT 1\" );\n  let _ = sqlx::query!( \"SELECT 2\" );\n}\n" );
  package.mkdir( ".sqlx" );

  let directives = package.directives( &[] );
  let expected =
  [
    "cargo:warning=sqlx_query: src/lib.rs:3 -- no .sqlx entry matches this query's SQL",
    "cargo:warning=sqlx_query: src/lib.rs:4 -- no .sqlx entry matches this query's SQL",
    "cargo:warning=sqlx_query: forcing SQLX_OFFLINE=true, so sqlx rejects the 2 queries above wherever they are compiled, exactly as an offline build (CI, a teammate without a database) would",
    "cargo:warning=sqlx_query: fix with `cargo sqlx prepare` (add `-- --all-targets --all-features` when queries live in tests or behind features), or rebuild with SQLX_QUERY_SOFT_WARN=1 to compile against DATABASE_URL and refresh .sqlx in place",
  ];
  assert_eq!( warnings( &directives ), expected, "{directives:#?}" );
}

/// `SQLX_OFFLINE=false` -- what `cargo sqlx prepare` sets -- is never
/// overridden: sqlx must compile live to rewrite the cache.
///
/// ## Root Cause
///
/// `ensure_offline_mode_with` in `src/lib.rs` printed
/// `cargo:rustc-env=SQLX_OFFLINE=true` whatever the environment held, and
/// Cargo's `rustc-env` beats an inherited variable. `cargo sqlx prepare` first
/// deletes every `.sqlx/query-*.json`, then runs `cargo check` with
/// `SQLX_OFFLINE=false`; the forced `true` made every query fail offline, so
/// prepare aborted and left `.sqlx/` empty, and `prepare --check` compiled
/// against the old entries, checking nothing.
///
/// ## Why Not Caught
///
/// No test ran with sqlx-cli's environment, and the design never considered
/// an explicit `SQLX_OFFLINE` at all; manual scenarios S2a, S2b and S3c did.
///
/// ## Fix Applied
///
/// `Options::offline` holds an explicit `SQLX_OFFLINE`, parsed as sqlx parses
/// it; `build_script_directives` returns right after the rerun directives when
/// it is `Some( false )`.
///
/// ## Prevention
///
/// Never override a variable the user or a tool set explicitly; force only
/// what is unset.
///
/// ## Pitfall
///
/// sqlx counts any value but `true` (any case) or `1` as `false` -- including
/// the empty string -- so `SQLX_OFFLINE=` stands aside too.
// test_kind: mre
#[ test ]
fn explicit_offline_false_mre_is_never_overridden()
{
  let directives = stale_package().directives( &[ ( "SQLX_OFFLINE", "false" ) ] );
  assert_eq!( rustc_env( &directives ), Vec::< &str >::new(), "{directives:#?}" );
}

/// Standing aside is silent: during `cargo sqlx prepare`, stale entries are
/// exactly what is being regenerated.
#[ test ]
fn standing_aside_reports_nothing()
{
  let directives = stale_package().directives( &[ ( "SQLX_OFFLINE", "false" ) ] );
  assert_eq!( warnings( &directives ), Vec::< &str >::new(), "{directives:#?}" );
}

/// sqlx rewrites a cache file every time it compiles that query live; a
/// watched cache directory would then re-run the build script, and recompile
/// the crate, on every live build. With `SQLX_OFFLINE=false` no cache directory
/// is watched -- not the default `.sqlx/`, and not a `SQLX_OFFLINE_DIR` nested
/// inside the package, whose parent is watched entry by entry instead.
#[ test ]
fn standing_aside_never_watches_a_cache_dir()
{
  let package = stale_package();
  package.mkdir( "data/sqlx" );
  let notes = package.write( "data/notes.txt", "notes" );

  let directives = package.directives( &[ ( "SQLX_OFFLINE", "false" ), ( "SQLX_OFFLINE_DIR", "data/sqlx" ) ] );
  let watched = watched_paths( &directives );
  let unwatched = [ package.root().join( ".sqlx" ), package.root().join( "data" ), package.root().join( "data/sqlx" ) ];
  assert!( !watched.iter().any( | path | unwatched.contains( path ) ), "{watched:#?}" );
  assert!( watched.contains( &notes ), "data/notes.txt should still be watched: {watched:#?}" );
}

/// An explicit `SQLX_OFFLINE=true` already makes sqlx offline; repeating it as
/// a `rustc-env` would only hide that the user chose it.
#[ test ]
fn explicit_offline_true_is_never_reforced()
{
  let directives = stale_package().directives( &[ ( "SQLX_OFFLINE", "true" ) ] );
  assert_eq!( rustc_env( &directives ), Vec::< &str >::new(), "{directives:#?}" );
}

/// With `SQLX_OFFLINE=true` already set, stale sites are still reported --
/// this build is about to fail on them -- with a summary that does not claim
/// `sqlx_query` forced anything.
#[ test ]
fn explicit_offline_true_still_reports_stale_sites()
{
  let directives = stale_package().directives( &[ ( "SQLX_OFFLINE", "true" ) ] );
  let expected =
  [
    site_warning( "src/lib.rs", 1 ),
    "cargo:warning=sqlx_query: SQLX_OFFLINE is already true in the environment or a .env file, so sqlx rejects the query above wherever it is compiled; run `cargo sqlx prepare`".to_string(),
  ];
  assert_eq!( warnings( &directives ), expected, "{directives:#?}" );
}

/// `SQLX_QUERY_SOFT_WARN=1` alone opts out of forcing offline mode -- sqlx
/// reads `DATABASE_URL` from `.env` by itself, where this process never sees it.
///
/// ## Root Cause
///
/// `ensure_offline_mode_with` in `src/lib.rs` honored the opt-out only when
/// `Options::live_db_reachable` -- a non-empty `DATABASE_URL` in the build
/// script's own environment -- was true. sqlx's standard setup keeps
/// `DATABASE_URL` in `.env`, which sqlx's macros load themselves, so for most
/// users `SQLX_QUERY_SOFT_WARN=1` still forced `SQLX_OFFLINE=true` and the
/// build failed exactly as before.
///
/// ## Why Not Caught
///
/// Every test passed `DATABASE_URL` in the process environment; the `.env`
/// setup (manual scenario S3a) was never exercised.
///
/// ## Fix Applied
///
/// `build_script_directives` checks only `opts.soft_warn` before leaving
/// `SQLX_OFFLINE` unforced; `live_db_reachable` was removed.
///
/// ## Prevention
///
/// Gate on what sqlx itself will see, not on this process's partial view of
/// it -- or not at all, when the safe default makes the gate unnecessary.
///
/// ## Pitfall
///
/// Without any database, sqlx falls back to offline mode by itself, so not
/// forcing is safe even when `DATABASE_URL` is truly unset.
// test_kind: mre
#[ test ]
fn soft_warn_mre_needs_no_database_url_in_process_env()
{
  let package = stale_package();

  let directives = package.directives( &[ ( "SQLX_QUERY_SOFT_WARN", "1" ) ] );
  let expected = format!( "SQLX_OFFLINE_DIR={}", package.root().join( ".sqlx" ).display() );
  assert_eq!( rustc_env( &directives ), [ expected.as_str() ], "{directives:#?}" );
}

/// The opt-out explains itself, naming where a live build writes the missing
/// entry.
#[ test ]
fn soft_warn_summary_names_where_entries_are_written()
{
  let package = stale_package();

  let directives = package.directives( &[ ( "SQLX_QUERY_SOFT_WARN", "1" ) ] );
  let expected = format!(
    "cargo:warning=sqlx_query: SQLX_QUERY_SOFT_WARN=1, so SQLX_OFFLINE is not forced: wherever the query above is compiled against DATABASE_URL, sqlx writes its missing entry into {} (commit it); without DATABASE_URL, run `cargo sqlx prepare`",
    package.root().join( ".sqlx" ).display(),
  );
  assert_eq!( warnings( &directives ).last(), Some( &expected.as_str() ), "{directives:#?}" );
}

/// With several stale sites the opt-out's summary counts the entries a live
/// build writes.
#[ test ]
fn soft_warn_summary_counts_several_missing_entries()
{
  let package = PackageDir::new( "fn f()\n{\n  let _ = sqlx::query!( \"SELECT 1\" );\n  let _ = sqlx::query!( \"SELECT 2\" );\n}\n" );
  package.mkdir( ".sqlx" );

  let directives = package.directives( &[ ( "SQLX_QUERY_SOFT_WARN", "1" ) ] );
  let expected = format!(
    "cargo:warning=sqlx_query: SQLX_QUERY_SOFT_WARN=1, so SQLX_OFFLINE is not forced: wherever the 2 queries above are compiled against DATABASE_URL, sqlx writes their missing entries into {} (commit them); without DATABASE_URL, run `cargo sqlx prepare`",
    package.root().join( ".sqlx" ).display(),
  );
  assert_eq!( warnings( &directives ).last(), Some( &expected.as_str() ), "{directives:#?}" );
}

/// The summaries hold for every build that replays them, including one that
/// never compiles the stale query -- `cargo build`, with the query in `tests/`.
///
/// ## Root Cause
///
/// The summaries in `src/report.rs` described what "this build" does with each
/// stale site: "sqlx rejects the query above", "this build writes the missing
/// entry". But the scan covers every `.rs` file in the package -- tests,
/// examples, benches and `#[cfg]`-gated code a given build may skip -- and
/// Cargo runs the build script per package, not per target. With a stale query
/// in `tests/`, `cargo build` claimed a rejection and then finished, and
/// `SQLX_QUERY_SOFT_WARN=1 cargo build` promised an entry it never wrote.
///
/// ## Why Not Caught
///
/// Every test and manual scenario put its queries in `src/lib.rs`, which every
/// build compiles; manual scenario S12 put one in `tests/`.
///
/// ## Fix Applied
///
/// The summaries speak of wherever the query is compiled: sqlx rejects it
/// "wherever it is compiled", and soft-warn writes its entry "wherever the
/// query above is compiled against `DATABASE_URL`".
///
/// ## Prevention
///
/// Build-script output is shared by every target of the package; a message
/// must hold for all of them, not only for the command that printed it.
///
/// ## Pitfall
///
/// Narrowing the scan to compiled code is not the fix: naming a test's stale
/// query before CI compiles it is the point.
// test_kind: mre
#[ test ]
fn test_only_query_mre_summaries_hold_for_builds_that_skip_it()
{
  let package = PackageDir::new( "pub fn f() {}\n" );
  package.mkdir( ".sqlx" );
  package.write( "tests/it.rs", "#[ test ]\nfn names()\n{\n  let _ = sqlx::query!( \"SELECT name FROM t\" );\n}\n" );
  let site = site_warning( "tests/it.rs", 4 );

  let directives = package.directives( &[] );
  let expected =
  [
    site.clone(),
    "cargo:warning=sqlx_query: forcing SQLX_OFFLINE=true, so sqlx rejects the query above wherever it is compiled, exactly as an offline build (CI, a teammate without a database) would".to_string(),
    "cargo:warning=sqlx_query: fix with `cargo sqlx prepare` (add `-- --all-targets --all-features` when queries live in tests or behind features), or rebuild with SQLX_QUERY_SOFT_WARN=1 to compile against DATABASE_URL and refresh .sqlx in place".to_string(),
  ];
  assert_eq!( warnings( &directives ), expected, "{directives:#?}" );

  let directives = package.directives( &[ ( "SQLX_OFFLINE", "true" ) ] );
  let expected =
  [
    site.clone(),
    "cargo:warning=sqlx_query: SQLX_OFFLINE is already true in the environment or a .env file, so sqlx rejects the query above wherever it is compiled; run `cargo sqlx prepare`".to_string(),
  ];
  assert_eq!( warnings( &directives ), expected, "{directives:#?}" );

  let directives = package.directives( &[ ( "SQLX_QUERY_SOFT_WARN", "1" ) ] );
  let expected =
  [
    site,
    format!(
      "cargo:warning=sqlx_query: SQLX_QUERY_SOFT_WARN=1, so SQLX_OFFLINE is not forced: wherever the query above is compiled against DATABASE_URL, sqlx writes its missing entry into {} (commit it); without DATABASE_URL, run `cargo sqlx prepare`",
      package.root().join( ".sqlx" ).display(),
    ),
  ];
  assert_eq!( warnings( &directives ), expected, "{directives:#?}" );
}

/// An explicit `SQLX_OFFLINE_DIR` already reaches sqlx; it is not re-exported
/// (sqlx resolves it the same way this crate does).
#[ test ]
fn soft_warn_keeps_an_explicit_offline_dir()
{
  let package = stale_package();
  let cache = package.mkdir( "cache" );
  let cache = cache.to_str().expect( "a UTF-8 temporary path" );

  let directives = package.directives( &[ ( "SQLX_QUERY_SOFT_WARN", "1" ), ( "SQLX_OFFLINE_DIR", cache ) ] );
  assert_eq!( rustc_env( &directives ), Vec::< &str >::new(), "{directives:#?}" );
}

/// The opt-out applies only to stale entries: with every entry fresh the build
/// is forced offline as usual.
#[ test ]
fn soft_warn_without_stale_sites_still_forces_offline()
{
  let package = stale_package();
  package.cache_entry( ".sqlx", "SELECT 1" );

  let directives = package.directives( &[ ( "SQLX_QUERY_SOFT_WARN", "1" ) ] );
  assert_eq!( rustc_env( &directives ), [ "SQLX_OFFLINE=true" ], "{directives:#?}" );
}

/// A `SQLX_OFFLINE=false` in `.env` is as explicit as one in the environment,
/// and sqlx reads it the same way: the check stands aside, silently, while
/// still watching the file that holds the setting.
#[ test ]
fn dotenv_offline_false_stands_aside()
{
  let package = stale_package();
  let dotenv = package.write( ".env", "SQLX_OFFLINE=false\n" );

  let directives = package.directives( &[] );
  assert_eq!( rustc_env( &directives ), Vec::< &str >::new(), "{directives:#?}" );
  assert_eq!( warnings( &directives ), Vec::< &str >::new(), "{directives:#?}" );
  assert!( watched_paths( &directives ).contains( &dotenv ), "{directives:#?}" );
}

/// A `SQLX_OFFLINE_DIR` in `.env` is searched like one in the environment --
/// relative to the workspace root -- so its entries count as present.
#[ test ]
fn dotenv_offline_dir_is_searched()
{
  let package = stale_package();
  package.cache_entry( "custom_cache", "SELECT 1" );
  package.write( ".env", "SQLX_OFFLINE_DIR=custom_cache\n" );

  let directives = package.directives( &[] );
  assert_eq!( warnings( &directives ), Vec::< &str >::new(), "{directives:#?}" );
  assert_eq!( rustc_env( &directives ), [ "SQLX_OFFLINE=true" ], "{directives:#?}" );
}

/// A `SQLX_OFFLINE_DIR` in `.env` already reaches sqlx, so the opt-out does
/// not export another over it, and its summary names that directory as the
/// one the entries are written into.
#[ test ]
fn soft_warn_keeps_a_dotenv_offline_dir()
{
  let package = stale_package();
  let custom = package.mkdir( "custom_cache" );
  package.write( ".env", "SQLX_OFFLINE_DIR=custom_cache\n" );

  let directives = package.directives( &[ ( "SQLX_QUERY_SOFT_WARN", "1" ) ] );
  assert_eq!( rustc_env( &directives ), Vec::< &str >::new(), "{directives:#?}" );
  let written_into = format!( "sqlx writes its missing entry into {} (commit it)", custom.display() );
  assert!( warnings( &directives ).last().is_some_and( | line | line.contains( &written_into ) ), "{directives:#?}" );
}

/// An explicit `SQLX_OFFLINE_DIR` that is not a directory takes no entries:
/// sqlx fails every query it compiles live instead, and the opt-out's summary
/// says so rather than promising an entry there.
///
/// ## Root Cause
///
/// `build_script_directives` in `src/lib.rs` took an explicit
/// `SQLX_OFFLINE_DIR` for the cache a live build writes into without checking
/// that it is a directory. sqlx never creates it: compiling live, every query
/// macro fails with "sqlx offline path does not exist". Yet the soft-warn
/// summary promised the missing entry would be written there, the fail-fast
/// advice sent the developer to that same soft-warn build, and `auto_regen`
/// spent a whole nested `cargo check` on it, reporting only its exit status.
///
/// ## Why Not Caught
///
/// Every test and manual scenario that set `SQLX_OFFLINE_DIR` created the
/// directory first; a manual build with one that does not exist found it, and
/// `offline_stale_warning_test.rs` now builds that case for real.
///
/// ## Fix Applied
///
/// `build_script_directives` checks the directory once. When it is not one,
/// the soft-warn summary (`report::soft_warn_without_offline_dir_summary`)
/// and the fail-fast advice name it to be created, `auto_regen` is skipped
/// with `regen::skipped_without_offline_dir`, and the directory is declared as
/// a rerun input while the warnings name it, so creating it re-runs the check.
///
/// ## Prevention
///
/// A message promising that sqlx writes somewhere must hold where sqlx
/// actually writes: check the precondition sqlx itself enforces.
///
/// ## Pitfall
///
/// Offline builds skip a missing directory silently, so while every entry is
/// present the build succeeds and nothing is reported; only a live compile
/// fails on it.
// test_kind: mre
#[ test ]
fn missing_offline_dir_mre_says_sqlx_cannot_write()
{
  let package = stale_package();
  let missing = package.root().join( "missing_cache" );

  let env = [ ( "SQLX_QUERY_SOFT_WARN", "1" ), ( "SQLX_OFFLINE_DIR", missing.to_str().expect( "a UTF-8 temporary path" ) ) ];
  let directives = package.directives( &env );
  let expected = format!(
    "cargo:warning=sqlx_query: SQLX_QUERY_SOFT_WARN=1, so SQLX_OFFLINE is not forced, but SQLX_OFFLINE_DIR names {}, which is not a directory: sqlx fails every query it compiles against DATABASE_URL until you create it",
    missing.display(),
  );
  assert_eq!( warnings( &directives ).last(), Some( &expected.as_str() ), "{directives:#?}" );
  assert_eq!( rustc_env( &directives ), Vec::< &str >::new(), "{directives:#?}" );
}

/// Forced offline, a build still searches the other cache directories -- sqlx
/// skips a missing one -- and its advice names an explicit `SQLX_OFFLINE_DIR`
/// that is not a directory as the one to create before a soft-warn rebuild.
#[ test ]
fn forced_offline_advice_names_a_missing_offline_dir()
{
  let package = stale_package();
  let missing = package.root().join( "missing_cache" );

  let directives = package.directives( &[ ( "SQLX_OFFLINE_DIR", missing.to_str().expect( "a UTF-8 temporary path" ) ) ] );
  let advice = format!(
    "cargo:warning=sqlx_query: fix with `cargo sqlx prepare` (add `-- --all-targets --all-features` when queries live in tests or behind features), or create {}, which SQLX_OFFLINE_DIR names, and rebuild with SQLX_QUERY_SOFT_WARN=1 to compile against DATABASE_URL and write into it",
    missing.display(),
  );
  assert_eq!( warnings( &directives ).last(), Some( &advice.as_str() ), "{directives:#?}" );
  assert_eq!( rustc_env( &directives ), [ "SQLX_OFFLINE=true" ], "{directives:#?}" );
}

/// The warnings say to create a missing `SQLX_OFFLINE_DIR`, so while they do
/// it is declared anyway: creating it re-runs the check, where Cargo would
/// otherwise replay "not a directory" -- and the `auto_regen` skip -- over a
/// directory that now exists. Where no warning names it -- every entry
/// present, or `SQLX_OFFLINE=true` already set -- it stays undeclared, as a
/// missing path would re-run every build.
#[ test ]
fn missing_offline_dir_is_watched_while_named()
{
  let package = stale_package();
  let missing = package.root().join( "missing_cache" );
  let offline_dir = ( "SQLX_OFFLINE_DIR", missing.to_str().expect( "a UTF-8 temporary path" ) );

  let directives = package.directives( &[ offline_dir ] );
  assert!( watched_paths( &directives ).contains( &missing ), "{directives:#?}" );

  let directives = package.directives( &[ offline_dir, ( "SQLX_OFFLINE", "true" ) ] );
  assert!( !watched_paths( &directives ).contains( &missing ), "{directives:#?}" );

  package.cache_entry( ".sqlx", "SELECT 1" );
  let directives = package.directives( &[ offline_dir ] );
  assert!( !watched_paths( &directives ).contains( &missing ), "{directives:#?}" );
  assert_eq!( warnings( &directives ), Vec::< &str >::new(), "{directives:#?}" );
}

/// Cargo re-runs the build script when any input of its decision changes: an
/// environment variable it reads, the package's sources, or a cache directory.
///
/// ## Root Cause
///
/// `ensure_offline_mode_with` in `src/lib.rs` emitted no rerun-if directives
/// on purpose, relying on Cargo's default -- re-run when a package file
/// changes. That default never covers environment variables, nor a `.sqlx/`
/// Cargo does not list (hidden outside git, or at the workspace root), so
/// Cargo replayed the previous run's output instead: a stale warning after
/// `cargo sqlx prepare` healed the cache, a failing `SQLX_QUERY_SOFT_WARN=1`
/// retry, a deleted entry going unnoticed.
///
/// ## Why Not Caught
///
/// Each end-to-end test used fresh target directories, and the bootstrap build
/// deliberately used a separate one -- so no test ever saw a replayed result.
///
/// ## Fix Applied
///
/// `build_script_directives` emits `rerun-if-env-changed` for every variable
/// it reads, `rerun-if-changed` for the package walk's watch list
/// (`src/scan.rs`), and, unless standing aside, for each existing cache dir.
///
/// ## Prevention
///
/// Every input a build script reads must be declared to Cargo; tests must
/// build repeatedly in one target directory, as developers do.
///
/// ## Pitfall
///
/// Declaring any rerun-if directive switches Cargo from its package-wide
/// default to the declared list only, so the sources have to be declared too.
// test_kind: mre
#[ test ]
fn rerun_triggers_mre_cover_every_input()
{
  let package = stale_package();

  let directives = package.directives( &[] );
  let env_vars : Vec< &str > = directives.iter().filter_map( | line | line.strip_prefix( "cargo:rerun-if-env-changed=" ) ).collect();
  assert_eq!( env_vars, [ "DATABASE_URL", "SQLX_OFFLINE", "SQLX_OFFLINE_DIR", "SQLX_QUERY_SOFT_WARN", "SQLX_QUERY_AUTO_REGEN" ] );

  let watched = watched_paths( &directives );
  let inputs : [ PathBuf; 3 ] = [ package.root().join( "Cargo.toml" ), package.root().join( "src" ), package.root().join( ".sqlx" ) ];
  for input in inputs
  {
    assert!( watched.contains( &input ), "{} should be watched: {watched:#?}", input.display() );
  }
}

/// With a `sqlx.toml` naming another database variable, that variable is the
/// one declared to Cargo and named in the advice, and `sqlx.toml` is watched.
#[ test ]
fn sqlx_toml_variable_is_declared_and_named()
{
  let package = stale_package();
  let config = package.write( "sqlx.toml", "[common]\ndatabase-url-var = \"APP_DATABASE_URL\"\n" );

  let directives = package.directives( &[] );
  let env_vars : Vec< &str > = directives.iter().filter_map( | line | line.strip_prefix( "cargo:rerun-if-env-changed=" ) ).collect();
  assert_eq!( env_vars, [ "APP_DATABASE_URL", "SQLX_OFFLINE", "SQLX_OFFLINE_DIR", "SQLX_QUERY_SOFT_WARN", "SQLX_QUERY_AUTO_REGEN" ] );
  assert!( watched_paths( &directives ).contains( &config ), "{directives:#?}" );
  let advice = "or rebuild with SQLX_QUERY_SOFT_WARN=1 to compile against APP_DATABASE_URL and refresh .sqlx in place";
  assert!( warnings( &directives ).last().is_some_and( | line | line.ends_with( advice ) ), "{directives:#?}" );
}

/// Every message `src/regen.rs` can print mentions "regen" -- both success
/// lines, the three failure lines, and the skip notice alike.
#[ cfg( feature = "auto_regen" ) ]
fn regeneration_reports( directives : &[ String ] ) -> Vec< &str >
{
  warnings( directives ).into_iter().filter( | line | line.contains( "regen" ) ).collect()
}

/// Without `DATABASE_URL` there is nothing to regenerate against: no nested
/// `cargo check` is spent on it, and the build says why its explicit opt-in
/// did nothing.
///
/// ## Root Cause
///
/// `build_script_directives` in `src/lib.rs` attempted regeneration only with
/// a non-empty `DATABASE_URL` in the build script's own environment, and
/// otherwise skipped it silently -- though
/// `docs/feature/002_auto_regeneration.md` promised a warning. With
/// `DATABASE_URL` in `.env`, which sqlx reads but this crate does not, the
/// developer saw the usual failure and no sign that `SQLX_QUERY_AUTO_REGEN=1`
/// had been ignored.
///
/// ## Why Not Caught
///
/// The test asserted only that nothing was regenerated, never that the skip
/// was reported; manual scenario S3b ran the `.env` setup end to end.
///
/// ## Fix Applied
///
/// The skip pushes `regen::skipped_without_database_url`, naming the variable
/// sqlx reads the database URL from. Reading `.env` itself is
/// `dotenv_mre_supplies_what_sqlx_reads_from_it`.
///
/// ## Prevention
///
/// An explicit opt-in that does not take effect must say why, in the same
/// build output.
///
/// ## Pitfall
///
/// The notice must not look like an attempt: on a skip it is the only
/// regeneration line.
// test_kind: mre
#[ cfg( feature = "auto_regen" ) ]
#[ test ]
fn auto_regen_without_database_url_mre_reports_the_skip()
{
  let directives = stale_package().directives( &[ ( "SQLX_QUERY_AUTO_REGEN", "1" ) ] );
  let expected = [ "cargo:warning=sqlx_query: auto_regen skipped: DATABASE_URL is unset or empty" ];
  assert_eq!( regeneration_reports( &directives ), expected, "{directives:#?}" );
}

/// An explicit `SQLX_OFFLINE=true` forbids live compilation; regeneration,
/// which compiles live, is skipped.
#[ cfg( feature = "auto_regen" ) ]
#[ test ]
fn auto_regen_with_explicit_offline_true_is_skipped()
{
  let env = [ ( "SQLX_OFFLINE", "true" ), ( "DATABASE_URL", "sqlite::memory:" ), ( "SQLX_QUERY_AUTO_REGEN", "1" ) ];
  let directives = stale_package().directives( &env );
  assert_eq!( regeneration_reports( &directives ), Vec::< &str >::new(), "{directives:#?}" );
}

/// A `DATABASE_URL` kept in `.env` -- sqlx's standard setup -- opens the gate:
/// the build attempts regeneration instead of reporting the skip. The attempt
/// itself, a real nested `cargo check` of this dependency-less package, fails
/// fast and is reported; succeeding is `offline_regen_coverage_test.rs`'s job.
#[ cfg( feature = "auto_regen" ) ]
#[ test ]
fn auto_regen_finds_database_url_in_dotenv()
{
  let package = stale_package();
  package.write( ".env", "DATABASE_URL=sqlite::memory:\n" );

  let directives = package.directives( &[ ( "SQLX_QUERY_AUTO_REGEN", "1" ) ] );
  let reports = regeneration_reports( &directives );
  assert_eq!( reports.len(), 1, "{directives:#?}" );
  assert!( !reports[ 0 ].contains( "auto_regen skipped" ), "{directives:#?}" );
}

/// A passing test prints nothing of the nested `cargo check` it runs: the
/// check's output goes through the test harness's capture, like the test's own.
///
/// ## Root Cause
///
/// The nested check inherited the process's stderr descriptor, which the
/// harness never captures -- it captures `print!` and `eprint!` only -- so
/// `auto_regen_finds_database_url_in_dotenv` printed the check's compile
/// errors (`error: could not compile`) into every `cargo test` log, CI's
/// included, while passing.
///
/// ## Why Not Caught
///
/// nextest captures whole processes, so its runs stayed clean; only plain
/// `cargo test`, which CI runs, showed the leak.
///
/// ## Fix Applied
///
/// `regen` pipes the check's stderr and passes it on line by line through
/// `eprint!`, which the harness captures and a build script's stderr receives.
///
/// ## Prevention
///
/// This test re-runs that test through its own binary, under the harness's
/// default capture, and fails on any of the check's output.
///
/// ## Pitfall
///
/// Only `print!` and `eprint!` are captured: writing to `std::io::stderr()`,
/// or handing a child the descriptor, leaks again.
// test_kind: mre
#[ cfg( feature = "auto_regen" ) ]
#[ test ]
fn auto_regen_mre_keeps_the_nested_check_output_captured()
{
  let output = std::process::Command::new( std::env::current_exe().expect( "this test binary" ) )
  .args( [ "--exact", "auto_regen_finds_database_url_in_dotenv" ] )
  .env_remove( "RUST_TEST_NOCAPTURE" )
  .output()
  .expect( "re-run this test binary" );
  let printed = format!( "{}{}", String::from_utf8_lossy( &output.stdout ), String::from_utf8_lossy( &output.stderr ) );

  assert!( output.status.success() && printed.contains( "1 passed" ), "{printed}" );
  assert!( !printed.contains( "could not compile" ), "the nested check's output escaped capture:\n{printed}" );
}

/// To a crate whose `sqlx.toml` names another variable, `DATABASE_URL` alone is
/// no database: no nested check is spent, and the notice names the variable
/// sqlx actually reads.
#[ cfg( feature = "auto_regen" ) ]
#[ test ]
fn auto_regen_follows_the_sqlx_toml_variable()
{
  let package = stale_package();
  package.write( "sqlx.toml", "[common]\ndatabase-url-var = \"APP_DATABASE_URL\"\n" );

  let directives = package.directives( &[ ( "SQLX_QUERY_AUTO_REGEN", "1" ), ( "DATABASE_URL", "sqlite::memory:" ) ] );
  let expected = [ "cargo:warning=sqlx_query: auto_regen skipped: APP_DATABASE_URL is unset or empty" ];
  assert_eq!( regeneration_reports( &directives ), expected, "{directives:#?}" );
}

/// sqlx writes no entry into an explicit `SQLX_OFFLINE_DIR` that is not a
/// directory, so a nested check against it could only fail: none is spent, and
/// the notice names the directory.
#[ cfg( feature = "auto_regen" ) ]
#[ test ]
fn auto_regen_skips_a_missing_offline_dir()
{
  let package = stale_package();
  let missing = package.root().join( "missing_cache" );

  let env =
  [
    ( "SQLX_QUERY_AUTO_REGEN", "1" ),
    ( "DATABASE_URL", "sqlite::memory:" ),
    ( "SQLX_OFFLINE_DIR", missing.to_str().expect( "a UTF-8 temporary path" ) ),
  ];
  let directives = package.directives( &env );
  let expected = format!( "cargo:warning=sqlx_query: auto_regen skipped: SQLX_OFFLINE_DIR names {}, which is not a directory", missing.display() );
  assert_eq!( regeneration_reports( &directives ), [ expected.as_str() ], "{directives:#?}" );
}
