//! Test `impls_optional`! and `tests_impls_optional`! macros.
//!
//! These macros differ from impls1! and `tests_impls`! by using
//! `#[allow(unused_macros)]` instead of `#[deny(unused_macros)]`,
//! allowing indexed functions to remain unused without compiler errors.

use super :: *;
use the_module ::exposed :: { impls_optional, tests_impls_optional, index };

//

#[ test ]
fn impls_optional_basic()
{
  // test.case( "impls_optional basic usage with all functions used" );
  {
  impls_optional!
{
   fn f1()
   {
  println!( "f1" );
 }
   pub fn f2()
   {
  println!( "f2" );
 }
 };

  f1!();
  f2!();

  f1();
  f2();
 }
}

//

#[ test ]
fn impls_optional_unused()
{
  // test.case( "impls_optional with unused function (should not error)" );
  {
  impls_optional!
{
   fn f1()
   {
  println!( "f1" );
 }
   fn f2()
   {
  println!( "f2" );
 }
 };

  // Only use f1, leave f2 unused - this should NOT cause compile error
  f1!();
  f1();
 }
}

//

#[ test ]
fn impls_optional_with_index()
{
  // test.case( "impls_optional with index! macro" );
  {
  impls_optional!
{
   fn f1()
   {
  println!( "f1" );
 }
   pub fn f2()
   {
  println!( "f2" );
 }
 };

  index! {
   f1,
   f2,
 }

  f1();
  f2();
 }
}

//

#[ test ]
fn impls_optional_partial_index()
{
  // test.case( "impls_optional with partial index usage" );
  {
  impls_optional!
{
   fn f1()
   {
  println!( "f1" );
 }
   fn f2()
   {
  println!( "f2" );
 }
   fn f3()
   {
  println!( "f3" );
 }
 };

  // Only index some functions, leave others unused
  index! {
   f1,
 }

  f1();
 }
}

//

// Define test functions using tests_impls_optional! at module level
// This allows them to be actual #[test] functions that can be discovered
tests_impls_optional! {

  fn optional_test1()
  {
  a_id!( true, true );
 }

  fn optional_test2()
  {
  a_id!( 1 + 1, 2 );
 }

  #[ cfg( all() ) ]
  fn optional_always_test()
  {
  println!( "optional_always_test" );
 }

  #[ cfg( any() ) ]
  fn optional_never_test()
  {
  println!( "optional_never_test" );
 }

}

// Use tests_index! to invoke only SOME of the optional tests
// This demonstrates that unused tests don't cause compile errors
index! {
  optional_test1,
  optional_test2,
  optional_always_test,
  // optional_never_test is not indexed - would fail cfg check anyway
}
