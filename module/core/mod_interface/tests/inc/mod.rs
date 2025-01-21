#[ allow( unused_imports ) ]
use super::*;

mod manual
{

  use super::*;

  mod micro_modules;
  mod micro_modules_two;
  mod layer;
  mod use_layer;

}

mod derive
{

  use super::*;

  // micro module
  mod micro_modules;
  mod micro_modules_two;
  mod micro_modules_two_joined;
  mod micro_modules_glob;

  // layer
  mod layer;
  mod layer_have_layer;
  mod layer_have_layer_separate_use;
  mod layer_have_layer_separate_use_two;
  mod layer_have_layer_cfg;
  mod layer_have_mod_cfg;
  mod layer_use_cfg;
  mod layer_use_macro;

  // use
  mod use_layer;
  mod use_basic;
  mod use_private_layers;
  #[ path = "./use_as/derive.rs" ]
  mod use_as_derive;
  #[ path = "./use_as/manual.rs" ]
  mod use_as_manual;

  // reuse
  mod reuse_basic;

  // attr
  mod attr_debug;

}

mod trybuild_test;

// xxx : enable