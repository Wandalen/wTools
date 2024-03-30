#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq ) ]
pub struct Struct1
{
  pub int_1 : i32,
  string_1 : String,
  int_optional_1 : core::option::Option< i32 >,
  string_optional_1 : Option< String >,
}

// = formed

// generated by former
impl Struct1
{
  pub fn former() -> Struct1Former
  {
    Struct1Former::new( the_module::ReturnPreformed )
  }
}

// = definition

#[ derive( Debug ) ]
pub struct Struct1FormerDefinition< Context = (), Formed = Struct1, End = former::ReturnPreformed >
// where
//   End : former::FormingEnd< Struct1FormerDefinition< Context, Formed, NoEnd > >,
{
  _phantom : core::marker::PhantomData< ( Context, Formed, End ) >,
}

impl< Context, Formed, End > Default
for Struct1FormerDefinition< Context, Formed, End >
{
  fn default() -> Self
  {
    Self
    {
      _phantom : core::marker::PhantomData,
    }
  }
}

#[ derive( Debug ) ]
pub struct Struct1FormerDefinitionTypes< Context = (), Formed = Struct1 >
{
  _phantom : core::marker::PhantomData< ( Context, Formed ) >,
}

impl< Context, Formed > Default
for Struct1FormerDefinitionTypes< Context, Formed >
{
  fn default() -> Self
  {
    Self
    {
      _phantom : core::marker::PhantomData,
    }
  }
}

impl< Context, Formed > former::FormerDefinitionTypes
for Struct1FormerDefinitionTypes< Context, Formed >
{
  type Storage = Struct1FormerStorage;
  type Formed = Formed;
  type Context = Context;
}

impl< Context, Formed, End > former::FormerDefinition
for Struct1FormerDefinition< Context, Formed, End >
where
  End : former::FormingEnd< Struct1FormerDefinitionTypes< Context, Formed > >,
{
  type Types = Struct1FormerDefinitionTypes< Context, Formed >;
  type End = End;
}

// = storage

// generated by former
pub struct Struct1FormerStorage
{
  pub int_1 : core::option::Option< i32 >,
  pub string_1 : core::option::Option< String >,
  pub int_optional_1 :  core::option::Option< i32 >,
  pub string_optional_1 : core::option::Option< String >,
}

impl Default for Struct1FormerStorage
{

  #[ inline( always ) ]
  fn default() -> Self
  {
    Self
    {
      int_1 : core::option::Option::None,
      string_1 : core::option::Option::None,
      int_optional_1 : core::option::Option::None,
      string_optional_1 : core::option::Option::None,
    }
  }

}

impl former::Storage
for Struct1FormerStorage
{
  type Formed = Struct1;
}

impl former::StoragePerform
for Struct1FormerStorage
{

  fn preform( mut self ) -> < Self as former::Storage >::Formed
  {

    let int_1 = if self.int_1.is_some()
    {
      self.int_1.take().unwrap()
    }
    else
    {
      let val : i32 = Default::default();
      val
    };

    let string_1 = if self.string_1.is_some()
    {
      self.string_1.take().unwrap()
    }
    else
    {
      let val : String = Default::default();
      val
    };

    let int_optional_1 = if self.int_optional_1.is_some()
    {
      Some( self.int_optional_1.take().unwrap() )
    }
    else
    {
      None
    };

    let string_optional_1 = if self.string_optional_1.is_some()
    {
      Some( self.string_optional_1.take().unwrap() )
    }
    else
    {
      None
    };

    // Rust failt to use parameter here
    // < < Self as former::Storage >::Definition::Types as former::FormerDefinitionTypes >::Formed
    Struct1
    {
      int_1,
      string_1,
      int_optional_1,
      string_optional_1,
    }

  }

}

// = former

pub struct Struct1Former
<
  Definition = Struct1FormerDefinition,
>
where
  Definition : former::FormerDefinition,
  < Definition::Types as former::FormerDefinitionTypes >::Storage : former::StoragePerform,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage >,
{
  storage : < Definition::Types as former::FormerDefinitionTypes >::Storage,
  context : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Context >,
  on_end : core::option::Option< Definition::End >,
}

impl< Definition > Struct1Former< Definition >
where
  Definition : former::FormerDefinition,
  < Definition::Types as former::FormerDefinitionTypes >::Storage : former::StoragePerform,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage >,
{

  #[ inline( always ) ]
  pub fn perform(self) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
  {
    let result = self.form();
    return result;
  }

  #[ inline( always ) ]
  pub fn begin
  (
    mut storage : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Storage >,
    context : core::option::Option< < Definition::Types as former::FormerDefinitionTypes >::Context >,
    on_end : < Definition as former::FormerDefinition >::End,
  ) -> Self
  {
    if storage.is_none()
    {
      storage = Some( core::default::Default::default() );
    }
    Self
    {
      storage : storage.unwrap(),
      context,
      on_end : ::core::option::Option::Some( on_end ),
    }
  }

  #[ inline( always ) ]
  pub fn end( mut self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    former::FormingEnd::< Definition::Types >::call( &on_end, self.storage, context )
    // former::FormingEnd::< Struct1FormerDefinition >::call( &on_end, self.storage, context )
  }

  #[ inline( always ) ]
  pub fn form( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
  {
    self.end()
  }

  pub fn int_1< Src >( mut self, src : Src ) -> Self
  where Src : core::convert::Into< i32 >,
  {
    debug_assert!( self.storage.int_1.is_none() );
    self.storage.int_1 = Some( src.into() );
    self
  }

  pub fn string_1< Src >( mut self, src : Src ) -> Self
  where Src : core::convert::Into< String >,
  {
    debug_assert!( self.storage.string_1.is_none() );
    self.storage.string_1 = Some( src.into() );
    self
  }

  pub fn string_optional_1< Src >( mut self, src : Src ) -> Self
  where Src : core::convert::Into< String >
  {
    debug_assert!( self.storage.string_optional_1.is_none() );
    self.storage.string_optional_1 = Some( src.into() );
    self
  }

}

impl< Definition > Struct1Former< Definition >
where
  Definition : former::FormerDefinition,
  < Definition::Types as former::FormerDefinitionTypes >::Storage : former::StoragePerform,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage, Formed = Struct1 >,
{

  pub fn preform( self ) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
  {
    former::StoragePerform::preform( self.storage )
  }

}

impl Struct1Former
{

  #[ inline( always ) ]
  pub fn new( on_end : < Struct1FormerDefinition as former::FormerDefinition >::End ) -> Self
  {
    Self::begin( None, None, on_end )
  }

}

//

include!( "./only_test/primitives.rs" );
