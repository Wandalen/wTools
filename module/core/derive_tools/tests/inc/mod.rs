use super::*;

// = import tests of clone_dyn

#[ cfg( feature = "derive_clone_dyn" ) ]
#[ path = "../../../../../module/core/clone_dyn/tests/inc/mod.rs" ]
mod clone_dyn_test;

// = import tests of variadic_from

#[ cfg( any( feature = "derive_variadic_from", feature = "type_variadic_from" ) ) ]
#[ path = "../../../../../module/core/variadic_from/tests/inc/mod.rs" ]
mod variadic_from_test;

// = own tests

mod all_manual_test;
#[ cfg
(
  all
  (
    feature = "derive_as_mut",
    feature = "derive_as_ref",
    feature = "derive_deref",
    feature = "derive_deref_mut",
    feature = "derive_from",
    feature = "derive_inner_from",
    feature = "derive_phantom"
  )
)]
mod all_test;

mod basic_test;

mod as_mut_manual_test;
#[ cfg( feature = "derive_as_mut" ) ]
mod as_mut_test;

mod as_ref_manual_test;
#[ cfg( feature = "derive_as_ref" ) ]
mod as_ref_test;

#[ cfg( feature = "derive_deref" ) ]
#[ path = "deref" ]
mod deref_tests
{
  #[ allow( unused_imports ) ]
  use super::*;

  //

  mod basic_test;
  mod basic_manual_test;

  //

  mod struct_unit;
  mod struct_unit_manual;
  mod struct_tuple;
  mod struct_tuple_manual;
  mod struct_tuple_empty;
  mod struct_tuple_empty_manual;
  mod struct_named;
  mod struct_named_manual;
  mod struct_named_empty;
  mod struct_named_empty_manual;

  mod enum_unit;
  mod enum_unit_manual;
  mod enum_tuple;
  mod enum_tuple_manual;
  mod enum_tuple_empty;
  mod enum_tuple_empty_manual;
  mod enum_named;
  mod enum_named_manual;
  mod enum_named_empty;
  mod enum_named_empty_manual;

  //

  mod generics_lifetimes;
  mod generics_lifetimes_manual;

  mod generics_types;
  mod generics_types_manual;
  mod generics_types_default;
  mod generics_types_default_manual;

  mod generics_constants;
  mod generics_constants_manual;
  mod generics_constants_default;
  mod generics_constants_default_manual;

  //

  mod bounds_inlined;
  mod bounds_inlined_manual;
  mod bounds_where;
  mod bounds_where_manual;
  mod bounds_mixed;
  mod bounds_mixed_manual;

  //

  mod name_collisions;
}

#[ cfg( feature = "derive_deref_mut" ) ]
#[ path = "deref_mut" ]
mod deref_mut_tests
{
  #[ allow( unused_imports ) ]
  use super::*;

  //

  mod basic_test;
  mod basic_manual_test;

  //

  mod struct_tuple;
  mod struct_tuple_manual;
  mod struct_named;
  mod struct_named_manual;

  mod enum_tuple;
  mod enum_tuple_manual;
  mod enum_named;
  mod enum_named_manual;

  //

  mod generics_lifetimes;
  mod generics_lifetimes_manual;

  mod generics_types;
  mod generics_types_manual;
  mod generics_types_default;
  mod generics_types_default_manual;

  mod generics_constants;
  mod generics_constants_manual;
  mod generics_constants_default;
  mod generics_constants_default_manual;

  //

  mod bounds_inlined;
  mod bounds_inlined_manual;
  mod bounds_where;
  mod bounds_where_manual;
  mod bounds_mixed;
  mod bounds_mixed_manual;

  //

  mod name_collisions;
}

#[ cfg( feature = "derive_new" ) ]
#[ path = "new" ]
mod new_tests
{
  #[ allow( unused_imports ) ]
  use super::*;

  // qqq : for each branch add generic test

  //

  mod basic_manual_test;
  mod basic_test;
  mod unit_manual_test;
  mod unit_test;
  mod named_manual_test;
  mod named_test;
  mod multiple_named_manual_test;
  mod multiple_named_test;
  mod multiple_unnamed_manual_test;
  // mod multiple_unnamed_test;
  // xxx : continue

  //

}

#[ cfg( feature = "derive_from" ) ]
#[ path = "from" ]
mod from_tests
{
  #[ allow( unused_imports ) ]
  use super::*;

  // qqq : for each branch add generic test

  //

  mod basic_test;
  mod basic_manual_test;

  //

  mod named_test;
  mod named_manual_test;

  mod multiple_named_manual_test;
  mod multiple_unnamed_manual_test;
  mod unit_manual_test;
  mod multiple_named_test;
  mod unit_test;
  mod multiple_unnamed_test;

  mod variants_manual;
  mod variants_derive;

  mod variants_duplicates_all_off;
  mod variants_duplicates_some_off;
  mod variants_duplicates_some_off_default_off;

  mod variants_generics;
  mod variants_generics_where;
  mod variants_collisions;
}

#[ cfg( feature = "derive_inner_from" ) ]
#[ path = "inner_from" ]
mod inner_from_tests
{
  #[ allow( unused_imports ) ]
  use super::*;

  //

  mod basic_test;
  mod basic_manual_test;

  //

  mod unit_test;
  mod named_manual_test;
  mod multiple_named_manual_test;
  mod unit_manual_test;
  mod named_test;
  mod multiple_named_test;
  mod multiple_unnamed_manual_test;
  mod multiple_unnamed_test;

}

#[ cfg( feature = "derive_phantom" ) ]
#[ path = "phantom" ]
mod phantom_tests
{
  #[ allow( unused_imports ) ]
  use super::*;

  mod struct_named;
  mod struct_named_manual;
  mod struct_named_empty;
  mod struct_named_empty_manual;
  mod struct_tuple;
  mod struct_tuple_manual;
  mod struct_tuple_empty;
  mod struct_tuple_empty_manual;
  mod struct_unit_to_tuple;
  mod struct_unit_to_tuple_manual;
  mod bounds_inlined;
  mod bounds_inlined_manual;
  mod bounds_mixed;
  mod bounds_mixed_manual;
  mod bounds_where;
  mod bounds_where_manual;
  mod name_collisions;
  mod covariant_type;
  mod covariant_type_manual;
  mod contravariant_type;
  mod contravariant_type_manual;
  mod send_sync_type;
  mod send_sync_type_manual;
  #[ test_tools::nightly ]
  #[ test ]
  fn phantom_trybuild()
  {

    println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
    let t = test_tools::compiletime::TestCases::new();

    t.compile_fail( "tests/inc/phantom/compiletime/enum.rs" );
    t.compile_fail( "tests/inc/phantom/compiletime/invariant_type.rs" );
  }
}
