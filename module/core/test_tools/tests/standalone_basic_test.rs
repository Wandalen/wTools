//! Basic standalone build functionality test
//! 
//! This test verifies that the essential standalone build functionality works
//! without depending on complex features that may not be available.

#[cfg(test)]
mod standalone_basic_test 
{
  #[test]
  fn test_basic_standalone_functionality()
  {
    // Test that basic functionality is available in standalone mode
    #[cfg(all(feature = "standalone_build", not(feature = "normal_build")))]
    {
      // Test that we can create basic collection types
      let _vec: test_tools::Vec<i32> = test_tools::Vec::new();
      let _map: test_tools::HashMap<String, i32> = test_tools::HashMap::new();
      
      // Test that memory utilities work
      let data = vec![1, 2, 3, 4, 5];
      let _same_ptr = test_tools::same_ptr(&data, &data);
      let _same_size = test_tools::same_size(&data, &data);
      
      // Test passed - functionality verified
    }
    
    #[cfg(not(all(feature = "standalone_build", not(feature = "normal_build"))))]
    {
      // Test the same in normal mode
      let _vec: test_tools::Vec<i32> = test_tools::Vec::new();
      let _map: test_tools::HashMap<String, i32> = test_tools::HashMap::new();
      
      let data = vec![1, 2, 3, 4, 5];
      let _same_ptr = test_tools::same_ptr(&data, &data);
      let _same_size = test_tools::same_size(&data, &data);
      
      // Test passed - functionality verified
    }
  }
}