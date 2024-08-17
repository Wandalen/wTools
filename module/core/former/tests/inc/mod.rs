// #![ deny( missing_docs ) ]

#[ allow( unused_imports ) ]
use super::*;

#[ cfg( feature = "derive_former" ) ]
mod former_tests
{
  #[ allow( unused_imports ) ]
  use super::*;

  // = basic

  #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
  mod a_basic_manual;
  #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
  mod a_basic;
  mod a_primitives_manual;
  mod a_primitives;
  mod tuple_struct;

  #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
  mod subform_collection_basic_scalar;
  #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
  mod subform_collection_basic_manual;
  #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
  mod subform_collection_basic;

  // = attribute

  #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
  mod attribute_default_collection;
  #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
  mod attribute_default_primitive;
  mod attribute_default_conflict;
  mod attribute_storage_with_end;
  mod attribute_storage_with_mutator;
  mod attribute_perform;
  mod attribute_setter;
  mod attribute_alias;
  mod attribute_feature;
  mod attribute_multiple;

  // = name collision

  mod name_collision_former_hashmap_without_parameter;
  mod name_collision_former_vector_without_parameter;
  mod name_collisions;
  // mod name_collision_context;
  // mod name_collision_end;
  // mod name_collision_on_end;
  // mod name_collision_core;

  // = parametrization

  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod parametrized_struct_manual;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod parametrized_struct_imm;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod parametrized_struct_where;
  mod parametrized_field;
  mod parametrized_field_where;

  mod parametrized_slice_manual;
  mod parametrized_slice;

  // = etc

  mod unsigned_primitive_types;
  mod default_user_type;
  mod user_type_no_default;
  mod user_type_no_debug;
  mod visibility;

  // = collection former

  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod collection_former_common;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod collection_former_btree_map;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod collection_former_btree_set;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod collection_former_binary_heap;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod collection_former_hashmap;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod collection_former_hashset;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod collection_former_linked_list;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod collection_former_vec;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod collection_former_vec_deque;

  // = subform collection

  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_collection_playground;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_collection;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_collection_manual;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_collection_implicit;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_collection_setter_off;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_collection_named;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_collection_custom;

  // = subform scalar

  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_scalar_manual;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_scalar;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_scalar_name;

  // = subform entry

  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_entry;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_entry_manual;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_entry_named;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_entry_named_manual;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_entry_setter_off;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_entry_setter_on;

  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_entry_hashmap;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_entry_hashmap_custom;

  // = subform all : scalar, subform_scalar, subform_entry, subform_collection

  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_all;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_all_private;
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  mod subform_all_parametrized;

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

  #[ cfg( feature = "derive_former" ) ]
  #[ test_tools::nightly ]
  #[ test ]
  fn former_trybuild()
  {

    println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
    let t = test_tools::compiletime::TestCases::new();

    t.compile_fail( "tests/inc/former_tests/compiletime/field_attr_bad.rs" );
    t.compile_fail( "tests/inc/former_tests/compiletime/struct_attr_bad.rs" );
    t.pass( "tests/inc/former_tests/compiletime/hashmap_without_parameter.rs" );
    t.pass( "tests/inc/former_tests/compiletime/vector_without_parameter.rs" );

  }

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
