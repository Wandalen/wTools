use sha2::{ Digest, Sha256 };

/// Compute the lowercase hex-encoded SHA-256 digest of `sql`, matching
/// sqlx's own cache key derivation (`sqlx-macros-core::query::data::hash_string`).
///
/// The digest covers the SQL text alone, by design -- the function takes no
/// bind-argument parameter at all, so it cannot drift into hashing bind types
/// too. See `docs/invariant/001_query_dispatch_invariant.md` (Hash Algorithm
/// Parity) for why that distinction matters.
#[ must_use ]
pub fn hash_query_sql( sql : &str ) -> String
{
  let digest = Sha256::digest( sql.as_bytes() );
  to_hex( &digest )
}

/// Build the `.sqlx` cache filename sqlx would use for `sql`.
#[ must_use ]
pub fn cache_filename( sql : &str ) -> String
{
  format!( "query-{}.json", hash_query_sql( sql ) )
}

/// Hand-rolled lowercase hex encoding -- avoids a `hex` crate dependency for
/// a single, trivial byte-to-hex conversion.
fn to_hex( bytes : &[ u8 ] ) -> String
{
  const DIGITS : &[ u8; 16 ] = b"0123456789abcdef";
  let mut out = String::with_capacity( bytes.len() * 2 );
  for byte in bytes
  {
    out.push( DIGITS[ ( byte >> 4 ) as usize ] as char );
    out.push( DIGITS[ ( byte & 0x0f ) as usize ] as char );
  }
  out
}
