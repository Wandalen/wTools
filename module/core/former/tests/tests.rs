#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! All tests.
#![allow(unused_imports)]

include!("../../../../module/step/meta/src/module/terminal.rs");

use former as the_module;

#[ cfg( feature = "enabled" ) ]
mod inc;
