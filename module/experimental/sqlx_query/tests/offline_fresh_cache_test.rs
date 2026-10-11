//! Real end-to-end build of a consumer crate whose `.sqlx` cache is fresh.
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Input | Expected | Status |
//! |-----------|----------|-------|----------|--------|
//! | `fresh_cache_forces_offline_mode_without_warnings` | Cache bootstrapped live, then a plain build in the same target directory | `DATABASE_URL` naming a database that does not exist | succeeds offline; no stale warning | ✅ |

#![ cfg( feature = "enabled" ) ]

mod support;

/// Real nested `cargo build`s, minutes when cold -- run explicitly with
/// `cargo nextest run -p sqlx_query --run-ignored all`.
///
/// The measured build points `DATABASE_URL` at a database that does not
/// exist: it can only succeed offline, so success proves `SQLX_OFFLINE=true`
/// was forced. Both builds share one target directory, as a developer's do --
/// the measured build reports fresh only because the changed
/// `SQLX_QUERY_SOFT_WARN` re-runs the build script instead of replaying the
/// bootstrap's stale warning (`build_script_directives_test.rs`,
/// `rerun_triggers_mre_cover_every_input`).
#[ tokio::test ]
#[ ignore = "spawns real nested `cargo build`s; run with --run-ignored all" ]
async fn fresh_cache_forces_offline_mode_without_warnings()
{
  let fixture = support::FixtureCrate::new( "SELECT id FROM t" ).await;
  fixture.bootstrap_cache();

  let unreachable = format!( "sqlite://{}", fixture.sqlx_dir().with_file_name( "absent" ).join( "db.sqlite" ).display() );
  let output = fixture.build( &[ ( "DATABASE_URL", &unreachable ) ] );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!( output.status.success(), "a fresh cache should build offline, without reaching the database: {stderr}" );
  assert!( !stderr.contains( "no .sqlx entry matches" ), "unexpected staleness warning on a fresh cache: {stderr}" );
}
