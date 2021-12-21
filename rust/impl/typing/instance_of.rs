#![ warn( missing_docs ) ]

//!
//! Macro to answer the question: does it implement a trait?
//!
//! This solution has a limitation:
//! - In case enity is a function and trat is `Fn`/`FnMut`/`FnOnce` which current entity does not implement you will get compile-time error instead of `false`.
//!

pub use implements::implements;
pub use implements::instance_of;
