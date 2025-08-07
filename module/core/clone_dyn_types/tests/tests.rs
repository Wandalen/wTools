//! Test suite for the `clone_dyn_types` crate.

#[allow(unused_imports)]
use clone_dyn_types as the_module;
#[allow(unused_imports)]
use test_tools::exposed::*;

#[cfg(feature = "enabled")]
mod inc;
