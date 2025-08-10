//! Test enum examples from README to ensure they compile and work correctly

#![ allow( clippy::std_instead_of_core ) ]  // Duration not available in core
//!
//! ## Test Matrix for Enum README Examples
//!
//! | ID   | Test Case                    | Expected Output                     |
//! |------|------------------------------|-------------------------------------|
//! | ER1  | Basic enum assignment        | Status variants assigned correctly  |
//! | ER2  | Enum with different types    | NetworkService works with enums     |
//! | ER3  | Field-specific enum methods  | set/with methods work with enums    |

use component_model::ComponentModel;

use std::time::Duration;

/// Test enum from README example (struct field, not derived)
/// Test Combination: ER1
#[ derive( Debug, PartialEq, Default ) ]
enum Status
{
  #[ default ]
  Pending,
  Processing { progress : f64 },
  Completed { result : String },
  #[ allow( dead_code ) ]
  Failed { error : String },
}

/// Test struct with enum field from README example
/// Test Combination: ER1
#[ derive( Default, Debug, ComponentModel ) ]
struct Task
{
  id : u32,
  status : Status,
  priority : u8,
}


/// Test enum assignment as shown in README
/// Test Combination: ER1
#[ test ]
fn test_basic_enum_assignment_from_readme()
{
  let mut task = Task::default();
  
  // Assign enum variants by type - field-specific methods
  task.id_set( 42u32 );
  task.priority_set( 5u8 );
  task.status_set( Status::Processing { progress: 0.75 } );
  
  assert_eq!( task.id, 42 );
  assert_eq!( task.priority, 5 );
  match task.status {
    #[ allow( clippy::float_cmp ) ]  // Exact comparison needed for test
    Status::Processing { progress } => assert_eq!( progress, 0.75 ),
    _ => panic!( "Expected Processing status" ),
  }
}

/// Test fluent enum assignment as shown in README
/// Test Combination: ER1
#[ test ]
fn test_fluent_enum_assignment_from_readme()
{
  let completed_task = Task::default()
    .id_with( 100u32 )
    .status_with( Status::Completed { result: "Success".to_string() } )
    .priority_with( 1u8 );
    
  assert_eq!( completed_task.id, 100 );
  assert_eq!( completed_task.priority, 1 );
  match completed_task.status {
    Status::Completed { result } => assert_eq!( result, "Success" ),
    _ => panic!( "Expected Completed status" ),
  }
}

/// Test enum from second README example (struct field, not derived)
/// Test Combination: ER2
#[ derive( Debug, Default ) ]
enum ConnectionState
{
  #[ default ]
  Disconnected,
  Connecting { timeout : Duration },
  Connected { session_id : String },
}

/// Test struct with complex enum field from README
/// Test Combination: ER2
#[ derive( Default, Debug, ComponentModel ) ]
struct NetworkService
{
  name : String,
  state : ConnectionState,
  retry_count : u32,
}

/// Test enum with different field types as shown in README
/// Test Combination: ER2 & ER3
#[ test ]
fn test_complex_enum_assignment_from_readme()
{
  let mut service = NetworkService::default();
  
  // Field-specific assignment methods
  service.name_set( "WebSocket".to_string() );
  service.retry_count_set( 3u32 );
  service.state_set( ConnectionState::Connected { 
    session_id: "sess_12345".to_string() 
  } );
  
  assert_eq!( service.name, "WebSocket" );
  assert_eq!( service.retry_count, 3 );
  match service.state {
    ConnectionState::Connected { session_id } => {
      assert_eq!( session_id, "sess_12345" );
    },
    _ => panic!( "Expected Connected state" ),
  }
}

/// Test field-specific methods with enums as shown in README
/// Test Combination: ER3
#[ test ]
fn test_field_specific_enum_methods_from_readme()
{
  let mut service = NetworkService::default();
  
  // Field-specific methods work with enums
  service.name_set( "Updated Service".to_string() );
  service.retry_count_set( 0u32 );
  
  assert_eq!( service.name, "Updated Service" );
  assert_eq!( service.retry_count, 0 );
  
  // Test fluent style too
  let fluent_service = NetworkService::default()
    .name_with( "Fluent Service".to_string() )
    .retry_count_with( 5u32 )
    .state_with( ConnectionState::Connecting { 
      timeout: Duration::from_secs( 30 ) 
    } );
    
  assert_eq!( fluent_service.name, "Fluent Service" );
  assert_eq!( fluent_service.retry_count, 5 );
  match fluent_service.state {
    ConnectionState::Connecting { timeout } => {
      assert_eq!( timeout, Duration::from_secs( 30 ) );
    },
    _ => panic!( "Expected Connecting state" ),
  }
}