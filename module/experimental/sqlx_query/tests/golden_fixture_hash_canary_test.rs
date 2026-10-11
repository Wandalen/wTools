//! Fast, no-subprocess canary against upstream drift. See
//! `tests/fixtures/readme.md` for how the checked-in fixture was produced,
//! and `docs/invariant/001_query_dispatch_invariant.md` (Hash Algorithm
//! Parity) for the residual risk this test exists to catch: sqlx's own
//! cache-key derivation is a private implementation detail, not a
//! semver-guaranteed contract.
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Input | Expected | Status |
//! |-----------|----------|-------|----------|--------|
//! | `checked_in_fixture_exists_at_the_expected_path` | Fixture not lost or renamed | `tests/fixtures/` | the golden entry exists | ✅ |
//! | `independently_computed_digest_matches_the_real_sqlx_generated_filename` | sqlx 0.9's derivation unchanged | `SELECT 1 as one` | SHA-256 names the real file | ✅ |
//! | `sqlx_query` matches the same derivation | — | — | `hash_test.rs` | N/A here |

#![ cfg( feature = "enabled" ) ]

mod support;

const GOLDEN_SQL : &str = "SELECT 1 as one";
const GOLDEN_FILENAME : &str =
  "query-bbf600f17712173206b754fd7c8f8f8fd46a03bf54e824ff8046c37a88407123.json";

#[ test ]
fn checked_in_fixture_exists_at_the_expected_path()
{
  let path = fixtures_dir().join( GOLDEN_FILENAME );
  assert!( path.is_file(), "missing golden fixture at {}; see tests/fixtures/readme.md", path.display() );
}

/// Deliberately re-derives SHA-256 independently of `src/hash.rs` (whose own
/// parity `hash_test.rs` verifies) -- the point of this test is to catch drift
/// in *sqlx's* algorithm, not to re-check our own hex encoding.
#[ test ]
fn independently_computed_digest_matches_the_real_sqlx_generated_filename()
{
  assert_eq!(
    support::entry_name( GOLDEN_SQL ), GOLDEN_FILENAME,
    "sqlx's cache-key derivation appears to have changed; see docs/invariant/001_query_dispatch_invariant.md"
  );
}

fn fixtures_dir() -> std::path::PathBuf
{
  std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) ).join( "tests" ).join( "fixtures" )
}
