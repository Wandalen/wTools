// | ID   | Struct Type                    | Fields or Input Expression  | Should Compile?  | Should Work at Runtime?| Notes                                |
// |------|--------------------------------|-----------------------------|------------------|------------------------|--------------------------------------|
// | T1.1 | Named                          | `{x: i32, y: i32}`          | +                | +                      | Basic case                           |
// | T1.2 | Tuple                          | `(i32)`                     | +                | +                      | Tuple struct                         |
// | T1.3 | Unit                           | `()`                        | -                | —                      | Should be rejected                   |
// | T1.4 | Named with String              | `{x: String}`               | -                | —                      | String doesn't implement `Add<Output = String>` in all cases |
// | T1.5 | Generic                        | `{x: T}`                    | -                | -                      | Test with bounds                     |
// | T1.6 | Generic, T: Add/Sub            | `{x: T: Add/Sub }`          | +                | +                      | Test with bounds                     |
// | T1.7 | Enum, the same variant         | `enum E { One(i32) }`       | +                | +                      | Basic enum case.                     |
// | T1.8 | Enum, different variants       | `E::One(1) + E::Two(2)`     | +                | +                      | Will return Err(String)              |
// | T1.9 | Enum, `#[add(error_type = Er)] |
// |attribute`, different variants         | `E::One(1) + E::Two(2)`     | +                | +                      | Will return Err(Er).                 |
// | T1.10| Enum, #[add(error_expr = Expr)]| `E::One(1) + E::Two(2)`     | +                | +                      | Will return Err(Expr)                |
// | T1.11| Enum, different variants
// | #[derive_ops(error_type = Er)].       | `E::One(1) + E::Two(2)`     | +                | +                      | Will return Err(Er)                  |
// | T1.12| Enum, different variants
// | #[derive_ops(error_expr = Expr)].     | `E::One(1) + E::Two(2)`     | +                | +                      | Will return Err(Er)                  |

#![ allow( unused_imports ) ]
#![ allow( dead_code ) ]

use test_tools::prelude::*;

use std::cmp::PartialEq;
use std::ops::{ Add, Sub };

// T1.1: Named struct
#[ derive( Clone ) ]
pub struct NamedStruct { x : i32, y : i32 }

impl Add for NamedStruct 
{
  type Output = Self;
  fn add( self, rhs : Self ) -> Self::Output 
  {
    Self 
    {
      x : self.x + rhs.x,
      y : self.y + rhs.y,
    }
  }
}

impl Sub for NamedStruct 
{
  type Output = Self;
  fn sub( self, rhs : Self ) -> Self::Output 
  {
    Self 
    {
      x : self.x - rhs.x,
      y : self.y - rhs.y,
    }
  }
}

// T1.2: Tuple struct
#[ derive( Clone ) ]
pub struct TupleStruct( i32 );

impl Add for TupleStruct 
{
  type Output = Self;
  fn add( self, rhs : Self ) -> Self::Output 
  {
    TupleStruct( self.0 + rhs.0 )
  }
}

impl Sub for TupleStruct 
{
  type Output = Self;
  fn sub( self, rhs : Self ) -> Self::Output 
  {
    TupleStruct( self.0 - rhs.0 )
  }
}

// T1.3: Unit struct (should not compile)
// pub struct UnitStruct;

// impl std::ops::Add for UnitStruct {
//     type Output = Self;
//     fn add(self, _rhs: Self) -> Self::Output {
//         self.0 + _rhs.0
//     }
// }

// impl std::ops::Sub for UnitStruct {
//     type Output = Self;
//     fn sub(self, _rhs: Self) -> Self::Output {
//         self.0 - _rhs.0

//     }
// }

// T1.4: Named struct with String (should not compile)
// pub struct StringStruct { x: String }

// impl std::ops::Add for StringStruct {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self::Output {
//         StringStruct {
//             x: self.x + rhs.x, // will not compile with rhs: String. Could compile with &String or &str
//         }
//     }
// }

// impl std::ops::Sub for StringStruct {
//     type Output = Self;
//     fn sub(self, rhs: Self) -> Self::Output {
//         StringStruct {
//             x: self.x - rhs.x,
//         }
//     }
// }

// T1.5: Generic struct (should not compile)
// pub struct GenericStruct<T> { x: T }

// impl<T> std::ops::Add for GenericStruct<T> {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self::Output {
//         GenericStruct {
//             x: self.x + rhs.x, // Will not compile unless T: Add
//         }
//     }
// }

// impl<T> std::ops::Sub for GenericStruct<T> {
//     type Output = Self;
//     fn sub(self, rhs: Self) -> Self::Output {
//         GenericStruct {
//             x: self.x - rhs.x, // Will not compile unless T: Sub
//         }
//     }
// }

// T1.6: Generic struct T: Add/Sub
#[ derive( Clone ) ]
pub struct GenericStruct< O >
where
  O : Add< Output = O > + Sub< Output = O >,
{
  x : O,
}

impl< O > Add for GenericStruct< O >
where
  O: Add< Output = O > + Sub< Output = O >,
{
  type Output = Self;
  fn add( self, rhs : Self ) -> Self::Output 
  {
    Self 
    {
      x : self.x + rhs.x,
    }
  }
}

impl< O > Sub for GenericStruct< O >
where
  O : Add< Output = O > + Sub< Output = O >,
{
  type Output = Self;
  fn sub( self, rhs: Self ) -> Self::Output 
  {
    Self 
    {
        x : self.x - rhs.x,
    }
  }
}

// T1.7: Enum 
#[ derive( Clone )]
pub enum E { One( i32 ), Two }

impl Add for E 
{
  type Output = Result< Self, String >;
  fn add( self, rhs : Self ) -> Self::Output 
  {
    match ( self, rhs ) 
    {
      ( E::One( a ), E::One( b ) ) => Ok( E::One( a + b ) ),
      _ => { Err( "Cannot add different variants".into() ) }
    }
  }
}

impl Sub for E 
{
  type Output = Result< Self, String >;
  fn sub( self, rhs : Self ) -> Self::Output 
  {
    match ( self, rhs ) 
    {
      ( E::One( a ), E::One( b ) ) => Ok( E::One( a - b ) ),
      _ => { Err( "Cannot sub different variants".into() ) }
    }
  }
}


// T1.9: Enum with #[error(Type)] attribute, returns Error(Type) on different variants
pub type BoxedError = Box< dyn std::error::Error >;

#[ derive( Clone, PartialEq, Debug )]
pub enum E2 
{
  One( i32 ),
  Two( i32 ),
}

impl std::ops::Add for E2 
{
  type Output = Result< Self, BoxedError >;
  fn add( self, rhs: Self ) -> Self::Output 
  {
    match ( self, rhs ) 
    {
      ( E2::One( a ), E2::One( b ) ) => Ok( E2::One( a + b ) ),
      ( E2::Two( a ), E2::Two( b ) ) => Ok( E2::Two( a + b ) ),
      _ => Err( "Different Variants".into() ),
    }
  }
}

impl std::ops::Sub for E2 
{
  type Output = Result< Self, BoxedError >;
  fn sub( self, rhs : Self ) -> Self::Output 
  {
    match ( self, rhs ) 
    {
      ( E2::One( a ), E2::One( b ) ) => Ok( E2::One( a - b ) ),
      ( E2::Two( a ), E2::Two( b ) ) => Ok( E2::Two( a - b ) ),
      _ => Err( "Different Variants".into() ),
    }
  }
}

// T1.10
#[ derive( Clone, PartialEq, Debug ) ]
pub enum E3 
{
  One( i32 ),
  Two( i32 ),
}

#[ derive( Clone, PartialEq, Debug ) ]
pub enum ErrorExpr 
{
  DifferentVariants,
  SomeError,
}

impl std::ops::Add for E3 
{
  type Output = Result< Self, ErrorExpr >;
  fn add( self, rhs : Self ) -> Self::Output 
  {
    match ( self, rhs ) 
    {
      ( E3::One( a ), E3::One( b ) ) => Ok( E3::One( a + b ) ),
      ( E3::Two( a ), E3::Two( b ) ) => Ok( E3::Two( a + b ) ),
      _ => Err( ErrorExpr::DifferentVariants ),
    }
  }
}

impl std::ops::Sub for E3 
{
  type Output = Result< Self, ErrorExpr >;
  fn sub( self, rhs : Self ) -> Self::Output 
  {
    match ( self, rhs ) 
    {
      ( E3::One( a ), E3::One( b ) ) => Ok( E3::One( a - b ) ),
      ( E3::Two( a ), E3::Two( b ) ) => Ok( E3::Two( a - b ) ),
      _ => Err( ErrorExpr::DifferentVariants ),
    }
  }
}

// Manual implementation for E4 (with BoxedError)
#[ derive( Clone, PartialEq, Debug ) ]
enum E4 
{
  One( i32 ),
  Two( i32 ),
}

impl std::ops::Add for E4 
{
  type Output = Result< Self, BoxedError >;
  fn add( self, rhs : Self ) -> Self::Output 
  {
    match ( self, rhs ) 
    {
      ( E4::One( a ), E4::One( b ) ) => Ok( E4::One( a + b ) ),
      ( E4::Two( a ), E4::Two( b ) ) => Ok( E4::Two( a + b ) ),
      _ => Err( "Different Variants".into() ),
    }
  }
}

impl std::ops::Sub for E4 
{
  type Output = Result< Self, BoxedError >;
  fn sub( self, rhs : Self ) -> Self::Output 
  {
    match ( self, rhs ) 
    {
      ( E4::One( a ), E4::One( b ) ) => Ok( E4::One( a - b ) ),
      ( E4::Two( a ), E4::Two( b ) ) => Ok( E4::Two( a - b ) ),
      _ => Err( "Different Variants".into() ),
    }
  }
}

// Manual implementation for E5 (with ErrorExpr)
#[ derive( Clone, PartialEq, Debug ) ]
enum E5 
{
  One( i32 ),
  Two( i32 ),
}

impl std::ops::Add for E5 
{
  type Output = Result< Self, ErrorExpr >;
  fn add( self, rhs : Self ) -> Self::Output 
  {
    match ( self, rhs )
    {
      ( E5::One( a ), E5::One( b ) ) => Ok( E5::One( a + b ) ),
      ( E5::Two( a ), E5::Two( b ) ) => Ok( E5::Two( a + b ) ),
      _ => Err( ErrorExpr::DifferentVariants ),
    }
  }
}

impl std::ops::Sub for E5 
{
  type Output = Result< Self, ErrorExpr >;
  fn sub( self, rhs : Self ) -> Self::Output 
  {
    match ( self, rhs ) 
    {
      ( E5::One( a ), E5::One( b ) ) => Ok( E5::One( a - b ) ),
      ( E5::Two( a ), E5::Two( b ) ) => Ok( E5::Two( a - b ) ),
      _ => Err( ErrorExpr::DifferentVariants ),
    }
  }
}

include!( "./only_test/basic.rs" );