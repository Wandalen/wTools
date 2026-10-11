//! Real end-to-end builds of a consumer crate whose `.sqlx` cache is stale:
//! the fail-fast default, the `SQLX_QUERY_SOFT_WARN` opt-out, and the build
//! script reruns both depend on.
//!
//! Every fixture runs `SELECT id FROM t` at `src/lib.rs:3` until a test
//! changes it; all of a fixture's builds share one target directory.
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Input | Expected | Status |
//! |-----------|----------|-------|----------|--------|
//! | `stale_cache_with_live_db_fails_fast_by_default` | SQL changed, cache not | plain build with a live `DATABASE_URL` | fails, naming the stale site | ✅ |
//! | `stale_cache_with_soft_warn_self_heals` | Same, opted out up front | `SQLX_QUERY_SOFT_WARN=1` | succeeds; the new entry is written | ✅ |
//! | `soft_warn_retry_after_a_failed_build_self_heals` | The failing build's own advice, followed | failed plain build, then `SQLX_QUERY_SOFT_WARN=1`, no source change | retry succeeds; the new entry is written | ✅ |
//! | `deleted_cache_entry_fails_the_next_build` | Entry removed after a good build | delete the only entry, nothing else | next build fails, naming the site | ✅ |
//! | `soft_warn_into_a_missing_offline_dir_fails_until_it_is_created` | `SQLX_OFFLINE_DIR` names no directory | `SQLX_QUERY_SOFT_WARN=1`, then the same build after creating it | fails with sqlx's error, as the summary says; then succeeds, writing the entry there | ✅ |

#![ cfg( feature = "enabled" ) ]

mod support;

use support::FixtureCrate;

/// The changed SQL every test switches its fixture to.
const CHANGED_SQL : &str = "SELECT id FROM t WHERE id > 0";

/// How Cargo prints `sqlx_query`'s warning for the fixture's call site.
const STALE_SITE : &str = "sqlx_query: src/lib.rs:3 -- no .sqlx entry matches this query's SQL";

/// Real nested `cargo build`s, minutes when cold -- run explicitly with
/// `cargo nextest run -p sqlx_query --run-ignored all`.
#[ tokio::test ]
#[ ignore = "spawns real nested `cargo build`s; run with --run-ignored all" ]
async fn stale_cache_with_live_db_fails_fast_by_default()
{
  let fixture = FixtureCrate::new( "SELECT id FROM t" ).await;
  fixture.bootstrap_cache();
  fixture.set_query_sql( CHANGED_SQL );

  let output = fixture.build( &[] );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!( stderr.contains( STALE_SITE ), "expected the stale site to be named: {stderr}" );
  assert!( !output.status.success(), "a stale cache should fail the build even with a live DATABASE_URL: {stderr}" );
}

/// Real nested `cargo build`s, minutes when cold -- run explicitly with
/// `cargo nextest run -p sqlx_query --run-ignored all`.
///
/// The bootstrap already wrote the original SQL's entry; a second entry
/// proves the live build wrote the changed SQL's, rather than that the old
/// one is merely still there.
#[ tokio::test ]
#[ ignore = "spawns real nested `cargo build`s; run with --run-ignored all" ]
async fn stale_cache_with_soft_warn_self_heals()
{
  let fixture = FixtureCrate::new( "SELECT id FROM t" ).await;
  fixture.bootstrap_cache();
  fixture.set_query_sql( CHANGED_SQL );

  let output = fixture.build( &[ ( "SQLX_QUERY_SOFT_WARN", "1" ) ] );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!( output.status.success(), "SQLX_QUERY_SOFT_WARN=1 should let the live build run and self-heal: {stderr}" );
  assert!( stderr.contains( STALE_SITE ), "the stale site should still be reported: {stderr}" );
  assert_eq!( fixture.cache_entries().len(), 2, "the live build should write the changed SQL's entry" );
}

/// Real nested `cargo build`s, minutes when cold -- run explicitly with
/// `cargo nextest run -p sqlx_query --run-ignored all`.
///
/// The failing build tells the developer to rebuild with
/// `SQLX_QUERY_SOFT_WARN=1`; doing exactly that, with no source change, must
/// work. Only the environment differs between the two builds, so the retry
/// succeeds only if Cargo re-runs the build script for it -- before
/// `cargo:rerun-if-env-changed` it replayed the forced `SQLX_OFFLINE=true` and
/// failed again (manual scenario S1;
/// `build_script_directives_test.rs`, `rerun_triggers_mre_cover_every_input`).
#[ tokio::test ]
#[ ignore = "spawns real nested `cargo build`s; run with --run-ignored all" ]
async fn soft_warn_retry_after_a_failed_build_self_heals()
{
  let fixture = FixtureCrate::new( "SELECT id FROM t" ).await;
  fixture.bootstrap_cache();
  fixture.set_query_sql( CHANGED_SQL );

  let failed = fixture.build( &[] );
  assert!( !failed.status.success(), "the plain build should fail first: {}", String::from_utf8_lossy( &failed.stderr ) );

  let retry = fixture.build( &[ ( "SQLX_QUERY_SOFT_WARN", "1" ) ] );
  let stderr = String::from_utf8_lossy( &retry.stderr );
  assert!( retry.status.success(), "the suggested retry should succeed without any source change: {stderr}" );
  assert_eq!( fixture.cache_entries().len(), 2, "the retry should write the changed SQL's entry" );
}

/// Real nested `cargo build`s, minutes when cold -- run explicitly with
/// `cargo nextest run -p sqlx_query --run-ignored all`.
///
/// Deleting an entry -- a bad merge, a cleanup -- changes nothing Cargo's
/// default tracking sees: `.sqlx/` is hidden, so outside git it is not part of
/// the package listing. The next build fails only because the cache directory
/// is a declared rerun input (manual scenarios S5c and S10); before that it
/// silently reused the previous result.
#[ tokio::test ]
#[ ignore = "spawns real nested `cargo build`s; run with --run-ignored all" ]
async fn deleted_cache_entry_fails_the_next_build()
{
  let fixture = FixtureCrate::new( "SELECT id FROM t" ).await;
  fixture.bootstrap_cache();
  let good = fixture.build( &[] );
  assert!( good.status.success(), "the build with a fresh cache should succeed: {}", String::from_utf8_lossy( &good.stderr ) );

  for entry in fixture.cache_entries()
  {
    std::fs::remove_file( &entry ).unwrap_or_else( | error | panic!( "remove {}: {error}", entry.display() ) );
  }

  let output = fixture.build( &[] );
  let stderr = String::from_utf8_lossy( &output.stderr );
  assert!( stderr.contains( STALE_SITE ), "expected the now-stale site to be named: {stderr}" );
  assert!( !output.status.success(), "the build should fail on the deleted entry: {stderr}" );
}

/// Real nested `cargo build`s, minutes when cold -- run explicitly with
/// `cargo nextest run -p sqlx_query --run-ignored all`.
///
/// sqlx never creates `SQLX_OFFLINE_DIR`: compiling live, it fails every query
/// with "sqlx offline path does not exist" instead of writing an entry. The
/// opt-out's summary must say exactly that -- it once promised the entry would
/// be written there (`build_script_directives_test.rs`,
/// `missing_offline_dir_mre_says_sqlx_cannot_write`) -- and this build checks
/// the promise against sqlx itself. Doing what the summary says, then
/// repeating the same command, must work: creating the directory re-runs the
/// check, which Cargo would otherwise replay.
#[ tokio::test ]
#[ ignore = "spawns real nested `cargo build`s; run with --run-ignored all" ]
async fn soft_warn_into_a_missing_offline_dir_fails_until_it_is_created()
{
  let fixture = FixtureCrate::new( "SELECT id FROM t" ).await;
  let missing = fixture.sqlx_dir().with_file_name( "missing_cache" );

  let env = [ ( "SQLX_QUERY_SOFT_WARN", "1" ), ( "SQLX_OFFLINE_DIR", missing.to_str().expect( "a UTF-8 temporary path" ) ) ];
  let output = fixture.build( &env );
  let stderr = String::from_utf8_lossy( &output.stderr );

  let summary = "which is not a directory: sqlx fails every query it compiles against DATABASE_URL until you create it";
  assert!( stderr.contains( summary ), "the summary should say why the live build fails: {stderr}" );
  assert!( stderr.contains( "sqlx offline path does not exist" ), "sqlx itself should fail on the missing directory: {stderr}" );
  assert!( !output.status.success(), "the live build should fail: {stderr}" );
  assert!( !missing.exists(), "sqlx should not have created {}", missing.display() );
  assert_eq!( fixture.cache_entries().len(), 0, "no entry should be written into .sqlx/ instead" );

  std::fs::create_dir( &missing ).unwrap_or_else( | error | panic!( "create {}: {error}", missing.display() ) );
  let retry = fixture.build( &env );
  let stderr = String::from_utf8_lossy( &retry.stderr );
  let written_into = format!( "sqlx writes its missing entry into {} (commit it)", missing.display() );
  assert!( retry.status.success(), "the same command should succeed once the directory exists: {stderr}" );
  assert!( stderr.contains( &written_into ), "the check should re-run, not replay \"not a directory\": {stderr}" );
  let written = std::fs::read_dir( &missing ).unwrap_or_else( | error | panic!( "read {}: {error}", missing.display() ) ).count();
  assert_eq!( written, 1, "the entry should be written into the created directory" );
}
