#[ allow( unused_imports ) ]
use super::*;

#[ cfg( feature = "derive_former" ) ]
mod former_tests
{
  #[ allow( unused_imports ) ]
  use super::*;

  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod container_former_common;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod container_former_vec;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod container_former_hashset;
  // #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  // mod container_former_hashmap;

  // mod a_primitives_manual;
  // mod a_primitives;
  // mod a_primitives_expanded;
//   mod a_containers_without_runtime_manual;
//   mod a_containers_without_runtime;
//   #[ cfg( not( feature = "no_std" ) ) ]
//   mod a_containers_with_runtime_manual;
//   #[ cfg( not( feature = "no_std" ) ) ]
//   mod a_containers_with_runtime ;
//
//   mod attribute_default_container;
//   mod attribute_default_primitive;
//   mod attribute_perform;
//   mod attribute_setter;
//   mod attribute_alias;
//
//   mod string_slice_manual;
//   mod string_slice;
//   mod unsigned_primitive_types;
//   mod default_user_type;
//   mod user_type_no_default;
//   mod user_type_no_debug;
//
//   mod name_collision_former_hashmap_without_parameter;
//   mod name_collision_former_vector_without_parameter;
//   mod name_collisions;
//   mod name_collision_context;
//   mod name_collision_end;
//   mod name_collision_on_end;
//
//   #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
//   mod parametrized_struct_manual;
//   #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
//   mod parametrized_struct_imm;
//   #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
//   mod parametrized_struct_where;
//
//   #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
//   mod subformer_basic_manual;
//   #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
//   mod subformer_basic;
//
//   #[ cfg( any( not( feature = "no_std" ) ) ) ]
//   mod subformer_shortcut;

}

#[ cfg( feature = "derive_components" ) ]
mod components_tests
{
  use super::*;

  #[ cfg( feature = "derive_component_from" ) ]
  mod component_from_manual;
  #[ cfg( feature = "derive_component_from" ) ]
  mod component_from;

  #[ cfg( feature = "derive_component_assign" ) ]
  mod component_assign_manual;
  #[ cfg( feature = "derive_component_assign" ) ]
  mod component_assign;

  #[ cfg( all( feature = "derive_component_assign", feature = "derive_components_assign" ) ) ]
  mod components_assign_manual;
  #[ cfg( all( feature = "derive_component_assign", feature = "derive_components_assign" ) ) ]
  mod components_assign;

  #[ cfg( all( feature = "derive_from_components" ) ) ]
  mod from_components_manual;
  #[ cfg( all( feature = "derive_from_components" ) ) ]
  mod from_components;

  #[ cfg( all( feature = "derive_component_from", feature = "derive_component_assign", feature = "derive_components_assign", feature = "derive_from_components" ) ) ]
  mod composite_manual;
  #[ cfg( all( feature = "derive_component_from", feature = "derive_component_assign", feature = "derive_components_assign", feature = "derive_from_components" ) ) ]
  mod composite;

}

only_for_terminal_module!
{

  // stable have different information about error
  // that's why these tests are active only for nightly
  #[ test_tools::nightly ]
  #[ test ]
  fn former_trybuild()
  {

    println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
    // let t = test_tools::compiletime::TestCases::new();

    // xxx
    // t.compile_fail( "tests/inc/compiletime/former_bad_attr.rs" );
    // t.pass( "tests/inc/compiletime/former_hashmap_without_parameter.rs" );
    // t.pass( "tests/inc/compiletime/former_vector_without_parameter.rs" );

  }

  // stable have different information about error
  // that's why these tests are active only for nightly
  #[ test_tools::nightly ]
  #[ test ]
  fn components_trybuild()
  {

    println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
    let _t = test_tools::compiletime::TestCases::new();

    //t.compile_fail( "tests/inc/compiletime/components_component_from_debug.rs" );

  }

}
