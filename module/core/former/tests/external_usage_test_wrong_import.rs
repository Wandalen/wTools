#![ allow( dead_code ) ]

//! This test reproduces the ACTUAL problem from `task/improve_former_derive_user_experience.md`
//!
//! ## The Real Issue
//!
//! When users import `Former` directly from `former_meta` (bypassing the re-exports),
//! they get 90+ cryptic errors because trait dependencies from `former_types` are not in scope.
//!
//! ## This test SHOULD FAIL with errors like:
//! ```
//! error[E0220]: associated type `Storage` not found for `Definition`
//! error[E0220]: associated type `Context` not found for `Definition`
//! error[E0433]: failed to resolve: could not find `FormingEnd` in `former`
//! error[E0433]: failed to resolve: could not find `StoragePreform` in `former`
//! ```

// WRONG IMPORT PATTERN - bypasses the helpful re-exports
use former_meta::Former;

/// Test configuration struct demonstrating broken import pattern.
///
/// This will FAIL to compile because `FormingEnd`, `StoragePreform`, etc. are not in scope.
#[ derive( Debug, PartialEq, Former ) ]
pub struct BrokenConfig
{
  host : String,
  port : u16,
}

#[ test ]
fn test_broken_usage()
{
  // This code is unreachable because the struct definition above fails to compile
  let _config = BrokenConfig::former()
    .host( "localhost".to_string() )
    .port( 8080_u16 )
    .form();
}
