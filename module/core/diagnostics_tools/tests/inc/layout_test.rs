#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use TheModule::prelude::*;

// qqq : do negative testing /* aaa : Dmytro : done */
// zzz : continue here

tests_impls!
{

  #[ cfg( any( feature = "diagnostics_compiletime_assertions", feature = "diagnostics_compiletime_assertions" ) ) ]
  fn cta_type_same_size_pass()
  {
    struct Int( i16 );
    let got = cta_type_same_size!( Int, i16 );
    assert!( got );
    // cta_type_same_size!( Int, i32 );
  }

  //

  #[ cfg( any( feature = "diagnostics_compiletime_assertions", feature = "diagnostics_compiletime_assertions" ) ) ]
  fn cta_type_same_align_pass()
  {
    struct Int1( i16 );
    #[ repr( align( 128 ) )]
    struct Int2( i16 );
    let got = cta_type_same_align!( Int1, i16 );
    assert!( got );
    // cta_type_same_align!( Int1, Int2 );
    // cta_type_same_align!( Int1, i32 );
  }

  #[ cfg( any( feature = "diagnostics_compiletime_assertions", feature = "diagnostics_compiletime_assertions" ) ) ]
  fn cta_ptr_same_size_pass()
  {
    struct Int( i16 );
    let ins1 = Int( 31 );
    let ins2 = 13_i16;
    let got = cta_ptr_same_size!( &ins1, &ins2 );
    assert!( got );
    let got = cta_ptr_same_size!( &ins1, &ins2 );
    assert!( got );
    let got = cta_ptr_same_size!( &ins1, &31_i16 );
    assert!( got );
    // cta_ptr_same_size!( &ins1, &13_i32 );
  }

  #[ cfg( any( feature = "diagnostics_compiletime_assertions", feature = "diagnostics_compiletime_assertions" ) ) ]
  fn cta_mem_same_size_pass()
  {
    struct Int( i16 );
    let ins1 = Int( 31 );
    let ins2 = 13_i16;
    let got = cta_mem_same_size!( ins1, ins2 );
    assert!( got );
    let got = cta_mem_same_size!( ins1, ins2 );
    assert!( got );
    let got = cta_mem_same_size!( ins1, 31_i16 );
    assert!( got );
    // cta_mem_same_size!( ins1, 13_i32 );
  }

}

#[ cfg( any( feature = "diagnostics_compiletime_assertions", feature = "diagnostics_compiletime_assertions" ) ) ]
#[ test_tools::nightly ]
#[ test ]
fn cta_trybuild_tests()
{
  let t = test_tools::compiletime::TestCases::new();
  t.compile_fail( "tests/inc/snipet/cta_type_same_size_fail.rs" );
  t.compile_fail( "tests/inc/snipet/cta_type_same_align_fail.rs" );
  t.compile_fail( "tests/inc/snipet/cta_ptr_same_size_fail.rs" );
  t.compile_fail( "tests/inc/snipet/cta_mem_same_size_fail.rs" );
}

//

tests_index!
{
  cta_type_same_size_pass,
  cta_type_same_align_pass,
  cta_ptr_same_size_pass,
  cta_mem_same_size_pass,
  // cta_trybuild_tests,
}
