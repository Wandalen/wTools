#[ allow( unused_imports ) ]
use super :: *;

use the_module ::
{
  Fields,
  IteratorTrait,
  TableWithFields,
  WithRef,
  OptionalCow,
};

use core ::
{
  hash ::Hasher,
  hash ::Hash,
  cmp ::Ordering,
};
use std ::borrow ::Cow;

use collection_tools ::HashMap;

/// Struct representing a test object with various fields.
#[ derive( Clone, Debug, PartialEq, Eq ) ]
pub struct TestObject
{
  pub id: String,
  pub created_at: i64,
  pub file_ids: Vec< String >,
  pub tools: Option< Vec< HashMap< String, String > > >,
}

impl TableWithFields for TestObject {}

// impl Fields< &'_ str, Option< Cow< '_, str > > >
// for TestObject
// {
//   type Key< 'k > = &'k str;
//   type Val< 'v > = OptionalCow< 'v, str >;
//
//   fn fields( &self ) -> impl IteratorTrait< Item = ( &'_ str, Option< Cow< '_, str > > ) >
//   {
//     use format_tools ::ref_or_display_or_debug_multiline ::field;
//     // use format_tools ::ref_or_display_or_debug ::field;
//     let mut dst: Vec< ( &'_ str, Option< Cow< '_, str > > ) > = Vec ::new();
//
//     dst.push( field!( &self.id ) );
//     dst.push( field!( &self.created_at ) );
//     dst.push( field!( &self.file_ids ) );
//
//     if let Some( tools ) = &self.tools
//     {
//       dst.push( field!( tools ) );
// }
//     else
//     {
//       dst.push( ( "tools", OptionalCow ::none() ) );
// }
//
//     dst.into_iter()
// }
//
// }

impl Fields< &'_ str, Option< Cow< '_, str > > >
for TestObject
{
  type Key< 'k > = &'k str;
  type Val< 'v > = Option< Cow< 'v, str > >;

  fn fields( &self ) -> impl IteratorTrait< Item = ( &'_ str, Option< Cow< '_, str > > ) >
  {
  use format_tools ::ref_or_display_or_debug_multiline ::field;
  // use format_tools ::ref_or_display_or_debug ::field;
  let mut dst: Vec< ( &'_ str, Option< Cow< '_, str > > ) > = Vec ::new();

  // trace_macros!( true );
  dst.push( field!( &self.id ) );
  // trace_macros!( false );
  dst.push( field!( &self.created_at ) );
  dst.push( field!( &self.file_ids ) );

  if let Some( tools ) = &self.tools
  {
   dst.push( field!( tools ) );
 }
  else
  {
   dst.push( ( "tools", Option ::None ) );
 }

  dst.into_iter()
 }

}

impl Hash for TestObject
{

  fn hash< H: Hasher >( &self, state: &mut H )
  {
  self.id.hash( state );
  self.created_at.hash( state );
  self.file_ids.hash( state );

  if let Some( tools ) = &self.tools
  {
   for tool in tools
   {
  for ( key, value ) in tool
  {
   key.hash( state );
   value.hash( state );
 }
 }
 }
  else
  {
   state.write_u8( 0 );
 }
 }

}

// impl PartialEq for TestObject
// {
//
//   fn eq( &self, other: &Self ) -> bool
//   {
//     self.id == other.id &&
//     self.created_at == other.created_at &&
//     self.file_ids == other.file_ids &&
//     self.tools == other.tools
// }
//
// }
//
// impl Eq for TestObject
// {
// }

impl PartialOrd for TestObject
{

  fn partial_cmp( &self, other: &Self ) -> Option< Ordering >
  {
  Some( self.cmp( other ) )
 }

}

impl Ord for TestObject
{

  fn cmp( &self, other: &Self ) -> Ordering
  {
  self.id
  .cmp( &other.id )
  .then_with( | | self.created_at.cmp( &other.created_at ) )
  .then_with( | | self.file_ids.cmp( &other.file_ids ) )
 }

}

//

pub fn test_objects_gen() -> Vec< TestObject >
{

  vec!
  [
  TestObject
  {
   id: "1".to_string(),
   created_at: 1627845583,
   file_ids: vec![ "file1".to_string(), "file2".to_string() ],
   tools: None
 },
  TestObject
  {
   id: "2".to_string(),
   created_at: 13,
   file_ids: vec![ "file3".to_string(), "file4\nmore details".to_string() ],
   tools: Some
   (
  vec!
  [
   {
  let mut map = HashMap ::new();
  map.insert( "tool1".to_string(), "value1".to_string() );
  map
 },
   {
  let mut map = HashMap ::new();
  map.insert( "tool2".to_string(), "value2".to_string() );
  map
 }
 ]
 ),
 },
 ]

}

pub fn test_objects_gen_with_unicode() -> Vec< TestObject >
{
  vec!
  [
  TestObject
  {
   id: "Доміно".to_string(),
   created_at: 100,
   file_ids: vec![ "файл1".to_string(), "файл2".to_string() ],
   tools: None,
 },
  TestObject
  {
   id: "Інший юнікод".to_string(),
   created_at: 120,
   file_ids: vec![],
   tools: Some
   (
  vec!
  [
   {
  let mut map = HashMap ::new();
  map.insert( "тулз1".to_string(), "значення1".to_string() );
  map
 },
   {
  let mut map = HashMap ::new();
  map.insert( "тулз2".to_string(), "значення2".to_string() );
  map
 }
 ]
 ),
 }
 ]
}

pub fn test_objects_gen_2_languages() -> Vec< TestObject >
{
  vec!
  [
  TestObject
  {
   id: "Доміно".to_string(),
   created_at: 100,
   file_ids: vec![ "файл1".to_string(), "файл2".to_string() ],
   tools: Some
   (
  vec!
  [
   {
  let mut map = HashMap ::new();
  map.insert( "тулз1".to_string(), "значення1".to_string() );
  map
 },
   {
  let mut map = HashMap ::new();
  map.insert( "тулз2".to_string(), "значення2".to_string() );
  map
 }
 ]
 ),
 },
  TestObject
  {
   id: "File".to_string(),
   created_at: 120,
   file_ids: vec![ "file1".to_string(), "file2".to_string() ],
   tools: Some
   (
  vec!
  [
   {
  let mut map = HashMap ::new();
  map.insert( "tools1".to_string(), "value1".to_string() );
  map
 },
   {
  let mut map = HashMap ::new();
  map.insert( "tools1".to_string(), "value2".to_string() );
  map
 }
 ]
 ),
 }
 ]
}