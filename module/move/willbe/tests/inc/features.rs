use super::*;

use TheModule::*;
use TheModule::features::features_powerset;

use std::collections::HashMap;
use cargo_metadata::Package;
use serde::Deserialize;

/// Constructs a mock `Package` with specified features for testing.
fn mock_package( features : Vec< ( &str, Vec< &str > ) > ) -> Package
{
  let mut features_map : HashMap< String, Vec< _ > > = HashMap::new();
  for ( feature, deps ) in features
  {
    features_map.insert( feature.to_string(), deps.iter().map( | &dep | dep.to_string() ).collect() );
  }

  let json = serde_json::json!
  (
    {
    "name" : "mock_package",
    "version" : "0.1.0",
    "id" : "mock_package 0.1.0",
    "dependencies" : [],
    "targets" : [],
    "features" : features_map,
    "manifest_path" : "".to_string(),
    "authors" : [],
    "categories" : [],
    "keywords" : [],
    "edition" : "2018",
    }
  );

  Package::deserialize( json ).unwrap()
}

#[ test ]
fn case_1()
{
  let package = mock_package
  (
    vec!
    [
      ( "f1", vec![] ),
      ( "f2", vec![] ),
      ( "f3", vec![] ),
    ]
  );

  let power = 1;

  let exclude_features = vec![];
  let include_features = vec![];
  let enabled_features = vec![ "f2".to_string() ];
  let result = features_powerset
  (
    &package,
    power,
    &exclude_features,
    &include_features,
    &enabled_features,
    false,
    false
  );
  dbg!(&result);

  assert!( result.contains( &vec![ "f1".to_string(), "f2".to_string() ].into_iter().collect()) );
  assert!( result.contains( &vec![ "f2".to_string() ].into_iter().collect()) );
  assert!( result.contains( &vec![ "f2".to_string(), "f3".to_string() ].into_iter().collect()) );
  assert_eq!( result.len(), 3 );
}

#[ test ]
fn case_2()
{
  let package = mock_package
    (
      vec!
      [
        ( "f1", vec![] ),
        ( "f2", vec![] ),
        ( "f3", vec![] ),
      ]
    );

  let power = 2;
  let exclude_features = vec![];
  let include_features = vec![];
  let enabled_features = vec![ "f2".to_string() ];
  let result = features_powerset
    (
      &package,
      power,
      &exclude_features,
      &include_features,
      &enabled_features,
      false,
      false
    );
  dbg!(&result);

  assert!( result.contains( &vec![ "f2".to_string() ].into_iter().collect()) );
  assert!( result.contains( &vec![ "f1".to_string(), "f2".to_string() ].into_iter().collect()) );
  assert!( result.contains( &vec![ "f1".to_string(), "f2".to_string(), "f3".to_string() ].into_iter().collect()) );
  assert!( result.contains( &vec![ "f2".to_string(), "f3".to_string() ].into_iter().collect()) );
  assert_eq!( result.len(), 4 );
}

#[ test ]
fn case_3()
{
  let package = mock_package
    (
      vec!
      [
        ( "f1", vec![] ),
        ( "f2", vec![] ),
        ( "f3", vec![] ),
      ]
    );

  let power = 1;
  let exclude_features = vec![];
  let include_features = vec![];
  let enabled_features = vec![ "f2".to_string() ];
  let result = features_powerset
    (
      &package,
      power,
      &exclude_features,
      &include_features,
      &enabled_features,
      false,
      true
    );
  dbg!(&result);

  assert!( result.contains( &vec![].into_iter().collect()) );
  assert_eq!( result.len(), 1 );
}

#[ test ]
fn case_4()
{
  let package = mock_package
    (
      vec!
      [
        ( "f1", vec![] ),
        ( "f2", vec![] ),
        ( "f3", vec![] ),
      ]
    );

  let power = 1;
  let exclude_features = vec![];
  let include_features = vec![];
  let enabled_features = vec![ "f2".to_string() ];
  let result = features_powerset
    (
      &package,
      power,
      &exclude_features,
      &include_features,
      &enabled_features,
      true,
      false
    );
  dbg!(&result);

  assert!( result.contains( &vec![ "f1".to_string(), "f2".to_string(), "f3".to_string(), ].into_iter().collect()) );
  assert_eq!( result.len(), 1 );
}

#[ test ]
fn case_5()
{
  let package = mock_package
    (
      vec!
      [
        ( "f1", vec![] ),
        ( "f2", vec![] ),
        ( "f3", vec![] ),
      ]
    );

  let power = 1;
  let exclude_features = vec![];
  let include_features = vec![ "f1".to_string() ];
  let enabled_features = vec![ "f2".to_string() ];
  let result = features_powerset
    (
      &package,
      power,
      &exclude_features,
      &include_features,
      &enabled_features,
      false,
      false
    );
  dbg!(&result);

  assert!( result.contains( &vec![ "f2".to_string() ].into_iter().collect()) );
  assert!( result.contains( &vec![ "f1".to_string(), "f2".to_string() ].into_iter().collect()) );
  assert_eq!( result.len(), 2 );
}

#[ test ]
fn case_6()
{
  let package = mock_package
    (
      vec!
      [
        ( "f1", vec![] ),
        ( "f2", vec![] ),
        ( "f3", vec![] ),
      ]
    );

  let power = 1;
  let exclude_features = vec![ "f3".to_string() ];
  let include_features = vec![];
  let enabled_features = vec![ "f2".to_string() ];
  let result = features_powerset
    (
      &package,
      power,
      &exclude_features,
      &include_features,
      &enabled_features,
      false,
      false
    );
  dbg!(&result);

  assert!( result.contains( &vec![ "f1".to_string(), "f2".to_string() ].into_iter().collect()) );
  assert!( result.contains( &vec![ "f2".to_string() ].into_iter().collect()) );

  assert_eq!( result.len(), 2 );
}