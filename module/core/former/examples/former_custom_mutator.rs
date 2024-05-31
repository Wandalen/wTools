// former_custom_mutator.rs

//! This example illustrates how to use the `FormerMutator` trait for implementing custom mutations
//! and demonstrates the concept of storage-specific fields in the forming process.
//!
//! #### Storage-Specific Fields
//!
//! Storage-specific fields are intermediate fields that exist only in the storage structure during
//! the forming process. These fields are not present in the final formed structure but are instrumental
//! in complex forming operations, such as conditional mutations, temporary state tracking, or accumulations.
//!
//! These fields are used to manage intermediate data or state that aids in the construction
//! of the final object but does not necessarily have a direct representation in the object's schema. For
//! instance, counters, flags, or temporary computation results that determine the final state of the object.
//!
//! The `FormerMutator` trait facilitates the implementation of custom mutation logic. It acts on the internal
//! state (context and storage) just before the final forming operation is completed, right before the `FormingEnd`
//! callback is invoked. This trait is crucial for making last-minute adjustments or computations based on the
//! accumulated state in the storage.
//!
//! In this example, the fields `a` and `b` are defined only within the storage and used
//! within the custom mutator to enrich or modify the field `c` of the formed entity. This approach
//! allows for a richer and more flexible formation logic that can adapt based on the intermediate state
//! held within the storage.
//!
//! #### Differences from `FormingEnd`
//!
//! Unlike `FormingEnd`, which is primarily responsible for integrating and finalizing the formation process of a field
//! within a parent former, `form_mutation` directly pertains to the entity itself. This method is designed to be independent
//! of whether the forming process is occurring within the context of a superformer or if the structure is a standalone
//! or nested field. This makes `form_mutation` suitable for entity-specific transformations that should not interfere
//! with the hierarchical forming logic managed by `FormingEnd`.
//!

#[ cfg( any( not( feature = "derive_former" ), not( feature = "enabled" ) ) ) ]
fn main() {}
#[ cfg( all( feature = "derive_former", feature = "enabled" ) ) ]
fn main()
{
  use former::Former;

  #[ derive( Debug, PartialEq, Former ) ]
  #[ storage_fields( a : i32, b : Option< String > ) ]
  #[ mutator( custom ) ]
  pub struct Struct1
  {
    c : String,
  }

  // = former mutator

  impl< Context, Formed > former::FormerMutator
  for Struct1FormerDefinitionTypes< Context, Formed >
  {
    //! Mutates the context and storage of the entity just before the formation process completes.
    #[ inline ]
    fn form_mutation( storage : &mut Self::Storage, _context : &mut ::core::option::Option< Self::Context > )
    {
      storage.a.get_or_insert_with( Default::default );
      storage.b.get_or_insert_with( Default::default );
      storage.c = Some( format!( "{:?} - {}", storage.a.unwrap(), storage.b.as_ref().unwrap() ) );
    }
  }

  let got = Struct1::former().a( 13 ).b( "abc" ).c( "def" ).form();
  let exp = Struct1
  {
    c : "13 - abc".to_string(),
  };
  assert_eq!( got, exp );
  dbg!( got );
  // > got = Struct1 {
  // >  c: "13 - abc",
  // > }

}
