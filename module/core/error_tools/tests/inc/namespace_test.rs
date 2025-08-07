use super::*;

#[test]
fn exposed_main_namespace() {
  the_module::error::assert::debug_assert_id!(1, 1);
  use the_module::prelude::*;
  debug_assert_id!(1, 1);
}
