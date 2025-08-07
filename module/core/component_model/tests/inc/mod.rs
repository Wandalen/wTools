//! # Test Module Structure and Coverage Outline

use super::*;
use test_tools::exposed::*;

#[cfg(feature = "derive_components")]
mod components_tests {
  use super::*;

  #[cfg(feature = "derive_component_from")]
  mod component_from;
  #[cfg(feature = "derive_component_from")]
  mod component_from_manual;
  #[cfg(feature = "derive_component_from")]
  mod component_from_tuple;
  #[cfg(feature = "derive_component_from")]
  mod component_from_tuple_manual;

  #[cfg(feature = "derive_component_assign")]
  mod component_assign;
  #[cfg(feature = "derive_component_assign")]
  mod component_assign_manual;
  #[cfg(feature = "derive_component_assign")]
  mod component_assign_tuple;
  #[cfg(feature = "derive_component_assign")]
  mod component_assign_tuple_manual;

  #[cfg(all(feature = "derive_component_assign", feature = "derive_components_assign"))]
  mod components_assign;
  #[cfg(all(feature = "derive_component_assign", feature = "derive_components_assign"))]
  mod components_assign_manual;
  #[cfg(all(feature = "derive_component_assign", feature = "derive_components_assign"))]
  mod components_assign_tuple;
  #[cfg(all(feature = "derive_component_assign", feature = "derive_components_assign"))]
  mod components_assign_tuple_manual;

  #[cfg(all(feature = "derive_from_components"))]
  mod from_components;
  #[cfg(all(feature = "derive_from_components"))]
  mod from_components_manual;
  #[cfg(all(feature = "derive_from_components"))]
  mod from_components_tuple;
  #[cfg(all(feature = "derive_from_components"))]
  mod from_components_tuple_manual;

  #[cfg(all(
    feature = "derive_component_from",
    feature = "derive_component_assign",
    feature = "derive_components_assign",
    feature = "derive_from_components"
  ))]
  mod composite;
  #[cfg(all(
    feature = "derive_component_from",
    feature = "derive_component_assign",
    feature = "derive_components_assign",
    feature = "derive_from_components"
  ))]
  mod composite_manual;
}

only_for_terminal_module! {

  // stable have different information about error
  // that's why these tests are active only for nightly
  #[ test_tools::nightly ]
  #[ test ]
  fn components_trybuild()
  {

    println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
    let _t = test_tools::compiletime::TestCases::new();

    // zzz : make it working test
    //t.run( "tests/inc/components_tests/compiletime/components_component_from_debug.rs" );

  }

}
