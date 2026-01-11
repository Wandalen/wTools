//! Basic Four-Layer Namespace Generation Test
//!
//! Validates that the `mod_interface` macro correctly generates all four
//! namespace layers (own, orphan, exposed, prelude) and that content
//! placed in each layer propagates correctly through the hierarchy.

pub mod test_module
{
  mod private {}

  mod_interface_meta ::mod_interface!
  {
    own mod mod_own;
    orphan mod mod_orphan;
    exposed mod mod_exposed;
    prelude mod mod_prelude;
  }
}

#[ test ]
fn all_four_layers_exist()
{
  // Verify all four namespace layers were generated
  // Direct access tests (module is re-exported at root level)
  assert!( test_module ::mod_own ::has_own() );
  assert!( test_module ::mod_orphan ::has_orphan() );
  assert!( test_module ::mod_exposed ::has_exposed() );
  assert!( test_module ::mod_prelude ::has_prelude() );
}

#[ test ]
fn own_layer_has_access_to_all_content()
{
  // own layer should re-export everything: orphan + exposed + prelude
  assert!( test_module ::own ::mod_own ::has_own() );
  assert!( test_module ::own ::mod_orphan ::has_orphan() );
  assert!( test_module ::own ::mod_exposed ::has_exposed() );
  assert!( test_module ::own ::mod_prelude ::has_prelude() );
}

#[ test ]
fn orphan_layer_propagates_correctly()
{
  // orphan layer should have: orphan + exposed + prelude (but not own)
  assert!( test_module ::orphan ::mod_orphan ::has_orphan() );
  assert!( test_module ::orphan ::mod_exposed ::has_exposed() );
  assert!( test_module ::orphan ::mod_prelude ::has_prelude() );
}

#[ test ]
fn exposed_layer_propagates_correctly()
{
  // exposed layer should have: exposed + prelude (but not own/orphan)
  assert!( test_module ::exposed ::mod_exposed ::has_exposed() );
  assert!( test_module ::exposed ::mod_prelude ::has_prelude() );
}

#[ test ]
fn prelude_layer_isolated()
{
  // prelude layer should only have prelude content
  assert!( test_module ::prelude ::mod_prelude ::has_prelude() );
}
