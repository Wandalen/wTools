use super::*;

#[ cfg( any( feature = "former", feature = "meta_former" ) ) ]
mod all
{

  use super::*;
  use super::TheModule::former as TheModule;
  use TheModule as former;
  use wtools::meta::*;

  mod basic_runtime_common;

  #[ path = "../common_front_test.rs" ]
  mod common_front_test;
}
