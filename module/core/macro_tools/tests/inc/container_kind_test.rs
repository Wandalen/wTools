
use super::*;
use the_module::qt;

//

#[ test ]
fn type_container_kind_basic()
{
  use the_module::exposed::container_kind;

  // test.case( "core::option::Option< i32 >" );
  let code = qt!( core::option::Option< i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = container_kind::of_type( &tree_type );
  a_id!( got, the_module::container_kind::ContainerKind::No );

  // test.case( "core::option::Option< Vec >" );
  let code = qt!( core::option::Option< Vec > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = container_kind::of_type( &tree_type );
  a_id!( got, the_module::container_kind::ContainerKind::No );

  // test.case( "alloc::vec::Vec< i32 >" );
  let code = qt!( alloc::vec::Vec< i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = container_kind::of_type( &tree_type );
  a_id!( got, the_module::container_kind::ContainerKind::Vector );

  // test.case( "alloc::vec::Vec" );
  let code = qt!( alloc::vec::Vec );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = container_kind::of_type( &tree_type );
  a_id!( got, the_module::container_kind::ContainerKind::Vector );

  // test.case( "std::vec::Vec< i32 >" );
  let code = qt!( std::vec::Vec< i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = container_kind::of_type( &tree_type );
  a_id!( got, the_module::container_kind::ContainerKind::Vector );

  // test.case( "std::vec::Vec" );
  let code = qt!( std::vec::Vec );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = container_kind::of_type( &tree_type );
  a_id!( got, the_module::container_kind::ContainerKind::Vector );

  // test.case( "std::Vec< i32 >" );
  let code = qt!( std::Vec< i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = container_kind::of_type( &tree_type );
  a_id!( got, the_module::container_kind::ContainerKind::Vector );

  // test.case( "std::Vec" );
  let code = qt!( std::Vec );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = container_kind::of_type( &tree_type );
  a_id!( got, the_module::container_kind::ContainerKind::Vector );

  // test.case( "not vector" );
  let code = qt!( std::SomeVector< i32, i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = container_kind::of_type( &tree_type );
  a_id!( got, the_module::container_kind::ContainerKind::No );

  // test.case( "hash map" );
  let code = qt!( std::collections::HashMap< i32, i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = container_kind::of_type( &tree_type );
  a_id!( got, the_module::container_kind::ContainerKind::HashMap );

  // test.case( "hash set" );
  let code = qt!( std::collections::HashSet< i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = container_kind::of_type( &tree_type );
  a_id!( got, the_module::container_kind::ContainerKind::HashSet );

}

//

#[ test ]
fn type_optional_container_kind_basic()
{

  // test.case( "non optional not container" );
  let code = qt!( i32 );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = the_module::container_kind::of_optional( &tree_type );
  a_id!( got, ( the_module::container_kind::ContainerKind::No, false ) );

  // test.case( "optional not container" );
  let code = qt!( core::option::Option< i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = the_module::container_kind::of_optional( &tree_type );
  a_id!( got, ( the_module::container_kind::ContainerKind::No, true ) );

  // test.case( "optional not container" );
  let code = qt!( Option< i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = the_module::container_kind::of_optional( &tree_type );
  a_id!( got, ( the_module::container_kind::ContainerKind::No, true ) );


  // test.case( "optional vector" );
  let code = qt!( core::option::Option< Vec > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = the_module::container_kind::of_optional( &tree_type );
  a_id!( got, ( the_module::container_kind::ContainerKind::Vector, true ) );

  // test.case( "optional vector" );
  let code = qt!( Option< Vec > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = the_module::container_kind::of_optional( &tree_type );
  a_id!( got, ( the_module::container_kind::ContainerKind::Vector, true ) );

  // test.case( "non optional vector" );
  let code = qt!( std::Vec< i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = the_module::container_kind::of_optional( &tree_type );
  a_id!( got, ( the_module::container_kind::ContainerKind::Vector, false ) );


  // test.case( "optional vector" );
  let code = qt!( core::option::Option< std::collections::HashMap< i32, i32 > > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = the_module::container_kind::of_optional( &tree_type );
  a_id!( got, ( the_module::container_kind::ContainerKind::HashMap, true ) );

  // test.case( "optional vector" );
  let code = qt!( Option< HashMap > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = the_module::container_kind::of_optional( &tree_type );
  a_id!( got, ( the_module::container_kind::ContainerKind::HashMap, true ) );

  // test.case( "non optional vector" );
  let code = qt!( HashMap< i32, i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = the_module::container_kind::of_optional( &tree_type );
  a_id!( got, ( the_module::container_kind::ContainerKind::HashMap, false ) );


  // test.case( "optional vector" );
  let code = qt!( core::option::Option< std::collections::HashSet< i32, i32 > > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = the_module::container_kind::of_optional( &tree_type );
  a_id!( got, ( the_module::container_kind::ContainerKind::HashSet, true ) );

  // test.case( "optional vector" );
  let code = qt!( Option< HashSet > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = the_module::container_kind::of_optional( &tree_type );
  a_id!( got, ( the_module::container_kind::ContainerKind::HashSet, true ) );

  // test.case( "non optional vector" );
  let code = qt!( HashSet< i32, i32 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = the_module::container_kind::of_optional( &tree_type );
  a_id!( got, ( the_module::container_kind::ContainerKind::HashSet, false ) );

}
