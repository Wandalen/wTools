//! Integration tests for the component_model_types crate.
#![allow(unused_imports)]

include!("../../../../module/step/meta/src/module/aggregating.rs");

use component_model_types as the_module;

#[cfg(feature = "enabled")]
mod inc;
