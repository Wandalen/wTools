
//! Namespace pattern verification tests for `strs_tools`.

#[ test ]
fn verify_namespace_patterns()
{
    use strs_tools::prelude::*;
    
    // Test prelude import works
    let _builder = split();
    
    // Test qualified path access  
    let _qualified_builder = strs_tools::string::split::split();
    
    println!("All namespace patterns work correctly");
}

