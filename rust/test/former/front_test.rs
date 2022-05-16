
use std::env;
// use test_tools::dependencies::*;

#[ test ]
#[ rustversion::stable ]
// #[ cfg( not( feature = "nightly" ) ) ]
fn trybuild_tests()
{
  println!( "current_dir : {:?}", env::current_dir().unwrap() );
}

// stable have different information about error
// that's why these tests are active only for nightly
#[ test ]
// #[ cfg( feature = "nightly" ) ]
#[ rustversion::nightly ]
fn trybuild_tests()
{
  println!( "current_dir : {:?}", env::current_dir().unwrap() );
  let t = trybuild::TestCases::new();
  t.compile_fail( "../../../rust/test/former/all/bad_attr.rs" );
  t.compile_fail( "../../../rust/test/former/all/vector_without_parameter.rs" );
  t.compile_fail( "../../../rust/test/former/all/hashmap_without_parameter.rs" );
}

/* zzz : use mod_at */

mod basic_runtime
{
  #[cfg( not( feature = "in_wtools" ) )]
  use meta_tools::*;

  #[cfg( feature = "in_wtools" )]
  use wtools::*;
  #[cfg( not( feature = "in_wtools" ) )]
  mod former
  {
    pub use former_runtime as runtime;
  }

  include!( "./all/basic_runtime_common.rs" );
}

/* zzz : introduce file all.rs */

mod basic { include!( "./all/basic.rs" ); }
mod conflict { include!( "./all/conflict.rs" ); }
mod string_slice_runtime { include!( "./all/string_slice_runtime.rs" ); }
mod string_slice { include!( "./all/string_slice.rs" ); }

mod default_user_type { include!( "./all/default_user_type.rs" ); }
mod default_primitive { include!( "./all/default_primitive.rs" ); }
mod default_container { include!( "./all/default_container.rs" ); }
mod after { include!( "./all/perform.rs" ); }
