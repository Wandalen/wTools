#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq, the_module::Former ) ]
#[ storage_fields( a : i32, b : Option< String > ) ]
#[ mutator( custom ) ]
// #[ debug ]
// #[ derive( Debug, PartialEq ) ]
pub struct Struct1
{
  c : String,
}

// = former mutator

impl< Context, Formed > former::FormerMutator
for Struct1FormerDefinitionTypes< Context, Formed >
{
  /// Mutates the context and storage of the entity just before the formation process completes.
  #[ inline ]
  fn form_mutation( storage : &mut Self::Storage, _context : &mut ::core::option::Option< Self::Context > )
  {
    storage.a.get_or_insert_with( Default::default );
    storage.b.get_or_insert_with( Default::default );
    storage.c = Some( format!( "{:?} - {}", storage.a.unwrap(), storage.b.as_ref().unwrap() ) );
  }
}

// == begin of generated

// == end of generated

tests_impls!
{

  fn test_complex()
  {
    let got = Struct1::former().a( 13 ).b( "abc" ).c( "def" ).form();
    let exp = Struct1
    {
      c : "13 - abc".to_string(),
    };
    a_id!( got, exp );
  }

}

tests_index!
{
  test_complex,
}
