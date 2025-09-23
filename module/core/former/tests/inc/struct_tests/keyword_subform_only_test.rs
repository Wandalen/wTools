#[allow(clippy::used_underscore_binding, clippy::all, warnings)]
// File: module/core/former/tests/inc/former_tests/keyword_subform_only_test.rs
use super::*; // Imports items from keyword_subform_derive.rs

#[ test ]
fn subform_methods_work_with_keywords()
{
  let got = KeywordSubformStruct::former()
    // Test #[ subform_collection ] on r#for
    .r#for() // Expects method named r#for returning VecFormer
      .add( "loop1".to_string() )
      .add( "loop2".to_string() )
    .end() // End VecFormer

    // Test #[ subform_entry ] on r#match
    .r#match() // Expects method named r#match returning SubEntryFormer
      .key( "key1".to_string() ) // Set key via SubEntryFormer
      .value( 10 )
    .end() // End SubEntryFormer, adds ("key1", SubEntry { key: "key1", value: 10 })
    .r#match() // Add another entry
      .key( "key2".to_string() ) // Set key via SubEntryFormer
      .value( 20 )
    .end() // End SubEntryFormer, adds ("key2", SubEntry { key: "key2", value: 20 })

    // Test #[ subform_scalar ] on r#impl
    .r#impl() // Expects method named r#impl returning SubScalarFormer
      .data( true )
    .end() // End SubScalarFormer

    .form(); // Finalize KeywordSubformStruct

  // --- Assertions ---

  // Check r#for field (Vec<String>)
  assert_eq!( got.r#for, vec![ "loop1".to_string(), "loop2".to_string() ] );

  // Check r#match field (HashMap< String, SubEntry >)
  assert_eq!( got.r#match.len(), 2 );
  assert!( got.r#match.contains_key( "key1" ) );
  assert_eq!( got.r#match[ "key1" ].value, 10 );
  assert_eq!( got.r#match[ "key1" ].key, "key1" ); // Verify key was set correctly
  assert!( got.r#match.contains_key( "key2" ) );
  assert_eq!( got.r#match[ "key2" ].value, 20 );
  assert_eq!( got.r#match[ "key2" ].key, "key2" ); // Verify key was set correctly


  // Check r#impl field (SubScalar)
  assert_eq!( got.r#impl, SubScalar { data: true } );
}