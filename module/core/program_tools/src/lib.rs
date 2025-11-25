#![ allow( unused_imports, dead_code, missing_docs ) ] // xxx: rid of

#[ cfg( feature = "enabled" ) ]
mod private
{
}

#[ cfg( feature = "enabled" ) ]
mod_interface::mod_interface!
{
  /// Compile and run a Rust program.
  layer program;
}
