//! Smoke test — verifies the crate compiles and core types construct without panic.

#[ cfg( feature = "enabled" ) ]
#[ test ]
fn smoke_core_types_construct()
{
  let archive = genfile_core::TemplateArchive::new( "smoke" );
  assert_eq!( archive.file_count(), 0 );
  assert!( archive.list_parameters().is_empty() );
}
