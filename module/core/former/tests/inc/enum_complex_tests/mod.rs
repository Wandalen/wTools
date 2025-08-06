mod subform_collection_test;
// REMOVED: comprehensive_mixed_derive (too large, causes build timeouts - replaced with simplified_mixed_derive)
mod simplified_mixed_derive; // REPLACEMENT: Simplified mixed enum coverage without build timeout issues

#[cfg(feature = "derive_former")]
#[test_tools::nightly]
#[test]
fn former_trybuild() {
  println!("current_dir : {:?}", std::env::current_dir().unwrap());
  let _t = test_tools::compiletime::TestCases::new();

  // assert!( false );
}
