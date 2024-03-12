use super::*;

#[ cfg( feature = "derive_former" ) ]
mod a_primitives_manual_test;
#[ cfg( feature = "derive_former" ) ]
mod a_containers_without_runtime_manual_test;
#[ cfg( feature = "derive_former" ) ]
mod a_containers_without_runtime_test;
#[ cfg( feature = "derive_former" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
mod a_containers_with_runtime_manual_test;
#[ cfg( feature = "derive_former" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
mod a_containers_with_runtime_test ;

#[ cfg( feature = "derive_former" ) ]
mod attribute_default_container;
#[ cfg( feature = "derive_former" ) ]
mod attribute_default_primitive;
#[ cfg( feature = "derive_former" ) ]
mod former_hashmap_without_parameter;
#[ cfg( feature = "derive_former" ) ]
mod former_vector_without_parameter;

#[ cfg( feature = "derive_former" ) ]
mod string_slice_manual_test;
#[ cfg( feature = "derive_former" ) ]
mod string_slice_test;

#[ cfg( feature = "derive_former" ) ]
mod default_user_type;
#[ cfg( feature = "derive_former" ) ]
mod user_type_no_default;
#[ cfg( feature = "derive_former" ) ]
mod user_type_no_debug;

#[ cfg( feature = "derive_former" ) ]
mod alias_test;
#[ cfg( feature = "derive_former" ) ]
mod name_collisions;
#[ cfg( feature = "derive_former" ) ]
mod name_collision_context;
#[ cfg( feature = "derive_former" ) ]
mod name_collision_end;
#[ cfg( feature = "derive_former" ) ]
mod name_collision_on_end;
#[ cfg( feature = "derive_former" ) ]
mod unsigned_primitive_types;

#[ cfg( feature = "derive_former" ) ]
mod attribute_perform;
#[ cfg( feature = "derive_former" ) ]
mod attribute_setter;

#[ cfg( feature = "derive_former" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
mod parametrized_struct_manual;
#[ cfg( feature = "derive_former" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
mod parametrized_struct_imm;
#[ cfg( feature = "derive_former" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
mod parametrized_struct_where;

#[ cfg( feature = "derive_former" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
mod subformer_basic_manual;
#[ cfg( feature = "derive_former" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
mod subformer_basic;

#[ cfg( feature = "derive_component_from" ) ]
mod components_component_from_manual;
#[ cfg( feature = "derive_component_from" ) ]
mod components_component_from;

#[ cfg( feature = "derive_component_from" ) ]
mod components_set_component_manual;
#[ cfg( feature = "derive_component_from" ) ]
mod components_set_component;

#[ cfg( all( feature = "derive_component_from", feature = "derive_set_component" ) ) ]
mod components_composite_manual;
#[ cfg( all( feature = "derive_component_from", feature = "derive_set_component" ) ) ]
mod components_composite;


only_for_terminal_module!
{

  // stable have different information about error
  // that's why these tests are active only for nightly
  #[ test_tools::nightly ]
  #[ cfg( feature = "derive_former" ) ]
  #[ test ]
  fn former_trybuild()
  {

    println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
    let t = test_tools::compiletime::TestCases::new();

    t.compile_fail( "tests/inc/compiletime/former_bad_attr.rs" );
    t.pass( "tests/inc/compiletime/former_hashmap_without_parameter.rs" );
    t.pass( "tests/inc/compiletime/former_vector_without_parameter.rs" );

  }

}