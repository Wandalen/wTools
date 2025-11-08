use super :: *;
use the_module :: *;
use the_module ::publish_reason ::PublishReason;
use the_module ::stale_dependency ::{ StaleDependency, StaleReason };

//

#[ test ]
fn publish_reason_local_changes()
{
  let reason = PublishReason ::LocalChanges;

  assert!( reason.is_local_change() );
  assert!( !reason.is_stale() );
  assert!( !reason.is_cascade() );

  let desc = reason.description();
  assert!( desc.contains( "Local" ) || desc.contains( "modified" ) );
}

//

#[ test ]
fn publish_reason_version_bump()
{
  let reason = PublishReason ::VersionBump;

  assert!( !reason.is_local_change() );
  assert!( !reason.is_stale() );
  assert!( !reason.is_cascade() );

  let desc = reason.description();
  assert!( desc.contains( "Version" ) || desc.contains( "bump" ) );
}

//

#[ test ]
fn publish_reason_stale_dependencies()
{
  use semver :: { Version, VersionReq };

  let stale_dep = StaleDependency
  {
    name: "former".to_string().into(),
    required: VersionReq ::parse( "~2.36.0" ).unwrap(),
    workspace_version: Version ::parse( "2.37.0" ).unwrap(),
    reason: StaleReason ::IncompatibleVersion,
  };

  let reason = PublishReason ::StaleDependencies
  {
    stale_deps: vec![ stale_dep ],
  };

  assert!( !reason.is_local_change() );
  assert!( reason.is_stale() );
  assert!( !reason.is_cascade() );

  let desc = reason.description();
  assert!( desc.contains( "Stale" ) || desc.contains( "dependencies" ) );
}

//

#[ test ]
fn publish_reason_cascade_effect()
{
  let reason = PublishReason ::CascadeEffect
  {
    triggered_by: vec![ "former".to_string().into() ],
  };

  assert!( !reason.is_local_change() );
  assert!( !reason.is_stale() );
  assert!( reason.is_cascade() );

  let desc = reason.description();
  assert!( desc.contains( "Cascade" ) || desc.contains( "former" ) );
}

//

#[ test ]
fn stale_dependency_incompatible_version()
{
  use semver :: { Version, VersionReq };

  let stale = StaleDependency
  {
    name: "former".to_string().into(),
    required: VersionReq ::parse( "~2.36.0" ).unwrap(),
    workspace_version: Version ::parse( "2.37.0" ).unwrap(),
    reason: StaleReason ::IncompatibleVersion,
  };

  // ~2.36.0 should NOT match 2.37.0
  assert!( !stale.is_compatible() );

  let desc = stale.description();
  assert!( desc.contains( "former" ) );
  assert!( desc.contains( "2.36" ) );
  assert!( desc.contains( "2.37" ) );
}

//

#[ test ]
fn stale_dependency_compatible_version()
{
  use semver :: { Version, VersionReq };

  let stale = StaleDependency
  {
    name: "former".to_string().into(),
    required: VersionReq ::parse( "^2.36.0" ).unwrap(),
    workspace_version: Version ::parse( "2.36.1" ).unwrap(),
    reason: StaleReason ::IncompatibleVersion,
  };

  // ^2.36.0 SHOULD match 2.36.1
  assert!( stale.is_compatible() );
}

//

#[ test ]
fn stale_dependency_being_published()
{
  use semver :: { Version, VersionReq };

  let stale = StaleDependency
  {
    name: "former".to_string().into(),
    required: VersionReq ::parse( "~2.37.0" ).unwrap(),
    workspace_version: Version ::parse( "2.37.0" ).unwrap(),
    reason: StaleReason ::BeingPublished,
  };

  // Version matches but still stale (being published in batch)
  assert!( stale.is_compatible() );

  let desc = stale.description();
  assert!( desc.contains( "being published" ) || desc.contains( "batch" ) );
}

//

#[ test ]
fn stale_reason_types()
{
  let incompatible = StaleReason ::IncompatibleVersion;
  let being_published = StaleReason ::BeingPublished;

  // Just verify they're different
  assert!( std ::mem ::discriminant( &incompatible ) != std ::mem ::discriminant( &being_published ) );
}
