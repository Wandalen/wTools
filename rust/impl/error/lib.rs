#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Basic exceptions handling mechanism.
//!
//! # Sample
//! ```
//! use werror::*;
//!
//! let err1 = Error::new( "Some error" );
//! println!( "err1 : {}", err1 );
//! // < err1 : Some error
//! ```

mod assert;
mod error;

pub use assert::*;
pub use error::*;
