// File: module/core/former/tests/inc/former_tests/keyword_subform_derive.rs
use super::*;
use collection_tools::{Vec, HashMap}; // Use standard collections

// Inner struct for subform_entry test
#[ derive( Debug, Default, PartialEq, Clone, former::Former ) ]
pub struct SubEntry {
  key: String, // Key will be set by ValToEntry
  value: i32,
}

// Implement ValToEntry to map SubEntry to HashMap key/value
impl former::ValToEntry<HashMap< String, SubEntry >> for SubEntry {
  type Entry = (String, SubEntry);
  #[ inline( always ) ]
  fn val_to_entry(self) -> Self::Entry {
    (self.key.clone(), self)
  }
}

// Inner struct for subform_scalar test
#[ derive( Debug, Default, PartialEq, Clone, former::Former ) ]
pub struct SubScalar {
  data: bool,
}

// Parent struct with keyword fields using subform attributes
#[ derive( Debug, Default, PartialEq, former::Former ) ]
// #[ debug ] // Uncomment to see generated code
pub struct KeywordSubformStruct {
  #[ subform_collection ] // Default definition is VectorDefinition
  r#for: Vec<String>,

  #[ subform_entry ] // Default definition is HashMapDefinition
  r#match: HashMap< String, SubEntry >,

  #[ subform_scalar ]
  r#impl: SubScalar,
}

// Include the test logic file (which we'll create next)
include!("keyword_subform_only_test.rs");

// qqq : xxx : fix it
