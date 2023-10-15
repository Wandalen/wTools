#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use TheModule::prelude::*;

// qqq : do negative testing /* aaa : Dmytro : done */
// zzz : continue here

tests_impls!
{
  #[ cfg( any( feature = "compiletime_assertions", feature = "diagnostics_compiletime_assertions" ) ) ]
  fn cta_type_same_size_pass()
  {
    struct Int( i16 );
    let got = cta_type_same_size!( Int, i16 );
    assert!( got );
    // cta_type_same_size!( Int, i32 );
  }

  //

  #[ cfg( any( feature = "compiletime_assertions", feature = "diagnostics_compiletime_assertions" ) ) ]
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

  #[ cfg( any( feature = "compiletime_assertions", feature = "diagnostics_compiletime_assertions" ) ) ]
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

  #[ cfg( any( feature = "compiletime_assertions", feature = "diagnostics_compiletime_assertions" ) ) ]
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

#[ cfg( feature = "compiletime_assertions" ) ]
#[ test_tools::rustversion::nightly ]
#[ test ]
fn cta_trybuild_tests()
{
  use test_tools::dependency::trybuild;
  let t = trybuild::TestCases::new();
  t.compile_fail( "tests/test/diagnostics/inc/cta_type_same_size_fail.rs" );
  t.compile_fail( "tests/test/diagnostics/inc/cta_type_same_align_fail.rs" );
  t.compile_fail( "tests/test/diagnostics/inc/cta_ptr_same_size_fail.rs" );
  t.compile_fail( "tests/test/diagnostics/inc/cta_mem_same_size_fail.rs" );
}

#[ cfg( feature = "diagnostics_compiletime_assertions" ) ]
#[ test_tools::rustversion::nightly ]
#[ test ]
fn cta_trybuild_tests()
{
  use test_tools::dependency::trybuild;
  let t = trybuild::TestCases::new();
  t.compile_fail( "tests/test/diagnostics/inc/wtools_cta_type_same_size_fail.rs" );
  t.compile_fail( "tests/test/diagnostics/inc/wtools_cta_type_same_align_fail.rs" );
  t.compile_fail( "tests/test/diagnostics/inc/wtools_cta_ptr_same_size_fail.rs" );
  t.compile_fail( "tests/test/diagnostics/inc/wtools_cta_mem_same_size_fail.rs" );
}

//

tests_index!
{
  cta_type_same_size_pass,
  cta_type_same_align_pass,
  cta_ptr_same_size_pass,
  cta_mem_same_size_pass,
}
