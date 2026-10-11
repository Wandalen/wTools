//! Real end-to-end build under the environment `cargo sqlx prepare` gives its
//! nested `cargo check`: `SQLX_OFFLINE=false`, `SQLX_OFFLINE_DIR` naming the
//! cache directory, and `DATABASE_URL`.
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Input | Expected | Status |
//! |-----------|----------|-------|----------|--------|
//! | `prepare_environment_rewrites_the_cache` | Every entry deleted and the sources touched, as `prepare` does | `SQLX_OFFLINE=false`, `SQLX_OFFLINE_DIR=<.sqlx>` | succeeds; the same entries are written back | ✅ |

#![ cfg( feature = "enabled" ) ]

mod support;

/// Real nested `cargo build`s, minutes when cold -- run explicitly with
/// `cargo nextest run -p sqlx_query --run-ignored all`.
///
/// `cargo sqlx prepare` deletes every `query-*.json`, touches the sources and
/// runs `cargo check` under the environment above, expecting sqlx to write each
/// entry back. Forcing `SQLX_OFFLINE=true` over its explicit `false` -- what
/// `sqlx_query` did whenever the cache was stale, exactly when `prepare` is
/// needed -- made every query fail instead (`build_script_directives_test.rs`,
/// `explicit_offline_false_mre_is_never_overridden`). `cargo build` stands in
/// for `prepare`'s `cargo check`, so no `sqlx-cli` install is needed: only the
/// environment matters to either crate.
#[ tokio::test ]
#[ ignore = "spawns real nested `cargo build`s; run with --run-ignored all" ]
async fn prepare_environment_rewrites_the_cache()
{
  let fixture = support::FixtureCrate::new( "SELECT id FROM t" ).await;
  fixture.bootstrap_cache();
  let expected = fixture.cache_entries();
  for entry in &expected
  {
    std::fs::remove_file( entry ).unwrap_or_else( | error | panic!( "remove {}: {error}", entry.display() ) );
  }
  fixture.set_query_sql( "SELECT id FROM t" );

  let sqlx_dir = fixture.sqlx_dir().display().to_string();
  let output = fixture.build( &[ ( "SQLX_OFFLINE", "false" ), ( "SQLX_OFFLINE_DIR", sqlx_dir.as_str() ) ] );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!( output.status.success(), "a build under prepare's environment should compile live: {stderr}" );
  assert_eq!( fixture.cache_entries(), expected, "sqlx should write back every entry prepare deleted" );
}
