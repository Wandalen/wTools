//! Tests for `MarkdownUpdateChain` functionality

#![ allow( clippy ::std_instead_of_core ) ]
#![ allow( clippy ::uninlined_format_args ) ]
#![ allow( clippy ::needless_raw_string_hashes ) ]
#![ allow( clippy ::doc_markdown ) ]

#[ cfg( feature = "integration" ) ]
#[ cfg( feature = "markdown_reports" ) ]
mod tests
{
  use benchkit ::prelude :: *;
  use std ::fs;
  use std ::path ::PathBuf;

  fn create_test_file( content: &str ) -> PathBuf
  {
  let temp_dir = std ::env ::temp_dir();
  let file_path = temp_dir.join( format!( "benchkit_test_{}.md", uuid ::Uuid ::new_v4() ) );
  fs ::write( &file_path, content ).unwrap();
  file_path
 }

  fn cleanup_test_file( path: &PathBuf )
  {
  let _ = fs ::remove_file( path );
  let backup_path = path.with_extension( "bak" );
  let _ = fs ::remove_file( backup_path );
 }

  #[ test ]
  fn test_empty_chain_fails()
  {
  let temp_file = create_test_file( "" );
  
  let chain = MarkdownUpdateChain ::new( &temp_file ).unwrap();
  let result = chain.execute();
  
  assert!( result.is_err() );
  cleanup_test_file( &temp_file );
 }

  #[ test ]
  fn test_single_section_update()
  {
  let initial_content = r#"# Test Document

## Existing Section

Old content here.

## Another Section

More content."#;

  let temp_file = create_test_file( initial_content );
  
  let chain = MarkdownUpdateChain ::new( &temp_file ).unwrap()
   .add_section( "Performance Results", "New benchmark data!" );
  
  chain.execute().unwrap();
  
  let updated_content = fs ::read_to_string( &temp_file ).unwrap();
  assert!( updated_content.contains( "## Performance Results" ) );
  assert!( updated_content.contains( "New benchmark data!" ) );
  assert!( updated_content.contains( "## Existing Section" ) );
  assert!( updated_content.contains( "## Another Section" ) );
  
  cleanup_test_file( &temp_file );
 }

  #[ test ]
  fn test_multiple_section_atomic_update()
  {
  let initial_content = r#"# Test Document

## Introduction

Welcome to the test.

## Conclusion

That's all folks!"#;

  let temp_file = create_test_file( initial_content );
  
  let chain = MarkdownUpdateChain ::new( &temp_file ).unwrap()
   .add_section( "Performance Results", "Fast operations measured" )
   .add_section( "Memory Analysis", "Low memory usage detected" )
   .add_section( "CPU Usage", "Efficient CPU utilization" );
  
  chain.execute().unwrap();
  
  let updated_content = fs ::read_to_string( &temp_file ).unwrap();
  
  // Check all new sections were added
  assert!( updated_content.contains( "## Performance Results" ) );
  assert!( updated_content.contains( "Fast operations measured" ) );
  assert!( updated_content.contains( "## Memory Analysis" ) );
  assert!( updated_content.contains( "Low memory usage detected" ) );
  assert!( updated_content.contains( "## CPU Usage" ) );
  assert!( updated_content.contains( "Efficient CPU utilization" ) );
  
  // Check original sections preserved
  assert!( updated_content.contains( "## Introduction" ) );
  assert!( updated_content.contains( "Welcome to the test." ) );
  assert!( updated_content.contains( "## Conclusion" ) );
  assert!( updated_content.contains( "That's all folks!" ) );
  
  cleanup_test_file( &temp_file );
 }

  #[ test ]
  fn test_conflict_detection()
  {
  let initial_content = r#"# Test Document

## Performance Analysis

Existing performance data.

## Performance Results

Different performance data."#;

  let temp_file = create_test_file( initial_content );
  
  let chain = MarkdownUpdateChain ::new( &temp_file ).unwrap()
   .add_section( "Performance", "This will conflict!" );
  
  let conflicts = chain.check_all_conflicts().unwrap();
  assert!( !conflicts.is_empty() );
  
  // Execution should fail due to conflicts
  let result = chain.execute();
  assert!( result.is_err() );
  
  cleanup_test_file( &temp_file );
 }

  #[ test ]
  fn test_backup_and_restore_on_failure()
  {
  let initial_content = r#"# Test Document

## Performance Analysis

Important data that must be preserved."#;

  let temp_file = create_test_file( initial_content );
  
  // Create chain that will fail due to conflicts
  let chain = MarkdownUpdateChain ::new( &temp_file ).unwrap()
   .add_section( "Performance", "Conflicting section name" );
  
  // Execution should fail
  let result = chain.execute();
  assert!( result.is_err() );
  
  // Original content should be preserved
  let final_content = fs ::read_to_string( &temp_file ).unwrap();
  assert_eq!( final_content, initial_content );
  
  cleanup_test_file( &temp_file );
 }

  #[ test ]
  fn test_section_replacement()
  {
  let initial_content = r#"# Test Document

## Performance Results

Old benchmark data.
With multiple lines.

## Other Section

Unrelated content."#;

  let temp_file = create_test_file( initial_content );
  
  let chain = MarkdownUpdateChain ::new( &temp_file ).unwrap()
   .add_section( "Performance Results", "Updated benchmark data!" );
  
  chain.execute().unwrap();
  
  let updated_content = fs ::read_to_string( &temp_file ).unwrap();
  
  // New content should be there
  assert!( updated_content.contains( "Updated benchmark data!" ) );
  
  // Old content should be gone
  assert!( !updated_content.contains( "Old benchmark data." ) );
  assert!( !updated_content.contains( "With multiple lines." ) );
  
  // Unrelated content should be preserved
  assert!( updated_content.contains( "## Other Section" ) );
  assert!( updated_content.contains( "Unrelated content." ) );
  
  cleanup_test_file( &temp_file );
 }

  #[ test ]
  fn test_new_file_creation()
  {
  let temp_dir = std ::env ::temp_dir();
  let file_path = temp_dir.join( format!( "benchkit_new_{}.md", uuid ::Uuid ::new_v4() ) );
  
  // File doesn't exist yet
  assert!( !file_path.exists() );
  
  let chain = MarkdownUpdateChain ::new( &file_path ).unwrap()
   .add_section( "Results", "First section content" )
   .add_section( "Analysis", "Second section content" );
  
  chain.execute().unwrap();
  
  // File should now exist
  assert!( file_path.exists() );
  
  let content = fs ::read_to_string( &file_path ).unwrap();
  assert!( content.contains( "## Results" ) );
  assert!( content.contains( "First section content" ) );
  assert!( content.contains( "## Analysis" ) );
  assert!( content.contains( "Second section content" ) );
  
  cleanup_test_file( &file_path );
 }

  #[ test ]
  fn test_chain_properties()
  {
  let temp_file = create_test_file( "" );
  
  let chain = MarkdownUpdateChain ::new( &temp_file ).unwrap()
   .add_section( "Section1", "Content1" )
   .add_section( "Section2", "Content2" );
  
  assert_eq!( chain.len(), 2 );
  assert!( !chain.is_empty() );
  assert_eq!( chain.file_path(), temp_file.as_path() );
  assert_eq!( chain.updates().len(), 2 );
  assert_eq!( chain.updates()[ 0 ].section_name, "Section1" );
  assert_eq!( chain.updates()[ 1 ].content, "Content2" );
  
  cleanup_test_file( &temp_file );
 }
}