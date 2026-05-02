use super :: *;
use the_module ::tool ::semver_utils :: { version_satisfies, version_incompatible };
use semver :: { Version, VersionReq };

#[ test ]
fn version_satisfies_within_caret_range()
{
  let req = VersionReq ::parse( "^2.36.0" ).unwrap();
  let v1 = Version ::parse( "2.36.1" ).unwrap();
  let v2 = Version ::parse( "2.37.0" ).unwrap();

  assert!( version_satisfies( &req, &v1 ) );
  assert!( version_satisfies( &req, &v2 ) );
}

#[ test ]
fn version_satisfies_below_range()
{
  let req = VersionReq ::parse( "^2.36.0" ).unwrap();
  let v = Version ::parse( "2.35.9" ).unwrap();

  assert!( !version_satisfies( &req, &v ) );
  assert!( version_incompatible( &req, &v ) );
}

#[ test ]
fn version_satisfies_exact_match()
{
  let req = VersionReq ::parse( "=1.0.0" ).unwrap();
  let v_match = Version ::parse( "1.0.0" ).unwrap();
  let v_no_match = Version ::parse( "1.0.1" ).unwrap();

  assert!( version_satisfies( &req, &v_match ) );
  assert!( !version_satisfies( &req, &v_no_match ) );
}

#[ test ]
fn version_incompatible_is_inverse_of_satisfies()
{
  let req = VersionReq ::parse( ">=1.2.0, <2.0.0" ).unwrap();
  let versions =
  [
    Version ::parse( "1.1.9" ).unwrap(),
    Version ::parse( "1.2.0" ).unwrap(),
    Version ::parse( "1.9.9" ).unwrap(),
    Version ::parse( "2.0.0" ).unwrap(),
  ];

  for v in &versions
  {
    assert_eq!( version_satisfies( &req, v ), !version_incompatible( &req, v ) );
  }
}
