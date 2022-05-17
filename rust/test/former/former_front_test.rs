mod basic_runtime
{
  use meta_tools::*;
  mod former
  {
    pub use former_runtime as runtime;
  }
  include!( "./all/basic_runtime_common.rs" );
}

include!( "./common_front_test.rs" );
