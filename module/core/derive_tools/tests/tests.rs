//! Tests for the `derive_tools` crate.
#![allow(unused_imports)]

include!("../../../../module/step/meta/src/module/terminal.rs");

use derive_tools as the_module;
use test_tools :: *;

#[ cfg( feature = "enabled" ) ]
mod inc;
