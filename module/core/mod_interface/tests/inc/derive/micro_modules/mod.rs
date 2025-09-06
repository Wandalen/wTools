#![allow(dead_code)]
#![allow(clippy::doc_markdown)]
use super::*;
use test_tools::a_id;

/// Private namespace of the module.
mod private {}

the_module::mod_interface! {
  // #![ debug ]

  /// mod_own
  own mod mod_own;
  /// mod_orphan
  orphan mod mod_orphan;
  /// mod_exposed
  exposed mod mod_exposed;
  /// mod_prelude
  prelude mod mod_prelude;

}

//

include!("../../only_test/micro_modules_only_test.rs");
