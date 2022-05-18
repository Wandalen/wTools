
mod for_each_test
{
  use super::TheModule as TheModule;
  include!( "./all/for_each_test.rs" );
}

mod generator_test
{
  use super::TheModule as TheModule;
  include!( "./all/generator_test.rs" );
}

mod impls_index
{
  use super::TheModule as TheModule;
  include!( "./impls_index/mod.rs" );
}

mod mod_interface
{
  // use super::TheModule as TheModule;
  include!( "./mod_interface/mod.rs" );
}

// mod for_each_test;
// mod generator_test;
// mod impls_index;
// mod mod_interface;
