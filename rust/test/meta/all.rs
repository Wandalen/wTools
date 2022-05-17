
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

mod impls
{
  use super::TheModule as TheModule;
  include!( "./impls_index/mod.rs" );
}

