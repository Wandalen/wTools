use std ::collections ::HashMap;
use cargo_metadata ::Package;
use serde ::Deserialize;
use willbe ::features ::features_powerset;

/// Constructs a mock `Package` with specified features for testing.
fn mock_package( features : Vec< ( &str, Vec< &str > ) > ) -> Package
{
  let mut features_map : HashMap< String, Vec< _ > > = HashMap ::new();
  for ( feature, deps ) in features
  {
    features_map.insert( feature.to_string(), deps.iter().map( | &dep | dep.to_string() ).collect() );
  }

  let json = serde_json ::json!
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

  Package ::deserialize( json ).unwrap()
}

#[ test ]
fn test_features_powerset()
{
  let package = mock_package
  (
    vec!
    [
      ( "feature1", vec![] ),
      ( "feature2", vec![] ),
      ( "feature3", vec![] ),
    ]
  );

  let power = 2;
  let exclude_features = vec![ "feature1".to_string() ];
  let include_features = vec![ "feature2".to_string() ];

  let result = features_powerset( &package, power, &exclude_features, &include_features );

  assert!( result.contains( &vec![ "feature2".to_string() ].into_iter().collect()) );
  assert!( result.contains( &vec![ "feature2".to_string(), "feature3".to_string() ].into_iter().collect() ) );
  assert_eq!( result.len(), 2 );
}