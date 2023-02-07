use super::*;
use assert_cmd::Command;
use predicates::prelude::*;

const MODULE_NAME : &str = "willbe";

mod each;
mod info;
mod dependency;
