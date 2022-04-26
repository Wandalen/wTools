#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Make sha-1 hash for data.
//!

use sha1::{Sha1, Digest};

///
/// Make sha-1 hash for data.
///

pub fn hash( data : &[ u8 ] ) -> Vec<u8>
{
  let mut hasher = Sha1::new();
  hasher.update( data );
  let result = hasher.finalize();
  result.to_vec()
}
