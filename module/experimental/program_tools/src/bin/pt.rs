//! Short-alias entry point: `pt run [OPTIONS] <TARGET>`.

#[ cfg( not( feature = "enabled" ) ) ]
fn main() {}

#[ cfg( feature = "enabled" ) ]
fn main()
{
  program_tools ::run_cli();
}
