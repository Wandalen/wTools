//!
//! # Test Matrix for `Add` and `Sub` Derive
//!
//! This matrix documents test cases for the `Add` and `Sub` derive macro.
//! 
//! | ID   | Struct Type                    | Fields or Input Expression  | Should Compile?  | Should Work at Runtime?| Notes                                |
//! |------|--------------------------------|-----------------------------|------------------|------------------------|--------------------------------------|
//! | T1.1 | Named                          | `{x: i32, y: i32}`          | +                | +                      | Basic case                           |
//! | T1.2 | Tuple                          | `(i32)`                     | +                | +                      | Tuple struct                         |
//! | T1.3 | Unit                           | `()`                        | -                | —                      | Should be rejected                   |
//! | T1.4 | Named with String              | `{x: String}`               | -                | —                      | String doesn't implement `Add<Output = String>` in all cases |
//! | T1.5 | Generic                        | `{x: T}`                    | -                | -                      | Test with bounds                     |
//! | T1.6 | Generic, T: Add/Sub            | `{x: T: Add/Sub }`          | +                | +                      | Test with bounds                     |
//! | T1.7 | Enum, the same variant         | `enum E { One(i32) }`       | +                | +                      | Basic enum case.                     |
//! | T1.8 | Enum, different variants       | `E::One(1) + E::Two(2)`     | +                | +                      | Will return Err(String)              |
//! | T1.9 | Enum, `#[add(error_type = Er)] |
//! |attribute`, different variants         | `E::One(1) + E::Two(2)`     | +                | +                      | Will return Err(Er).                 |
//! | T1.10| Enum, #[add(error_expr = Expr)]| `E::One(1) + E::Two(2)`     | +                | +                      | Will return Err(Expr)                |
//! | T1.11| Enum, different variants
//! | #[derive_ops(error_type = Er)].       | `E::One(1) + E::Two(2)`     | +                | +                      | Will return Err(Er)                  |
//! | T1.12| Enum, different variants
//! | #[derive_ops(error_expr = Expr)].     | `E::One(1) + E::Two(2)`     | +                | +                      | Will return Err(Er)                  |

#![ allow( unused_imports ) ]
#![ allow( dead_code ) ]

use test_tools::prelude::*;
use derive_tools::{ Add, Sub };
use std::cmp::PartialEq;

// T1.1: Named struct
#[ derive( Add, Sub, Clone ) ]
pub struct NamedStruct { x : i32, y : i32 }

// T1.2: Tuple struct
#[ derive( Add, Sub, Clone ) ]
pub struct TupleStruct( i32 );

// // T1.3: Unit struct (should not compile)
#[test]
fn unit_struct_basic_fails() {
    let t = test_tools::compiletime::TestCases::new();
    let mut test_file = std::env::current_dir().unwrap();
    test_file.push(std::path::Path::new("tests/inc/add"));
    test_file.push(std::path::Path::new("compile_fails/unit_struct_basic.rs"));
    t.compile_fail(test_file);
}
// // T1.4: Named struct with String (should not compile)
#[test]
fn string_field_basic_fails() {
    let t = test_tools::compiletime::TestCases::new();
    let mut test_file = std::env::current_dir().unwrap();
    test_file.push(std::path::Path::new("tests/inc/add"));
    test_file.push(std::path::Path::new("compile_fails/string_field_basic.rs"));
    t.compile_fail(test_file);
}

// // T1.5: Generic struct (should not compile)
#[test]
fn generic_struct_basic_fails() {
    let t = test_tools::compiletime::TestCases::new();
    let mut test_file = std::env::current_dir().unwrap();
    test_file.push(std::path::Path::new("tests/inc/add"));
    test_file.push(std::path::Path::new("compile_fails/generic_struct_basic.rs"));
    t.compile_fail(test_file);
}

// T1.6: Generic struct T: Add/Sub
#[ derive( Add, Sub, Clone ) ]
pub struct GenericStruct< T >
where
	T : std::ops::Add< Output = T > + std::ops::Sub< Output = T > + Copy,
{
	x : T,
}

// T1.6 | T1.7: Enum 
#[ derive( Add, Sub, Clone ) ]
pub enum E { One( i32 ), Two }

// T1.9: Enum with #[error(Type)] attribute, returns Error(Type) on different variants

pub type BoxedError = Box< dyn std::error::Error >;
#[ derive( Add, Sub, Clone, PartialEq, Debug )]
#[ sub( error_type = BoxedError ) ]
#[ add( error_type = BoxedError ) ]
pub enum E2 
{
  One( i32 ),
  Two( i32 ),
}

// T1.10
#[ derive( Clone, PartialEq, Debug ) ]
enum ErrorExpr 
{
  DifferentVariants,
  SomeError
}

#[ derive( Add, Sub, Clone, PartialEq, Debug ) ]
#[ sub( error_expr = ErrorExpr::DifferentVariants ) ]
#[ add( error_expr = ErrorExpr::DifferentVariants ) ]
enum E3 
{
  One( i32 ),
  Two( i32 ),  
}


// T1.11
#[ derive( Add, Sub, Clone, PartialEq, Debug ) ]
#[ derive_ops( error_type = BoxedError ) ]
enum E4 
{
  One( i32 ),
  Two( i32 ),
}

// T1.12
#[ derive( Add, Sub, Clone, PartialEq, Debug )]
#[ derive_ops( error_expr = ErrorExpr::DifferentVariants ) ]
enum E5 
{
  One( i32 ),
  Two( i32 ),  
}


include!( "./only_test/basic.rs" );
