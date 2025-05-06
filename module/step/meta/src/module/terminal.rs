#[ macro_export ]
macro_rules! only_for_terminal_module
{
( $( $Any : tt )* ) =>
  {
    #[ cfg( feature = "derive_former" ) ]
    #[ test_tools::nightly ]
    #[ test ]
    fn former_trybuild()
    {

      println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
      let t = test_tools::compiletime::TestCases::new();

      t.compile_fail( "tests/inc/former_struct_tests/compiletime/field_attr_bad.rs" );
      t.compile_fail( "tests/inc/former_struct_tests/compiletime/struct_attr_bad.rs" );
      t.pass( "tests/inc/former_struct_tests/compiletime/hashmap_without_parameter.rs" );
      t.pass( "tests/inc/former_struct_tests/compiletime/vector_without_parameter.rs" );
      t.compile_fail( "tests/inc/former_enum_tests/compile_fail/unit_subform_scalar_error.rs" ); // Added the new test case

      // assert!( false );

    }

    // stable have different information about error
    // that's why these tests are active only for nightly
    #[ cfg( feature = "derive_former" ) ]
    #[ test_tools::nightly ]
    #[ test ]
    fn components_trybuild()
    {

      println!( "current_dir : {:?}", std::env::current_dir().unwrap() );
      let _t = test_tools::compiletime::TestCases::new();

      // zzz : make it working test
      //t.run( "tests/inc/components_tests/compiletime/components_component_from_debug.rs" );

    }
  };
}

/// Mechanism to include tests only to aggregating crate.
/// It exclude code in terminal module ( crate ), but include for aggregating module ( crate ).
#[ macro_export ]
macro_rules! only_for_aggregating_module
{
  ( $( $Any : tt )* ) =>
  {
  }
}
