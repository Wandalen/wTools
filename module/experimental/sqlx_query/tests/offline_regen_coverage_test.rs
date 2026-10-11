//! Real `auto_regen` builds: the nested `cargo check` repairs a stale query
//! wherever the outer build may compile it, and reports what it cannot reach
//! (`docs/feature/002_auto_regeneration.md`).
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Input | Expected | Status |
//! |-----------|----------|-------|----------|--------|
//! | `auto_regen_mre_repairs_queries_outside_the_default_targets` | Stale queries in an integration test, one never compiled (MRE) | `SQLX_QUERY_AUTO_REGEN=1` | the test's entry written; "regenerated 1 of 2" | ✅ |
//! | No deadlock on the shared target directory | — | — | `offline_regen_deadlock_guard_test.rs` | N/A here |

#![ cfg( feature = "enabled" ) ]

mod support;

/// An integration test holding one query `cargo check --all-targets` compiles,
/// and one that no configuration compiles.
const INTEGRATION_TEST : &str = "#[ test ]
fn in_test()
{
  let _ = sqlx::query!( \"SELECT id AS in_test FROM t\" );
}

#[ cfg( any() ) ]
fn never()
{
  let _ = sqlx::query!( \"SELECT id AS never_compiled FROM t\" );
}
";

/// `auto_regen` repairs a stale query outside the library -- in a test, an
/// example, a bench, or behind a feature -- and says how many entries are
/// still missing when some code is compiled by no configuration at all.
///
/// ## Root Cause
///
/// `regen::attempt_regeneration` in `src/regen.rs` ran a plain `cargo check`:
/// default targets, default features. The scan covers every `.rs` file in the
/// package, so a stale query in `tests/` was reported but never compiled by
/// the nested check, and never written: `cargo test` kept failing with
/// `SQLX_QUERY_AUTO_REGEN=1` set, right after a report claiming "regenerated
/// stale .sqlx entries".
///
/// ## Why Not Caught
///
/// The only end-to-end regeneration test kept its query in `src/lib.rs`, which
/// a plain `cargo check` compiles; manual scenario S13 put one in `tests/`.
///
/// ## Fix Applied
///
/// The nested check runs with `--all-targets --all-features`, the flags the
/// fail-fast summary already recommends for `cargo sqlx prepare`, and
/// `regen::regenerated` reports from the re-diff that follows it, not from its
/// exit status.
///
/// ## Prevention
///
/// A repair step must compile at least what the build it repairs may compile,
/// and report what the re-check found, not what the tool returned.
///
/// ## Pitfall
///
/// Code that no configuration compiles -- `#[ cfg( any() ) ]`, another
/// platform's `cfg` -- stays stale whatever runs; the report must count it
/// instead of claiming success.
// test_kind: mre
#[ tokio::test ]
#[ ignore = "spawns real nested `cargo build` + `cargo check` processes; run with --run-ignored all" ]
async fn auto_regen_mre_repairs_queries_outside_the_default_targets()
{
  let fixture = support::FixtureCrate::new( "SELECT id FROM t" ).await;
  fixture.bootstrap_cache();
  fixture.write( "tests/it.rs", INTEGRATION_TEST );

  let output = fixture.build( &[ ( "SQLX_QUERY_AUTO_REGEN", "1" ) ] );
  let stderr = String::from_utf8_lossy( &output.stderr );

  assert!( output.status.success(), "`cargo build` compiles neither stale query: {stderr}" );
  let entry = fixture.sqlx_dir().join( support::entry_name( "SELECT id AS in_test FROM t" ) );
  assert!( entry.is_file(), "the integration test's query should be regenerated: {stderr}" );
  assert!( stderr.contains( "regenerated 1 of 2 stale .sqlx entries" ), "the report should count the entry nothing compiles: {stderr}" );
}
