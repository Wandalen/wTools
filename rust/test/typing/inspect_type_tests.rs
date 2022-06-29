#![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]

#[ allow( unused_imports ) ]
use inspect_type as TheModule;
#[ allow( unused_imports ) ]
use test_tools::
{
  tests_impls,
  tests_index,
  a_id,
};

#[ path = "./inc/inspect_type_test.rs" ]
mod inspect_type_test;

