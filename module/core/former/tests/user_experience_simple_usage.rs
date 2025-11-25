#![ allow( dead_code, missing_docs ) ]

//! # User Experience Test: Recommended Simple Usage Pattern
//!
//! This test documents and verifies the RECOMMENDED way to use the Former crate.
//!
//! ## ✅ Recommended Pattern (WORKS)
//!
//! ### Step 1: Add dependency
//! ```toml
//! [dependencies]
//! former = { version = "2.31.0", features = ["default"] }
//! # OR within workspace:
//! former = { workspace = true }
//! ```
//!
//! ### Step 2: Import
//! ```rust
//! use former::Former;
//! ```
//!
//! ### Step 3: Use
//! ```rust
//! #[derive(Former)]
//! struct MyStruct {
//!     field: String,
//! }
//! ```
//!
//! ## ❌ Over-Complicated Pattern (UNNECESSARY)
//!
//! ### DON'T do this:
//! ```toml
//! [dependencies]
//! former = { workspace = true }
//! former_meta = { workspace = true }      # ❌ NOT NEEDED!
//! former_types = { workspace = true }     # ❌ NOT NEEDED!
//!
//! [features]
//! enabled = [
//!   "former/enabled",
//!   "former_meta/enabled",              # ❌ NOT NEEDED!
//!   "former_meta/derive_former",        # ❌ NOT NEEDED!
//!   "former_types/enabled",             # ❌ NOT NEEDED!
//! ]
//! ```
//!
//! ### DON'T do this:
//! ```rust,ignore
//! use former_meta::Former;  // ❌ Bypasses helpful re-exports
//! use former_types::*;      // ❌ Unnecessary explicit import
//! ```
//!
//! ## Why the Simple Pattern Works
//!
//! The `former` crate already re-exports everything you need:
//! - `Former` derive macro (from `former_meta`)
//! - All necessary traits (from `former_types`)
//! - Helper types and functions
//!
//! See `/home/user1/pro/lib/wTools/module/core/former/src/lib.rs:237-241`

// ✅ CORRECT: Single import gives you everything
use former::Former;

// Test 1: Basic struct
#[ derive( Debug, PartialEq, Former ) ]
pub struct UserConfig
{
  username : String,
  email : String,
  age : u32,
}

#[ test ]
fn test_basic_former()
{
  let user = UserConfig::former()
    .username( "alice".to_string() )
    .email( "alice@example.com".to_string() )
    .age( 30_u32 )
    .form();

  assert_eq!( user.username, "alice" );
  assert_eq!( user.email, "alice@example.com" );
  assert_eq!( user.age, 30 );
}

// Test 2: With Option fields
#[ derive( Debug, PartialEq, Former ) ]
pub struct ServerConfig
{
  host : String,
  port : u16,
  description : Option< String >,
  max_connections : Option< u32 >,
}

#[ test ]
fn test_with_options()
{
  let server = ServerConfig::former()
    .host( "localhost".to_string() )
    .port( 8080_u16 )
    .description( "Development server".to_string() )
    .form();

  assert_eq!( server.host, "localhost" );
  assert_eq!( server.port, 8080 );
  assert_eq!( server.description, Some( "Development server".to_string() ) );
  assert_eq!( server.max_connections, None );
}

// Test 3: Nested structs with subform
#[ derive( Debug, PartialEq, Former ) ]
pub struct Address
{
  street : String,
  city : String,
  zipcode : String,
}

#[ derive( Debug, PartialEq, Former ) ]
pub struct Person
{
  name : String,
  #[ subform_scalar ]
  address : Address,
}

#[ test ]
fn test_subform()
{
  let person = Person::former()
    .name( "Bob".to_string() )
    .address()
      .street( "123 Main St".to_string() )
      .city( "Springfield".to_string() )
      .zipcode( "12345".to_string() )
      .form()
    .form();

  assert_eq!( person.name, "Bob" );
  assert_eq!( person.address.street, "123 Main St" );
  assert_eq!( person.address.city, "Springfield" );
}

// Test 4: Collection fields
use collection_tools::Hmap;

#[ derive( Debug, Former ) ]
pub struct AppConfig
{
  name : String,
  #[ subform_collection( definition = former::VectorDefinition ) ]
  tags : Vec< String >,
  #[ subform_collection( definition = former::HashMapDefinition ) ]
  metadata : Hmap< String, String >,
}

#[ test ]
fn test_collections()
{
  let config = AppConfig::former()
    .name( "MyApp".to_string() )
    .tags()
      .add( "web".to_string() )
      .add( "api".to_string() )
      .end()
    .metadata()
      .add( ( "version".to_string(), "1.0.0".to_string() ) )
      .add( ( "author".to_string(), "Alice".to_string() ) )
      .end()
    .form();

  assert_eq!( config.name, "MyApp" );
  assert_eq!( config.tags, vec![ "web", "api" ] );
  assert_eq!( config.metadata.get( "version" ), Some( &"1.0.0".to_string() ) );
}
