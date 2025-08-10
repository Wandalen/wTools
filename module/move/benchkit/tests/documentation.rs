//! Test documentation functionality

#![allow(clippy::std_instead_of_core)]
#![allow(clippy::writeln_empty_string)]

#[cfg(feature = "integration")]
use benchkit::prelude::*;
#[cfg(feature = "markdown_reports")]
#[allow(unused_imports)]
use benchkit::documentation::*;
use std::io::Write;

type Result<T> = core::result::Result<T, Box<dyn core::error::Error>>;

#[test]
#[cfg(feature = "markdown_reports")]
fn test_documentation_update() -> Result<()>
{
  // Create temporary test file
  let temp_file = std::env::temp_dir().join("test_readme.md");
  let mut file = std::fs::File::create(&temp_file)?;
  writeln!(file, "# Test Project")?;
  writeln!(file)?;
  writeln!(file, "## Performance")?;
  writeln!(file)?;
  writeln!(file, "Old performance data")?;
  writeln!(file)?;
  writeln!(file, "## Other Section")?;
  writeln!(file)?;
  writeln!(file, "This should remain")?;
  drop(file);
  
  // Update the performance section
  let config = DocumentationConfig::readme_performance(&temp_file);
  let updater = DocumentationUpdater::new(config);
  
  let new_content = "| Algorithm | Speed |\n|-----------|-------|\n| Fast | 100 ops/sec |";
  let _diff = updater.update_section(new_content)?;
  
  // Verify update
  let updated_content = std::fs::read_to_string(&temp_file)?;
  assert!(updated_content.contains("Fast | 100 ops/sec"));
  assert!(updated_content.contains("This should remain"));
  
  // Cleanup
  let _ = std::fs::remove_file(temp_file);
  
  Ok(())
}