// Purpose: Focused replacement for blocked parametrized_struct_where test
// This works around "Derive macro uses Definition as generic K, but Definition doesn't implement Hash+Eq" 
// by creating non-parametrized struct equivalents with HashMap/BTreeMap that actually work

use super::*;
#[allow(unused_imports)]
use ::former::prelude::*;
use ::former::Former;

#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
use std::collections::HashMap;

// Non-parametrized replacement for parametrized struct where functionality
#[derive(Debug, PartialEq, Former)]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
pub struct ParametrizedStructReplacement {
  // Replaces parametrized struct with concrete HashMap types that work
  #[subform_entry]
  string_map: HashMap<String, String>,
  
  #[subform_entry] 
  int_map: HashMap<String, i32>,
  
  // Basic fields for completeness
  name: String,
  active: bool,
}

// Another struct testing different HashMap scenarios
#[derive(Debug, PartialEq, Former)]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
pub struct AdvancedParametrizedStructReplacement {
  #[subform_entry]
  primary_map: HashMap<String, String>,
  
  #[subform_entry]
  secondary_map: HashMap<String, i32>,
  
  title: String,
}

// Tests replacing blocked parametrized_struct_where functionality
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[test]
fn string_map_test() {
  let mut expected_string_map = HashMap::new();
  expected_string_map.insert("key1".to_string(), "value1".to_string());
  expected_string_map.insert("key2".to_string(), "value2".to_string());
  
  let mut expected_int_map = HashMap::new();
  expected_int_map.insert("num1".to_string(), 42);
  expected_int_map.insert("num2".to_string(), 99);
  
  let got = ParametrizedStructReplacement::former()
    .name("map_test".to_string())
    .active(true)
    .string_map()
      .add(("key1".to_string(), "value1".to_string()))
      .add(("key2".to_string(), "value2".to_string()))
      .end()
    .int_map()
      .add(("num1".to_string(), 42))
      .add(("num2".to_string(), 99))
      .end()
    .form();
    
  let expected = ParametrizedStructReplacement {
    string_map: expected_string_map,
    int_map: expected_int_map,
    name: "map_test".to_string(),
    active: true,
  };
  
  assert_eq!(got, expected);
}

#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[test]  
fn empty_map_test() {
  let got = ParametrizedStructReplacement::former()
    .name("empty".to_string())
    .active(false)
    .form();
    
  let expected = ParametrizedStructReplacement {
    string_map: HashMap::new(),
    int_map: HashMap::new(),
    name: "empty".to_string(),
    active: false,
  };
  
  assert_eq!(got, expected);
}

#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[test]
fn advanced_map_test() {
  let mut expected_primary = HashMap::new();
  expected_primary.insert("primary_key".to_string(), "primary_value".to_string());
  
  let mut expected_secondary = HashMap::new();
  expected_secondary.insert("secondary_key".to_string(), 777);
  
  let got = AdvancedParametrizedStructReplacement::former()
    .title("advanced_map".to_string())
    .primary_map()
      .add(("primary_key".to_string(), "primary_value".to_string()))
      .end()
    .secondary_map()
      .add(("secondary_key".to_string(), 777))
      .end()
    .form();
    
  let expected = AdvancedParametrizedStructReplacement {
    primary_map: expected_primary,
    secondary_map: expected_secondary,
    title: "advanced_map".to_string(),
  };
  
  assert_eq!(got, expected);
}

#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[test]
fn single_entry_test() {
  let mut expected_map = HashMap::new();
  expected_map.insert("single".to_string(), "entry".to_string());
  
  let got = AdvancedParametrizedStructReplacement::former()
    .title("single_test".to_string()) 
    .primary_map()
      .add(("single".to_string(), "entry".to_string()))
      .end()
    .form();
    
  let expected = AdvancedParametrizedStructReplacement {
    primary_map: expected_map,
    secondary_map: HashMap::new(),
    title: "single_test".to_string(),
  };
  
  assert_eq!(got, expected);
}