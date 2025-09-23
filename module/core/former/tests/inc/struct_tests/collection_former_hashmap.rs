#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
#![allow(dead_code)]

#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use test_tools::a_id;
#[ allow( unused_imports ) ]
use collection_tools::HashMap;
use the_module::HashMapExt;

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ] -- done
// #[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn add() {
  // expliccit with CollectionFormer

  let got: HashMap< String, String > = the_module::CollectionFormer::<
    (String, String),
    former::HashMapDefinition<String, String, (), HashMap< String, String >, the_module::ReturnStorage>,
  >::new(former::ReturnStorage)
  .add(("a".into(), "x".into()))
  .add(("b".into(), "y".into()))
  .form();
  let exp = collection_tools::hmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!(got, exp);

  // expliccit with HashMapFormer

  let got: HashMap< String, String > =
    the_module::HashMapFormer::<String, String, (), HashMap< String, String >, the_module::ReturnStorage>::new(
      former::ReturnStorage,
    )
    .add(("a".into(), "x".into()))
    .add(("b".into(), "y".into()))
    .form();
  let exp = collection_tools::hmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!(got, exp);

  // compact with HashMapFormer

  let got: HashMap< String, String > = the_module::HashMapFormer::new(former::ReturnStorage)
    .add(("a".into(), "x".into()))
    .add(("b".into(), "y".into()))
    .form();
  let exp = collection_tools::hmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!(got, exp);

  // with begin

  let got: HashMap< String, String > = the_module::HashMapFormer::begin(
    Some(collection_tools::hmap![ "a".to_string() => "x".to_string() ]),
    Some(()),
    former::ReturnStorage,
  )
  .add(("b".into(), "y".into()))
  .form();
  let exp = collection_tools::hmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!(got, exp);

  // with help of ext

  let got: HashMap< String, String > = HashMap::former()
    .add(("a".into(), "x".into()))
    .add(("b".into(), "y".into()))
    .form();
  let exp = collection_tools::hmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!(got, exp);

  //
}

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ] -- done
// #[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn replace() {
  let got: HashMap< String, String > = the_module::HashMapFormer::new(former::ReturnStorage)
    .add(("x".to_string(), "y".to_string()))
    .replace(collection_tools::hmap![ "a".to_string() => "x".to_string(), "b".to_string() => "y".to_string(), ])
    .form();
  let exp = collection_tools::hmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!(got, exp);
}

#[ test ]
fn entity_to() {
  let got = <HashMap< i32, i32 > as former::EntityToFormer<
    former::HashMapDefinition<i32, i32, (), HashMap< i32, i32 >, former::ReturnStorage>,
  >>::Former::new(former::ReturnStorage)
  .add((13, 14))
  .form();
  let exp = collection_tools::hmap![ 13 => 14 ];
  a_id!(got, exp);

  let got = <HashMap< i32, i32 > as former::EntityToStorage>::Storage::default();
  let exp = <HashMap< i32, i32 > as former::EntityToFormer<
    former::HashMapDefinition<i32, i32, (), HashMap< i32, i32 >, former::ReturnStorage>,
  >>::Former::new(former::ReturnStorage)
  .form();
  a_id!(got, exp);

  let got = <HashMap< i32, i32 > as former::EntityToStorage>::Storage::default();
  let exp = <HashMap< i32, i32 > as former::EntityToFormer<
    <HashMap< i32, i32 > as former::EntityToDefinition<(), HashMap< i32, i32 >, former::ReturnPreformed>>::Definition,
  >>::Former::new(former::ReturnPreformed)
  .form();
  a_id!(got, exp);
}

#[ test ]
fn entry_to_val() {
  let got = former::EntryToVal::<HashMap< u32, i32 >>::entry_to_val((1u32, 13i32));
  let exp = 13i32;
  a_id!(got, exp);
}

#[ test ]
fn val_to_entry() {
  #[ derive( Clone, Copy, Debug, PartialEq ) ]
  struct Val {
    key: u32,
    data: i32,
  }

  impl former::ValToEntry<HashMap< u32, Val >> for Val {
    type Entry = (u32, Val);
    #[ inline( always ) ]
    fn val_to_entry(self) -> Self::Entry {
      (self.key, self)
    }
  }

  let got = former::ValToEntry::<HashMap< u32, Val >>::val_to_entry(Val { key: 1u32, data: 13i32 });
  let exp = (1u32, Val { key: 1u32, data: 13i32 });
  a_id!(got, exp);
}

#[ test ]
fn subformer() {
  /// Parameter description.
  #[ derive( Debug, Default, PartialEq, the_module::Former ) ]
  pub struct Child {
    name: String,
    data: bool,
  }

  /// Parent required for the template.
  #[ derive( Debug, Default, PartialEq, the_module::Former ) ]
  pub struct Parent {
    #[ subform_collection( definition = former::HashMapDefinition ) ]
    children: HashMap< u32, Child >,
  }

  let got = Parent::former()
    .children()
    .add((0, Child::former().name("a").form()))
    .add((1, Child::former().name("b").form()))
    .end()
    .form();

  let children = collection_tools::hmap!
  [
    0 => Child { name : "a".to_string(), data : false },
    1 => Child { name : "b".to_string(), data : false },
  ];
  let exp = Parent { children };
  a_id!(got, exp);
}

#[ test ]
fn comprehensive_hashmap_validation() {

  /// Complex child for comprehensive HashMap testing
  #[ derive( Debug, Default, PartialEq, the_module::Former ) ]
  pub struct AdvancedChild {
    id: u32,
    name: String,
    metadata: Option< String >,
    active: bool,
    priority: u8,
  }

  /// Parent with multiple HashMap collections for validation
  #[ derive( Debug, Default, PartialEq, the_module::Former ) ]
  pub struct MultiHashMapParent {
    title: String,
    #[ subform_collection( definition = former::HashMapDefinition ) ]
    children: HashMap< String, AdvancedChild >,
    #[ subform_collection( definition = former::HashMapDefinition ) ]
    metadata: HashMap< String, String >,
    #[ subform_collection( definition = former::HashMapDefinition ) ]
    numeric_data: HashMap< u32, i32 >,
  }

  // Test 1: Complex multi-collection HashMap structure
  let complex_result = MultiHashMapParent::former()
    .title( "Comprehensive HashMap Test".to_string() )
    .children()
      .add(( "child_alpha".to_string(), AdvancedChild::former()
        .id( 1u32 )
        .name( "Alpha Child".to_string() )
        .metadata( "Priority: High".to_string() )
        .active( true )
        .priority( 9u8 )
        .form() ))
      .add(( "child_beta".to_string(), AdvancedChild::former()
        .id( 2u32 )
        .name( "Beta Child".to_string() )
        .active( false )
        .priority( 5u8 )
        .form() ))
      .end()
    .metadata()
      .add(( "version".to_string(), "3.0".to_string() ))
      .add(( "environment".to_string(), "comprehensive_test".to_string() ))
      .add(( "feature_flags".to_string(), "all_enabled".to_string() ))
      .end()
    .numeric_data()
      .add(( 100u32, 200i32 ))
      .add(( 50u32, 150i32 ))
      .add(( 75u32, 300i32 ))
      .end()
    .form();

  // Comprehensive validation
  assert_eq!( complex_result.title, "Comprehensive HashMap Test" );
  assert_eq!( complex_result.children.len(), 2 );
  assert_eq!( complex_result.metadata.len(), 3 );
  assert_eq!( complex_result.numeric_data.len(), 3 );

  // Validate specific child data
  let alpha_child = complex_result.children.get( "child_alpha" ).unwrap();
  assert_eq!( alpha_child.id, 1u32 );
  assert_eq!( alpha_child.name, "Alpha Child" );
  assert_eq!( alpha_child.metadata, Some( "Priority: High".to_string() ) );
  assert_eq!( alpha_child.active, true );
  assert_eq!( alpha_child.priority, 9u8 );

  let beta_child = complex_result.children.get( "child_beta" ).unwrap();
  assert_eq!( beta_child.id, 2u32 );
  assert_eq!( beta_child.name, "Beta Child" );
  assert_eq!( beta_child.metadata, None );
  assert_eq!( beta_child.active, false );
  assert_eq!( beta_child.priority, 5u8 );

  // Test 2: Empty collections edge case
  let empty_result = MultiHashMapParent::former()
    .title( "Empty Collections Test".to_string() )
    .children()
      .end()
    .metadata()
      .end()
    .numeric_data()
      .end()
    .form();

  assert_eq!( empty_result.title, "Empty Collections Test" );
  assert_eq!( empty_result.children.len(), 0 );
  assert_eq!( empty_result.metadata.len(), 0 );
  assert_eq!( empty_result.numeric_data.len(), 0 );

  // Test 3: Direct HashMap former construction with type preservation
  let _direct_hashmap: HashMap< String, AdvancedChild > = HashMap::former()
    .add(( "direct_test".to_string(), AdvancedChild::former()
      .id( 999u32 )
      .name( "Direct HashMap Construction".to_string() )
      .active( true )
      .form() ))
    .form();

  // Test 4: Large collection handling
  let large_test = MultiHashMapParent::former()
    .numeric_data()
      .add(( 1u32, 10i32 ))
      .add(( 2u32, 20i32 ))
      .add(( 3u32, 30i32 ))
      .add(( 4u32, 40i32 ))
      .add(( 5u32, 50i32 ))
      .add(( 6u32, 60i32 ))
      .add(( 7u32, 70i32 ))
      .add(( 8u32, 80i32 ))
      .add(( 9u32, 90i32 ))
      .add(( 10u32, 100i32 ))
      .end()
    .form();

  assert_eq!( large_test.numeric_data.len(), 10 );
  assert_eq!( large_test.numeric_data.get( &5u32 ), Some( &50i32 ) );
  assert_eq!( large_test.numeric_data.get( &10u32 ), Some( &100i32 ) );
}
