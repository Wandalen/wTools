#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
#![allow(dead_code)]

#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use test_tools::a_id;
#[ allow( unused_imports ) ]
use collection_tools::BTreeMap;
use the_module::BTreeMapExt;

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ] -- done
// #[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn add() {
  // expliccit with CollectionFormer

  let got: BTreeMap< String, String > = the_module::CollectionFormer::<
    (String, String),
    former::BTreeMapDefinition<String, String, (), BTreeMap< String, String >, the_module::ReturnStorage>,
  >::new(former::ReturnStorage)
  .add(("a".into(), "x".into()))
  .add(("b".into(), "y".into()))
  .form();
  let exp = collection_tools::bmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!(got, exp);

  // expliccit with BTreeMapFormer

  let got: BTreeMap< String, String > =
    the_module::BTreeMapFormer::<String, String, (), BTreeMap< String, String >, the_module::ReturnStorage>::new(
      former::ReturnStorage,
    )
    .add(("a".into(), "x".into()))
    .add(("b".into(), "y".into()))
    .form();
  let exp = collection_tools::bmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!(got, exp);

  // compact with BTreeMapFormer

  let got: BTreeMap< String, String > = the_module::BTreeMapFormer::new(former::ReturnStorage)
    .add(("a".into(), "x".into()))
    .add(("b".into(), "y".into()))
    .form();
  let exp = collection_tools::bmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!(got, exp);

  // with begin

  let got: BTreeMap< String, String > = the_module::BTreeMapFormer::begin(
    Some(collection_tools::bmap![ "a".to_string() => "x".to_string() ]),
    Some(()),
    former::ReturnStorage,
  )
  .add(("b".into(), "y".into()))
  .form();
  let exp = collection_tools::bmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!(got, exp);

  // with help of ext

  let got: BTreeMap< String, String > = BTreeMap::former()
    .add(("a".into(), "x".into()))
    .add(("b".into(), "y".into()))
    .form();
  let exp = collection_tools::bmap!
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
  let got: BTreeMap< String, String > = the_module::BTreeMapFormer::new(former::ReturnStorage)
    .add(("x".to_string(), "y".to_string()))
    .replace(collection_tools::bmap![ "a".to_string() => "x".to_string(), "b".to_string() => "y".to_string(), ])
    .form();
  let exp = collection_tools::bmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!(got, exp);
}

#[ test ]
fn entity_to() {
  let got = <BTreeMap< i32, i32 > as former::EntityToFormer<
    former::BTreeMapDefinition<i32, i32, (), BTreeMap< i32, i32 >, former::ReturnStorage>,
  >>::Former::new(former::ReturnStorage)
  .add((13, 14))
  .form();
  let exp = collection_tools::bmap![ 13 => 14 ];
  a_id!(got, exp);

  let got = <BTreeMap< i32, i32 > as former::EntityToStorage>::Storage::default();
  let exp = <BTreeMap< i32, i32 > as former::EntityToFormer<
    former::BTreeMapDefinition<i32, i32, (), BTreeMap< i32, i32 >, former::ReturnStorage>,
  >>::Former::new(former::ReturnStorage)
  .form();
  a_id!(got, exp);

  let got = <BTreeMap< i32, i32 > as former::EntityToStorage>::Storage::default();
  let exp = <BTreeMap< i32, i32 > as former::EntityToFormer<
    <BTreeMap< i32, i32 > as former::EntityToDefinition<(), BTreeMap< i32, i32 >, former::ReturnPreformed>>::Definition,
  >>::Former::new(former::ReturnPreformed)
  .form();
  a_id!(got, exp);
}

#[ test ]
fn entry_to_val() {
  let got = former::EntryToVal::<BTreeMap< u32, i32 >>::entry_to_val((1u32, 13i32));
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

  impl former::ValToEntry<BTreeMap< u32, Val >> for Val {
    type Entry = (u32, Val);
    #[ inline( always ) ]
    fn val_to_entry(self) -> Self::Entry {
      (self.key, self)
    }
  }

  let got = former::ValToEntry::<BTreeMap< u32, Val >>::val_to_entry(Val { key: 1u32, data: 13i32 });
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
    #[ subform_collection( definition = former::BTreeMapDefinition ) ]
    children: BTreeMap< u32, Child >,
  }

  let got = Parent::former()
    .children()
    .add((0, Child::former().name("a").form()))
    .add((1, Child::former().name("b").form()))
    .end()
    .form();

  let children = collection_tools::bmap!
  [
    0 => Child { name : "a".to_string(), data : false },
    1 => Child { name : "b".to_string(), data : false },
  ];
  let exp = Parent { children };
  a_id!(got, exp);
}

#[ test ]
fn comprehensive_btree_map_validation() {

  /// Complex child with multiple field types for comprehensive testing
  #[ derive( Debug, Default, PartialEq, the_module::Former ) ]
  pub struct AdvancedChild {
    id: u32,
    name: String,
    metadata: Option< String >,
    active: bool,
    priority: u8,
  }

  /// Parent with multiple BTreeMap collections for advanced validation
  #[ derive( Debug, Default, PartialEq, the_module::Former ) ]
  pub struct MultiBTreeMapParent {
    title: String,
    #[ subform_collection( definition = former::BTreeMapDefinition ) ]
    children: BTreeMap< String, AdvancedChild >,
    #[ subform_collection( definition = former::BTreeMapDefinition ) ]
    metadata: BTreeMap< String, String >,
    #[ subform_collection( definition = former::BTreeMapDefinition ) ]
    numeric_data: BTreeMap< u32, i32 >,
  }

  // Test 1: Complex multi-collection BTreeMap structure
  let complex_result = MultiBTreeMapParent::former()
    .title( "Comprehensive BTreeMap Test".to_string() )
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
  assert_eq!( complex_result.title, "Comprehensive BTreeMap Test" );
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

  // Test 2: BTreeMap ordering preservation (critical BTreeMap feature)
  let ordered_result = MultiBTreeMapParent::former()
    .title( "Ordering Test".to_string() )
    .children()
      .add(( "zebra".to_string(), AdvancedChild::former().id( 26u32 ).form() ))
      .add(( "alpha".to_string(), AdvancedChild::former().id( 1u32 ).form() ))
      .add(( "beta".to_string(), AdvancedChild::former().id( 2u32 ).form() ))
      .add(( "gamma".to_string(), AdvancedChild::former().id( 3u32 ).form() ))
      .end()
    .numeric_data()
      .add(( 300u32, 300i32 ))
      .add(( 100u32, 100i32 ))
      .add(( 200u32, 200i32 ))
      .end()
    .form();

  // Validate BTreeMap maintains sorted order
  let child_keys: Vec< &String > = ordered_result.children.keys().collect();
  assert_eq!( child_keys, vec![ &"alpha".to_string(), &"beta".to_string(), &"gamma".to_string(), &"zebra".to_string() ] );

  let numeric_keys: Vec< &u32 > = ordered_result.numeric_data.keys().collect();
  assert_eq!( numeric_keys, vec![ &100u32, &200u32, &300u32 ] );

  // Test 3: Empty collections edge case
  let empty_result = MultiBTreeMapParent::former()
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

  // Test 4: Direct BTreeMap former construction with type preservation
  let _direct_btree: BTreeMap< String, AdvancedChild > = BTreeMap::former()
    .add(( "direct_test".to_string(), AdvancedChild::former()
      .id( 999u32 )
      .name( "Direct BTree Construction".to_string() )
      .active( true )
      .form() ))
    .form();

  // Test 5: Range operations on BTreeMap (BTreeMap-specific functionality)
  let range_test = MultiBTreeMapParent::former()
    .numeric_data()
      .add(( 10u32, 10i32 ))
      .add(( 20u32, 20i32 ))
      .add(( 30u32, 30i32 ))
      .add(( 40u32, 40i32 ))
      .add(( 50u32, 50i32 ))
      .end()
    .form();

  // Validate BTreeMap range functionality
  let range_values: Vec< (&u32, &i32) > = range_test.numeric_data.range( 20u32..=40u32 ).collect();
  assert_eq!( range_values.len(), 3 ); // Should include 20, 30, 40
  assert_eq!( range_values[0], (&20u32, &20i32) );
  assert_eq!( range_values[1], (&30u32, &30i32) );
  assert_eq!( range_values[2], (&40u32, &40i32) );
}
