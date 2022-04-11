
use std::env;
use wtest_basic::dependencies::*;

#[ test ]
#[ rustversion::stable ]
fn trybuild_tests()
{
  println!( "current_dir : {:?}", env::current_dir().unwrap() );
}

#[ test ]
#[ rustversion::nightly ]
fn trybuild_tests()
{
  println!( "current_dir : {:?}", env::current_dir().unwrap() );
  let t = trybuild::TestCases::new();
  t.compile_fail( "../../../rust/test/former/all/bad_attr.rs" );
  t.compile_fail( "../../../rust/test/former/all/vector_without_parameter.rs" );
  t.compile_fail( "../../../rust/test/former/all/hashmap_without_parameter.rs" );
}

/* xxx : use mod_at */

mod basic_runtime { include!( "./all/basic_runtime.rs" ); }
mod basic { include!( "./all/basic.rs" ); }
mod conflict { include!( "./all/conflict.rs" ); }
mod string_slice_runtime { include!( "./all/string_slice_runtime.rs" ); }
mod string_slice { include!( "./all/string_slice.rs" ); }

mod default_user_type { include!( "./all/default_user_type.rs" ); }
mod default_primitive { include!( "./all/default_primitive.rs" ); }
mod default_container { include!( "./all/default_container.rs" ); }
mod after { include!( "./all/perform.rs" ); }
