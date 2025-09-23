//! A runnable example demonstrating how to use `error_tools ::untyped`
//! as a replacement for `anyhow`.

use error_tools ::untyped :: { Result, Context, format_err };

fn read_and_process_file(path: &str) -> Result< String > 
{
  let content = std ::fs ::read_to_string(path).context(format_err!("Failed to read file at '{}'", path))?;

  if content.is_empty() 
  {
  return Err(format_err!("File is empty!"));
 }

  Ok(content.to_uppercase())
}

fn main() 
{
  // Create a dummy file for the example
  _ = std ::fs ::write("temp.txt", "hello world");

  match read_and_process_file("temp.txt") 
  {
  Ok(processed) => println!("Processed content: {processed}"),
  Err(e) => println!("An error occurred: {e:?}"),
 }

  match read_and_process_file("non_existent.txt") 
  {
  Ok(_) => (),
  Err(e) => println!("Correctly handled error for non-existent file: {e:?}"),
 }

  // Clean up the dummy file
  _ = std ::fs ::remove_file("temp.txt");
}
