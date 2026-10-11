//! Real nested-build coverage guarding against the isolated-target-dir
//! regeneration deadlock (see `docs/invariant/002_consumer_integration_contract.md`).
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Input | Expected | Status |
//! |-----------|----------|-------|----------|--------|
//! | `auto_regen_completes_without_deadlocking_on_shared_target_dir` | Stale entry, live database, regeneration on | `SQLX_QUERY_AUTO_REGEN=1` | regenerates, build succeeds, within the bound | ✅ |
//! | Regeneration skipped without a database, or under `SQLX_OFFLINE=true` | — | — | `build_script_directives_test.rs` | N/A here |

#![ cfg( feature = "enabled" ) ]

mod support;

/// Real nested `cargo build` plus a further nested `cargo check` (via
/// `auto_regen`); run explicitly with
/// `cargo nextest run -p sqlx_query --features auto_regen --run-ignored all`.
///
/// The bounded wait itself is the assertion that matters here: a regression
/// that points the nested `cargo check` at the shared/default target
/// directory (see `src/regen.rs` and
/// `docs/invariant/002_consumer_integration_contract.md`, Isolated
/// Target-Dir Regeneration) would hang this build indefinitely. A cold,
/// isolated `cargo check` of sqlx's dependency tree alone took ~42s when
/// measured, and this build pays a cold compile twice -- the nested check's
/// isolated target directory and the outer build's own -- so the bound
/// (`support::BUILD_BOUND`) is generous rather than tight; a real deadlock
/// never returns at all. `support::run_bounded` kills the child and fails the
/// test instead of stalling the whole suite.
#[ tokio::test ]
#[ ignore = "spawns real nested `cargo build` + `cargo check` processes; run with --run-ignored all" ]
async fn auto_regen_completes_without_deadlocking_on_shared_target_dir()
{
  let fixture = support::FixtureCrate::new( "SELECT id FROM t" ).await;
  fixture.bootstrap_cache();

  fixture.set_query_sql( "SELECT id FROM t WHERE id > 0" );

  let output = fixture.build( &[ ( "SQLX_QUERY_AUTO_REGEN", "1" ) ] );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!( stderr.contains( "regenerated stale .sqlx entries" ), "expected auto_regen to report success: {stderr}" );
  assert!( output.status.success(), "build should succeed once auto_regen repairs the cache: {stderr}" );
}
