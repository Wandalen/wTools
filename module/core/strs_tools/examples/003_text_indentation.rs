//! Text indentation and formatting examples.
//!
//! This example demonstrates how to use `strs_tools` for consistent text formatting,
//! code generation, and document processing tasks that require precise control
//! over line-by-line formatting.

use strs_tools::*;

fn main()
{
  println!( "=== Text Indentation Examples ===" );
  
  basic_indentation();
  code_generation_example();
  nested_structure_formatting();
  custom_line_processing();
}

/// Demonstrates basic text indentation functionality.
///
/// Shows how to add consistent indentation to multi-line text,
/// which is essential for code generation and document formatting.
fn basic_indentation()
{
  println!( "\n--- Basic Text Indentation ---" );
  
  #[ cfg( all( feature = "string_indentation", not( feature = "no_std" ) ) ) ]
  {
    let original_text = "First line\nSecond line\nThird line";
    
    println!( "Original text:" );
    println!( "{original_text}" );
    
    // Add 2-space indentation to each line
    let indented = string::indentation::indentation( "  ", original_text, "" );
    
    println!( "\nWith 2-space indentation:" );
    println!( "{indented}" );
    
    // Verify each line is properly indented
    let lines : Vec< &str > = indented.lines().collect();
    for line in &lines
    {
      assert!( line.starts_with( "  " ), "Line should start with 2 spaces: '{line}'" );
    }
    
    println!( "✓ All lines properly indented" );
  }
}

/// Demonstrates code generation use case.
///
/// Shows how to format generated code with proper indentation
/// levels for different nesting levels.
fn code_generation_example()
{
  println!( "\n--- Code Generation Example ---" );
  
  #[ cfg( all( feature = "string_indentation", not( feature = "no_std" ) ) ) ]
  {
    // Simulate generating a Rust function with nested blocks
    let mut generated_code = String::new();
    
    // Function signature (no indentation)
    generated_code.push_str( "fn example_function()" );
    generated_code.push( '\n' );
    generated_code.push( '{' );
    generated_code.push( '\n' );
    
    // Function body content (will be indented)
    let function_body = "let x = 42;\nlet y = x * 2;\nif y > 50 {\n    println!(\"Large value: {}\", y);\n}";
    
    // Add 2-space indentation for function body
    let indented_body = string::indentation::indentation( "  ", function_body, "" );
    generated_code.push_str( &indented_body );
    
    generated_code.push( '\n' );
    generated_code.push( '}' );
    
    println!( "Generated Rust code:" );
    println!( "{generated_code}" );
    
    // Verify the structure looks correct
    let lines : Vec< &str > = generated_code.lines().collect();
    assert!( lines[ 0 ].starts_with( "fn " ) );
    assert!( lines[ 2 ].starts_with( "  let x" ) );  // Body indented
    assert!( lines[ 4 ].starts_with( "  if " ) );    // Condition indented
    
    println!( "✓ Code properly structured with indentation" );
  }
}

/// Demonstrates nested structure formatting.
///
/// Shows how to create documents with multiple indentation levels,
/// useful for configuration files, documentation, or data serialization.
fn nested_structure_formatting()
{
  println!( "\n--- Nested Structure Formatting ---" );
  
  #[ cfg( all( feature = "string_indentation", not( feature = "no_std" ) ) ) ]
  {
    // Create a hierarchical document structure
    let mut document = String::new();
    
    // Top level - no indentation
    document.push_str( "Configuration:\n" );
    
    // Level 1 - single indentation
    let level1_content = "database:\nlogging:\nserver:";
    let level1_indented = string::indentation::indentation( "  ", level1_content, "" );
    document.push_str( &level1_indented );
    document.push( '\n' );
    
    // Level 2 - double indentation for database config
    let db_config = "host: localhost\nport: 5432\nname: myapp_db";
    let db_indented = string::indentation::indentation( "    ", db_config, "" );
    
    // Insert database config after the database line
    let lines : Vec< &str > = document.lines().collect();
    let mut final_doc = String::new();
    
    for line in lines.iter()
    {
      final_doc.push_str( line );
      final_doc.push( '\n' );
      
      // Add detailed config after "database:" line
      if line.trim() == "database:"
      {
        final_doc.push_str( &db_indented );
        final_doc.push( '\n' );
      }
    }
    
    println!( "Nested configuration document:" );
    println!( "{final_doc}" );
    
    // Verify indentation levels are correct
    let final_lines : Vec< &str > = final_doc.lines().collect();
    
    // Check that database settings have 4-space indentation
    let host_line = final_lines.iter().find( | line | line.contains( "host:" ) ).unwrap();
    assert!( host_line.starts_with( "    " ), "Database config should have 4-space indent" );
    
    println!( "✓ Nested structure properly formatted" );
  }
}

/// Demonstrates custom line processing with prefix and postfix.
///
/// Shows advanced formatting options including line prefixes and suffixes,
/// useful for creating comments, documentation, or special formatting.
fn custom_line_processing()
{
  println!( "\n--- Custom Line Processing ---" );
  
  #[ cfg( all( feature = "string_indentation", not( feature = "no_std" ) ) ) ]
  {
    let documentation = "This is a function that processes data.\nIt takes input and returns output.\nUsed in data processing pipelines.";
    
    println!( "Original documentation:" );
    println!( "{documentation}" );
    
    // Convert to Rust documentation comments
    let rust_docs = string::indentation::indentation( "/// ", documentation, "" );
    
    println!( "\nAs Rust documentation:" );
    println!( "{rust_docs}" );
    
    // Convert to C-style block comments
    let c_comments = string::indentation::indentation( " * ", documentation, "" );
    let c_block = format!( "/*\n{c_comments}\n */" );
    
    println!( "\nAs C-style block comment:" );
    println!( "{c_block}" );
    
    // Create a boxed comment
    let boxed_content = string::indentation::indentation( "│ ", documentation, " │" );
    let boxed_comment = format!( "┌─{}─┐\n{}\n└─{}─┘", 
                                "─".repeat( 50 ), 
                                boxed_content,
                                "─".repeat( 50 ) );
    
    println!( "\nAs boxed comment:" );
    println!( "{boxed_comment}" );
    
    // Verify the formatting
    let doc_lines : Vec< &str > = rust_docs.lines().collect();
    for line in doc_lines
    {
      assert!( line.starts_with( "/// " ), "Rust doc line should start with '/// '" );
    }
    
    println!( "✓ Custom line processing formats applied successfully" );
  }
}