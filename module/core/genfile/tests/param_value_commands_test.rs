//! Integration tests for parameter and value commands
//!
//! Tests parameter management (.parameter.add, .list, .remove) and
//! value management (.value.set, .list, .clear)

mod test_utils;

#[ test ]
fn parameter_add_and_list()
{
  let script = ".archive.new name::test\n\
                .parameter.add name::host mandatory::true description::\"Server hostname\"\n\
                .parameter.add name::port default::\"8080\" description::\"Server port\"\n\
                .parameter.list\n\
                exit";

  let output = test_utils::repl_command( script )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Added parameter: host" ), "Should add host parameter" );
  assert!( stdout.contains( "Added parameter: port" ), "Should add port parameter" );
  assert!( stdout.contains( "Parameters (2):" ), "Should list 2 parameters" );
  assert!( stdout.contains( "host *" ), "Should show mandatory marker for host" );
  assert!( stdout.contains( "port" ), "Should show port parameter" );
}

#[ test ]
fn parameter_add_with_verbosity()
{
  let script = ".archive.new name::test\n\
                .parameter.add name::host mandatory::true default::\"localhost\" description::\"Server host\" verbosity::2\n\
                exit";

  let output = test_utils::repl_command( script )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Added parameter: host" ), "Should add parameter" );
  assert!( stdout.contains( "Mandatory: true" ), "Should show mandatory status" );
  assert!( stdout.contains( "Default: localhost" ), "Should show default value" );
  assert!( stdout.contains( "Description: Server host" ), "Should show description" );
}

#[ test ]
fn parameter_list_with_verbosity()
{
  let script = ".archive.new name::test\n\
                .parameter.add name::host mandatory::true default::\"localhost\"\n\
                .parameter.list verbosity::2\n\
                exit";

  let output = test_utils::repl_command( script )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Parameters (1):" ), "Should show parameter count" );
  assert!( stdout.contains( "Mandatory: true" ), "Should show mandatory detail" );
  assert!( stdout.contains( "Default: localhost" ), "Should show default detail" );
}

#[ test ]
fn parameter_remove()
{
  let script = ".archive.new name::test\n\
                .parameter.add name::host\n\
                .parameter.add name::port\n\
                .parameter.list\n\
                .parameter.remove name::host\n\
                .parameter.list\n\
                exit";
  let output = test_utils::repl_command( script )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Parameters (2):" ), "Should show 2 parameters initially" );
  assert!( stdout.contains( "Removed parameter: host" ), "Should remove host" );
  assert!( stdout.contains( "Parameters (1):" ), "Should show 1 parameter after removal" );
}

#[ test ]
fn parameter_remove_nonexistent()
{
  let script = ".archive.new name::test\n\
                .parameter.remove name::nonexistent\n\
                exit";
  let output = test_utils::repl_command( script )
    .output()
    .expect( "Command should execute" );

  let combined = String::from_utf8_lossy( &output.stdout );
  assert!(
    combined.contains( "not found" ) || combined.contains( "ERROR" ),
    "Should show error for nonexistent parameter"
  );
}

#[ test ]
fn parameter_without_archive_returns_error()
{
  let script = ".parameter.add name::host\nexit";
  let output = test_utils::repl_command( script )
    .output()
    .expect( "Command should execute" );

  let combined = String::from_utf8_lossy( &output.stdout );
  assert!(
    combined.contains( "No archive loaded" ) || combined.contains( "ERROR" ),
    "Should show error when no archive is loaded"
  );
}

#[ test ]
fn value_set_and_list()
{
  let script = ".archive.new name::test\n\
                .value.set name::host value::\"example.com\"\n\
                .value.set name::port value::\"8080\"\n\
                .value.list\n\
                exit";

  let output = test_utils::repl_command( script )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Set value: host = example.com" ), "Should set host value" );
  assert!( stdout.contains( "Set value: port = 8080" ), "Should set port value" );
  assert!( stdout.contains( "Values (2):" ), "Should list 2 values" );
  assert!( stdout.contains( "host" ), "Should show host in list" );
  assert!( stdout.contains( "port" ), "Should show port in list" );
}

#[ test ]
fn value_set_with_verbosity()
{
  let script = ".archive.new name::test\n\
                .value.set name::host value::\"example.com\" verbosity::2\n\
                exit";

  let output = test_utils::repl_command( script )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Set value:" ), "Should show set value message" );
  assert!( stdout.contains( "Parameter: host" ), "Should show parameter name" );
  assert!( stdout.contains( "Value: example.com" ), "Should show value" );
}

#[ test ]
fn value_list_with_verbosity()
{
  let script = ".archive.new name::test\n\
                .value.set name::host value::\"example.com\"\n\
                .value.list verbosity::2\n\
                exit";

  let output = test_utils::repl_command( script )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Values (1):" ), "Should show value count" );
  assert!( stdout.contains( "Value:" ), "Should show value detail label" );
}

#[ test ]
fn value_clear()
{
  let script = ".archive.new name::test\n\
                .value.set name::host value::\"example.com\"\n\
                .value.set name::port value::\"8080\"\n\
                .value.list\n\
                .value.clear\n\
                .value.list\n\
                exit";

  let output = test_utils::repl_command( script )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Values (2):" ), "Should show 2 values initially" );
  assert!( stdout.contains( "Cleared 2 value(s)" ), "Should clear 2 values" );
  assert!( stdout.contains( "No values set" ), "Should show no values after clear" );
}

#[ test ]
fn value_without_archive_returns_error()
{
  let script = ".value.set name::host value::\"test\"\nexit";

  let output = test_utils::repl_command( script )
    .output()
    .expect( "Command should execute" );

  let combined = String::from_utf8_lossy( &output.stdout );
  assert!(
    combined.contains( "No archive loaded" ) || combined.contains( "ERROR" ),
    "Should show error when no archive is loaded"
  );
}

#[ test ]
fn parameters_and_values_integration()
{
  let script = ".archive.new name::test\n\
                .parameter.add name::host mandatory::true\n\
                .parameter.add name::port default::\"8080\"\n\
                .value.set name::host value::\"example.com\"\n\
                .value.set name::port value::\"9000\"\n\
                .parameter.list verbosity::2\n\
                .value.list verbosity::2\n\
                exit";

  let output = test_utils::repl_command( script )
    .output()
    .expect( "Command should execute" );

  let stdout = String::from_utf8_lossy( &output.stdout );
  assert!( stdout.contains( "Parameters (2):" ), "Should list parameters" );
  assert!( stdout.contains( "Values (2):" ), "Should list values" );
  assert!( stdout.contains( "Mandatory: true" ), "Should show parameter metadata" );
  assert!( stdout.contains( "Default: 8080" ), "Should show default value" );
}
