//! Cache-entry naming parity with sqlx: a call site counts as fresh exactly
//! when `.sqlx/query-<lowercase hex SHA-256 of its SQL text>.json` exists
//! (`docs/invariant/001_query_dispatch_invariant.md`, Hash Algorithm Parity).
//!
//! Every digest below was computed independently of this crate (Python's
//! `hashlib.sha256`, `sha256sum`), so a regression in `sqlx_query`'s own
//! hashing or hex encoding surfaces as a stale warning. Upstream drift in
//! sqlx's derivation is `golden_fixture_hash_canary_test.rs`'s job.
//!
//! ## Test Matrix
//!
//! | Test Case | Scenario | Input | Expected | Status |
//! |-----------|----------|-------|----------|--------|
//! | `verified_sha256_vectors_name_the_cache_entries` | Known digests, empty SQL as the boundary | four `query!` literals, each with `.sqlx/query-<digest>.json` | stale without the entry, fresh with it | ✅ |
//! | `unicode_sql_is_hashed_as_utf8` | Non-ASCII SQL text | `SELECT 'привіт' AS greeting` | fresh with the entry named by its UTF-8 digest | ✅ |
//! | `cache_entry_name_must_match_exactly` | Near-miss names | `<digest>.json`, `query-<digest>`, a directory `query-<digest>.json/` | still stale | ✅ |
//! | Bind arguments never change the key | — | — | `call_site_scan_test.rs`, `bind_arguments_never_change_the_cache_key` | N/A here |
//! | Uppercase hex digest | Case-insensitive filesystems match it, as sqlx's own lookup does | — | — | N/A: filesystem-dependent |

#![ cfg( feature = "enabled" ) ]

mod support;

use support::{ assert_site_keyed_by, site_warning, site_warnings, PackageDir };

/// The single digest every near-miss name below is derived from: `SELECT 1`'s.
const SELECT_1_DIGEST : &str = "e004ebd5b5532a4b85984a62f8ad48a81aa3460c1ca07701f386135d72cdecf5";

/// Each vector names the entry sqlx writes for that SQL text; `""` is the
/// boundary case, SHA-256's well-known empty-input digest.
#[ test ]
fn verified_sha256_vectors_name_the_cache_entries()
{
  let vectors =
  [
    ( "SELECT 1", SELECT_1_DIGEST ),
    ( "", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855" ),
    ( "SELECT * FROM users WHERE id = $1", "843923b9a0257cf80f1dff554e7dc8fdfc05f489328e8376513124dfb42996e3" ),
    ( "INSERT INTO users (name, email) VALUES ($1, $2)", "9702ad832ffcf6bbfc8df6c21b5d8642b0fa1c914f2f3ec1bd67ff00c4d2ea7d" ),
  ];

  for ( sql, digest ) in vectors
  {
    let package = PackageDir::new( &format!( "fn f() {{ let _ = sqlx::query!( {sql:?} ); }}\n" ) );
    assert_site_keyed_by( &package, 1, &format!( "query-{digest}.json" ) );
  }
}

/// sqlx hashes the SQL text's UTF-8 bytes; a non-ASCII literal must produce
/// the same digest, not one of some other encoding or of the escaped source.
#[ test ]
fn unicode_sql_is_hashed_as_utf8()
{
  let package = PackageDir::new( "fn f() { let _ = sqlx::query!( \"SELECT 'привіт' AS greeting\" ); }\n" );
  assert_site_keyed_by( &package, 1, "query-68a3fedb8a581e3309b3acd4a196b06e994af3011121c84873b51b5bd0909d00.json" );
}

/// Only the exact `query-<digest>.json` file counts: a name missing its prefix
/// or suffix, or a directory carrying the right name, is not sqlx's entry.
#[ test ]
fn cache_entry_name_must_match_exactly()
{
  let near_misses = [ format!( "{SELECT_1_DIGEST}.json" ), format!( "query-{SELECT_1_DIGEST}" ) ];
  for name in near_misses
  {
    let package = PackageDir::new( "fn f() { let _ = sqlx::query!( \"SELECT 1\" ); }\n" );
    package.write( &format!( ".sqlx/{name}" ), "{}" );

    let directives = package.directives( &[] );
    assert_eq!( site_warnings( &directives ), [ site_warning( "src/lib.rs", 1 ) ], "{name} must not count: {directives:#?}" );
  }

  let package = PackageDir::new( "fn f() { let _ = sqlx::query!( \"SELECT 1\" ); }\n" );
  package.mkdir( &format!( ".sqlx/query-{SELECT_1_DIGEST}.json" ) );

  let directives = package.directives( &[] );
  assert_eq!( site_warnings( &directives ), [ site_warning( "src/lib.rs", 1 ) ], "a directory must not count: {directives:#?}" );
}
