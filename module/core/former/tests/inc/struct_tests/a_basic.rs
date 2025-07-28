#![deny(missing_docs)]

#[allow(unused_imports)]
use super::*;

// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// See: /home/user1/pro/lib/wTools/module/core/macro_tools/task/task_issue.md
// #[derive(Debug, PartialEq, former::Former)]
// #[ derive( Debug, PartialEq, former::Former ) ] #[ debug ]
#[ derive( Debug, PartialEq ) ]
pub struct Struct1 {
  pub int_1: i32,
}

// == begin of generated

// == end of generated

include!("./only_test/basic.rs");
