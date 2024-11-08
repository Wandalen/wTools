#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn generates_unique_names_on_consecutive_calls()
{
  let name1 = the_module::path::unique_folder_name().unwrap();
  let name2 = the_module::path::unique_folder_name().unwrap();
  assert_ne!( name1, name2 );
}

#[ test ]
fn proper_name()
{
  use regex::Regex;

  let name1 = the_module::path::unique_folder_name().unwrap();
  dbg!( &name1 );

  assert!( !name1.contains( "Thread" ), "{} has bad illegal chars", name1 );
  assert!( !name1.contains( "thread" ), "{} has bad illegal chars", name1 );
  assert!( !name1.contains( "(" ), "{} has bad illegal chars", name1 );
  assert!( !name1.contains( ")" ), "{} has bad illegal chars", name1 );

  // let name1 = "_1232_1313_".to_string();
  let re = Regex::new( r"^[0-9_]*$" ).unwrap();
  assert!( re.is_match( &name1 ), "{} has bad illegal chars", name1 )

  // ThreadId(1)
}

#[ test ]
fn respects_thread_local_counter_increment()
{
  let initial_name = the_module::path::unique_folder_name().unwrap();
  let counter_value_in_initial_name : usize = initial_name
  .split( '_' )
  .last()
  .unwrap()
  .parse()
  .unwrap();

  // Ensuring the next call increments the counter as expected
  let next_name = the_module::path::unique_folder_name().unwrap();
  let counter_value_in_next_name : usize = next_name
  .split( '_' )
  .last()
  .unwrap()
  .parse()
  .unwrap();

  assert_eq!( counter_value_in_next_name, counter_value_in_initial_name + 1 );
}

#[ test ]
fn handles_high_frequency_calls()
{
  let mut names = std::collections::HashSet::new();

  for _ in 0..1000
  {
    let name = the_module::path::unique_folder_name().unwrap();
    assert!( names.insert( name ) );
  }

  assert_eq!( names.len(), 1000 );
}

#[ test ]
fn format_consistency_across_threads()
{
  let mut handles = vec![];

  for _ in 0..10
  {
    let handle = std::thread::spawn( ||
    {
      the_module::path::unique_folder_name().unwrap()
    });
    handles.push( handle );
  }

  let mut format_is_consistent = true;
  let mut previous_format = "".to_string();
  for handle in handles
  {
    let name = handle.join().unwrap();
    let current_format = name.split( '_' ).collect::< Vec< &str > >().len();

    if previous_format != ""
    {
      format_is_consistent = format_is_consistent && ( current_format == previous_format.split( '_' ).collect::< Vec< &str > >().len() );
    }

    previous_format = name;
  }

  assert!( format_is_consistent );
}
