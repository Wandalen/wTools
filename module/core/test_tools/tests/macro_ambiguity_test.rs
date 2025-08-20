//! Test to document vec! macro ambiguity and resolution patterns
//!
//! This test documents the macro ambiguity that occurs when using `use test_tools::*`
//! and demonstrates the recommended resolution patterns.

#[test]
fn test_qualified_std_vec_usage()
{
  // RECOMMENDED: Use std::vec! explicitly when test_tools is in scope
  let _std_vec = std::vec![ 1, 2, 3 ];
}

#[test]  
fn test_collection_tools_direct_access()
{
  // All collection constructors accessible via collection_tools directly
  let _heap = collection_tools::heap![ 1, 2, 3 ];
  let _vec = collection_tools::vec![ 1, 2, 3 ];
  let _bmap = collection_tools::bmap!{ 1 => "one", 2 => "two" };
  let _hset = collection_tools::hset![ 1, 2, 3 ];
}

#[test]
fn test_aliased_import_pattern()
{
  // RECOMMENDED: Use aliases to avoid ambiguity
  use collection_tools::{vec as cvec, heap};
  
  let _std_vec = std::vec![ 1, 2, 3 ];    // Use std explicitly
  let _collection_vec = cvec![ 1, 2, 3 ]; // Use aliased collection macro
  let _heap = heap![ 1, 2, 3 ];
}

#[test]
fn test_selective_import_pattern()
{
  // RECOMMENDED: Import only what you need instead of `use test_tools::*`
  use test_tools::BTreeMap; // Import specific items
  
  #[allow(clippy::useless_vec)]
  let _std_vec = vec![ 1, 2, 3 ]; // No ambiguity since collection macros not imported
  let _btree: BTreeMap<i32, i32> = BTreeMap::new();
}