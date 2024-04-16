use super::*;

// #[ derive( Debug, PartialEq, former::Former ) ]
// #[ derive( Debug, PartialEq, former::Former ) ] #[ debug ]
#[ derive( Debug, PartialEq ) ]
pub struct Struct1< 'a >
{
  pub string_slice_1 : &'a str,
}

// === begin of generated

#[ automatically_derived ]
impl< 'a > Struct1< 'a >
{
  #[ doc = r"" ]
  #[ doc = r" Make former, variation of builder pattern to form structure defining values of fields step by step." ]
  #[ doc = r"" ] #[ inline( always ) ]
  pub fn former() -> Struct1Former< 'a, >
  {
    Struct1Former::< 'a, >::new( former::ReturnPreformed )
  }
}

#[ derive( Debug ) ]
pub struct Struct1FormerDefinitionTypes< Context, Formed >
{
  _phantom : core::marker::PhantomData< ( Context, Formed ) >,
}

impl< Context, Formed > Default for Struct1FormerDefinitionTypes< Context, Formed >
{
  fn default() -> Self
  {
    Self
    {
      _phantom : core::marker::PhantomData,
    }
  }
}

impl< Context, Formed > former::FormerDefinitionTypes for Struct1FormerDefinitionTypes< Context, Formed >
{
  type Storage = Struct1FormerStorage< 'a >;
  type Formed = Formed;
  type Context = Context;
}

#[ derive( Debug ) ]
pub struct Struct1FormerDefinition< Context, Formed, End >
{
  _phantom : core::marker::PhantomData< ( Context, Formed, End ) >,
}

impl< Context, Formed, End > Default for Struct1FormerDefinition< Context, Formed, End >
{
  fn default() -> Self
  {
    Self
    {
      _phantom : core::marker::PhantomData,
    }
  }
}

impl< Context, Formed, End > former::FormerDefinition for Struct1FormerDefinition< Context, Formed, End >
where End : former::FormingEnd< Struct1FormerDefinitionTypes< Context, Formed > >
{
  type Types = Struct1FormerDefinitionTypes< Context, Formed >;
  type End = End;
}

pub type Struct1FormerWithClosure< Context, Formed > = Struct1FormerDefinition< Context, Formed, former::FormingEndClosure< Struct1FormerDefinitionTypes< Context, Formed > > >;

#[ doc = "Container of a corresponding former." ]
pub struct Struct1FormerStorage< 'a >
{
  #[ doc = r" A field" ]
  pub string_slice_1 : ::core::option::Option< &'a str >,
}

impl< 'a > ::core::default::Default for Struct1FormerStorage< 'a >
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    Self
    {
      string_slice_1 : ::core::option::Option::None,
    }
  }
}

impl< 'a > former::Storage for Struct1FormerStorage< 'a >
{
  type Formed = Struct1< 'a >;
}

impl< 'a > former::StoragePreform for Struct1FormerStorage< 'a >
{
  fn preform( mut self ) -> < Self as former::Storage >::Formed
  {
    let string_slice_1 = self.string_slice_1.take().unwrap_or_else( ||
    {
      panic!( "Field 'string_slice_1' isn't initialized" )
    });
    Struct1::< 'a >
    {
      string_slice_1,
    }
  }
}

// === end of generated

// include!( "./only_test/string_slice.rs" );
// xxx : uncomment
