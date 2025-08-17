use test_tools::exposed::*;
use super::*;

#[path = "../../../component_model/tests/inc/components_tests"]
mod components_tests {
  use super::*;

  mod component_from_manual;

  #[ cfg( feature = "types_component_assign" ) ]
  mod component_assign_manual;

  #[cfg(all(feature = "types_component_assign"))]
  mod components_assign_manual;

  // #[ cfg( all( feature = "derive_from_components" ) ) ]
  mod from_components_manual;

  #[cfg(all(feature = "types_component_assign"))]
  mod composite_manual;
}
