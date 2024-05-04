#[ allow( unused_imports ) ]
use super::*;
// xxx2 : implement

#[ derive( Debug, PartialEq, the_module::Former ) ]
#[ storage_fields( a : i32, b : Option< String > ) ]
// #[ debug ]
// #[ derive( Debug, PartialEq ) ]
pub struct Struct1
{
  c : String,
}

pub struct Struct1CustomEnd< Definition >
{
  _phantom : core::marker::PhantomData< ( Definition, ) >,
}

impl< Definition > Default for Struct1CustomEnd< Definition >
{

  #[ inline( always ) ]
  fn default() -> Self
  {
    Self
    {
      _phantom : core::marker::PhantomData,
    }
  }

}

#[ automatically_derived ]
impl< Context, > former::FormingEnd
<
  Struct1FormerDefinitionTypes< Context, Struct1 >
>
for Struct1CustomEnd< Struct1FormerDefinitionTypes< Context, Struct1 > >
{
  #[ inline( always ) ]
  fn call
  (
    &self,
    storage : Struct1FormerStorage,
    super_former : Option< Context >,
  )
  -> Struct1
  {
    let a = if let Some( a ) = storage.a
    {
      a
    }
    else
    {
      Default::default()
    };
    let b = if let Some( b ) = storage.b
    {
      b
    }
    else
    {
      Default::default()
    };
    Struct1 { c : format!( "{:?} - {:?}", a, b ) }
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
      c : "def".to_string(),
    };
    a_id!( got, exp );
  }
}

tests_index!
{
  test_complex,
}
