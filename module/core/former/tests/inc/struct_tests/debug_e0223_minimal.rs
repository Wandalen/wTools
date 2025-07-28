//! Test case for debugging E0223 error in subform_collection
//! This is a minimal reproduction test

use super::*;

// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// See: /home/user1/pro/lib/wTools/module/core/macro_tools/task/task_issue.md
// #[derive(Default, Debug, PartialEq, former::Former)]
#[derive(Default, Debug, PartialEq)]
pub struct MinimalStruct {
  // #[subform_collection( definition = former::VectorDefinition )]
  vec_1: Vec<String>,
}

#[test]
#[ignore = "Disabled until Former derive is re-enabled"]
fn minimal_test() {
  // let _instance = MinimalStruct::former()
  //   .vec_1()
  //   .add("test".to_string())
  //   .end()
  //   .form();
}