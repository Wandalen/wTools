#[ allow( unused_imports ) ]
use super::TheModule;
use test_tools::*;
#[ allow( unused_imports ) ]
use TheModule::prelude::*;

// qqq : do negative testing
// xxx2 : continue here

tests_impls!
{

  #[ cfg( any( feature = "compiletime_assertions", feature = "diagnostics_compiletime_assertions" ) ) ]
  #[ test ]
  fn cta_type_same_size_test()
  {

    struct Int( i16 );
    let got = cta_type_same_size!( Int, i16 );
    assert!( got );
    // cta_type_same_size!( Int, i32 );

  }

  #[ cfg( any( feature = "compiletime_assertions", feature = "diagnostics_compiletime_assertions" ) ) ]
  #[ test ]
  fn cta_type_same_align_test()
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
  #[ test ]
  fn cta_ptr_same_size_test()
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
  #[ test ]
  fn cta_mem_same_size_test()
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

//

tests_index!
{
  cta_type_same_size_test,
  cta_type_same_align_test,
  cta_ptr_same_size_test,
  cta_mem_same_size_test,
}
