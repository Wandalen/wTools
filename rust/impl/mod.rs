#![warn( missing_docs )]
// #![ feature( concat_idents ) ]

//!
//! wTools - Collection of general purpose tools for solving problems. Fundamentally extend the language without spoiling, so may be used solely or in conjunction with another module of such kind.
//!

// pub mod former;
// pub mod meta;
// pub mod str;
// pub mod time;
// pub mod vector;

// pub use werror;

pub use inspect_type::*;
pub use is_slice::*;
pub use implements::*;

pub use former as former;
pub use werror as error;
pub use wtest_basic as test;

// former = { path = "../former", version = "~0" }
// inspect_type = { path = "../inspect_type", version = "~0" }
// is_slice = { path = "../is_slice", version = "~0" }
// implements = { path = "../implements", version = "~0" }
// wtest_basic = { path = "../wtest_basic", version = "~0" }
// werror = { path = "../werror", version = "~0" }
// wproc_macro = { path = "../wproc_macro", version = "~0", optional = true }
