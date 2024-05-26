// #![ deny( missing_docs ) ]

#[ allow( unused_imports ) ]
use super::*;

#[ cfg( feature = "derive_former" ) ]
#[ path = "../../../former/tests/inc/former_tests" ]
mod former_tests
{
  #[ allow( unused_imports ) ]
  use super::*;

  // = basic

  #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
  mod a_basic_manual;
  mod a_primitives_manual;

  #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
  mod subform_collection_basic_manual;

  // = parametrization

  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod parametrized_struct_manual;
  mod parametrized_slice_manual;

}

#[ cfg( feature = "types_components" ) ]
#[ path = "../../../former/tests/inc/components_tests" ]
mod components_tests
{
  use super::*;

  #[ cfg( feature = "types_component_from" ) ]
  mod component_from_manual;

  #[ cfg( feature = "types_component_assign" ) ]
  mod component_assign_manual;

  #[ cfg( all( feature = "types_component_assign" ) ) ]
  mod components_assign_manual;

  // #[ cfg( all( feature = "derive_from_components" ) ) ]
  mod from_components_manual;

  #[ cfg( all( feature = "types_component_assign" ) ) ]
  mod composite_manual;

}
