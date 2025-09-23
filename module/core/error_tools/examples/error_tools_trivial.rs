//! A trivial example for `error_tools`.

fn get_message() -> &'static str 
{
  "Hello, world!"
  // This could return an error in a more complex example
}

fn main() 
{
  let msg = get_message();
  println!("Success: {msg}");
}
