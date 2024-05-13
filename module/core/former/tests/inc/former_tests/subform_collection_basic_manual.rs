#[ allow( unused_imports ) ]
use super::*;

#[ derive( Default, Debug, PartialEq ) ]
pub struct Struct1
{
  vec_1 : Vec< String >,
  hashmap_1 : collection_tools::HashMap< String, String >,
  hashset_1 : collection_tools::HashSet< String >,
}

// == begin of generated

#[automatically_derived]
impl< > Struct1< >
where
{

  #[ inline( always ) ]
  pub fn former() -> Struct1Former<
    Struct1FormerDefinition<(), Struct1<>, former::ReturnPreformed>
  >
  {
    Struct1Former::< Struct1FormerDefinition< (), Struct1<>, former::ReturnPreformed > >::new_coercing(former::ReturnPreformed)
  }
}

impl< Definition > former::EntityToFormer< Definition >
for Struct1< >
where
  Definition : former::FormerDefinition< Storage = Struct1FormerStorage<> >,
{
  type Former = Struct1Former< Definition >;
}

impl< > former::EntityToStorage for Struct1< >
where
{
  type Storage = Struct1FormerStorage<>;
}

#[derive(Debug)]
pub struct Struct1FormerDefinitionTypes< Context = (), Formed = Struct1<>, >
where
{
  _phantom : core::marker::PhantomData<(Context, Formed)>,
}

impl< Context, Formed, > core::default::Default
for Struct1FormerDefinitionTypes< Context, Formed, >
where
{
  fn default() -> Self
  {
    Self
    {
      _phantom : core::marker::PhantomData,
    }
  }
}

impl< Context, Formed, > former::FormerDefinitionTypes
for Struct1FormerDefinitionTypes< Context, Formed, >
where
{
  type Storage = Struct1FormerStorage<>;
  type Formed = Formed;
  type Context = Context;
}

impl< Context, Formed > former::FormerMutator
for Struct1FormerDefinitionTypes< Context, Formed >
{
}

#[derive(Debug)]
pub struct Struct1FormerDefinition< Context = (), Formed = Struct1<>, End = former::ReturnPreformed, >
where
{
  _phantom : core::marker::PhantomData<(Context, Formed, End)>,
}

impl< Context, Formed, End, > core::default::Default for Struct1FormerDefinition< Context, Formed, End, >
where
{
  fn default() -> Self
  {
    Self
    {
      _phantom : core::marker::PhantomData,
    }
  }
}

impl< Context, Formed, End, > former::FormerDefinition for Struct1FormerDefinition< Context, Formed, End, >
where
  End : former::FormingEnd< Struct1FormerDefinitionTypes< Context, Formed, > >,
{
  type Types = Struct1FormerDefinitionTypes< Context, Formed, >;
  type End = End;
  type Storage = Struct1FormerStorage<>;
  type Formed = Formed;
  type Context = Context;
}


pub struct Struct1FormerStorage<>
where
{

  pub vec_1 : core::option::Option<Vec<String>>,

  pub hashmap_1 : core::option::Option<collection_tools::HashMap<String, String>>,

  pub hashset_1 : core::option::Option<collection_tools::HashSet<String>>,
}

impl< > core::default::Default for Struct1FormerStorage<>
where
{
  #[ inline( always ) ]
  fn default() -> Self
  {
    Self
    {
      vec_1 : core::option::Option::None,
      hashmap_1 : core::option::Option::None,
      hashset_1 : core::option::Option::None,
    }
  }
}

impl< > former::Storage for Struct1FormerStorage<>
where
{
  type Preformed = Struct1<>;
}

impl< > former::StoragePreform for Struct1FormerStorage<>
where
{
  // type Preformed = Struct1<>;

  fn preform(mut self) -> Self::Preformed
  {
    let vec_1 = if self.vec_1.is_some()
    {
      self.vec_1.take().unwrap()
    }
    else
    {
      {
        trait MaybeDefault<T>
        {
          fn maybe_default(self: &Self) -> T
          {
            panic!("Field 'vec_1' isn't initialized")
          }
        }

        impl<T> MaybeDefault<T> for &core::marker::PhantomData<T> {}

        impl<T> MaybeDefault<T> for core::marker::PhantomData<T>
        where
          T : core::default::Default,
        {
          fn maybe_default(self: &Self) -> T
          {
            T::default()
          }
        }

        (&core::marker::PhantomData::<Vec<String>>).maybe_default()
      }
    };

    let hashmap_1 = if self.hashmap_1.is_some()
    {
      self.hashmap_1.take().unwrap()
    }
    else
    {
      {
        trait MaybeDefault<T>
        {
          fn maybe_default(self: &Self) -> T
          {
            panic!("Field 'hashmap_1' isn't initialized")
          }
        }

        impl<T> MaybeDefault<T> for &core::marker::PhantomData<T> {}

        impl<T> MaybeDefault<T> for core::marker::PhantomData<T>
        where
          T : core::default::Default,
        {
          fn maybe_default(self: &Self) -> T
          {
            T::default()
          }
        }

        (&core::marker::PhantomData::<collection_tools::HashMap<String, String>>).maybe_default()
      }
    };

    let hashset_1 = if self.hashset_1.is_some()
    {
      self.hashset_1.take().unwrap()
    }
    else
    {
      {
        trait MaybeDefault<T>
        {
          fn maybe_default(self: &Self) -> T
          {
            panic!("Field 'hashset_1' isn't initialized")
          }
        }

        impl<T> MaybeDefault<T> for &core::marker::PhantomData<T> {}

        impl<T> MaybeDefault<T> for core::marker::PhantomData<T>
        where
          T : core::default::Default,
        {
          fn maybe_default(self: &Self) -> T
          {
            T::default()
          }
        }

        (&core::marker::PhantomData::<collection_tools::HashSet<String>>).maybe_default()
      }
    };

    let result = Struct1::<>
    {
      vec_1, hashmap_1, hashset_1,
    };

    return result;
  }
}

pub struct Struct1Former< Definition = Struct1FormerDefinition<(), Struct1<>, former::ReturnPreformed>, >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage<> >,
{
  storage : <Definition::Types as former::FormerDefinitionTypes>::Storage,
  context : core::option::Option<<Definition::Types as former::FormerDefinitionTypes>::Context>,
  on_end : core::option::Option<Definition::End>,
}

#[automatically_derived]
impl< Definition, > Struct1Former< Definition, >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage<> >,
{



  #[ inline( always ) ]
  pub fn new(on_end: Definition::End) -> Self
  {
    Self::begin_coercing(None, None, on_end)
  }




  #[ inline( always ) ]
  pub fn new_coercing<IntoEnd>(end: IntoEnd) -> Self
  where
    IntoEnd : Into<Definition::End>,
  {
    Self::begin_coercing(None, None, end,)
  }




  #[ inline( always ) ]
  pub fn begin(mut storage: core::option::Option<<Definition::Types as former::FormerDefinitionTypes>::Storage>, context: core::option::Option<<Definition::Types as former::FormerDefinitionTypes>::Context>, on_end: <Definition as former::FormerDefinition>::End,) -> Self
  {
    if storage.is_none()
    {
      storage = Some(core::default::Default::default());
    }
    Self
    {
      storage: storage.unwrap(),
      context: context,
      on_end: core::option::Option::Some(on_end),
    }
  }




  #[ inline( always ) ]
  pub fn begin_coercing<IntoEnd>(mut storage: core::option::Option<<Definition::Types as former::FormerDefinitionTypes>::Storage>, context: core::option::Option<<Definition::Types as former::FormerDefinitionTypes>::Context>, on_end: IntoEnd,) -> Self
  where
    IntoEnd : core::convert::Into<<Definition as former::FormerDefinition>::End>,
  {
    if storage.is_none()
    {
      storage = Some(core::default::Default::default());
    }
    Self
    {
      storage: storage.unwrap(),
      context: context,
      on_end: core::option::Option::Some(core::convert::Into::into(on_end)),
    }
  }




  #[ inline( always ) ]
  pub fn form(self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed
  {
    self.end()
  }

  #[ inline( always ) ]
  pub fn end(mut self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed
  {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    former::FormingEnd::<Definition::Types>::call(&on_end, self.storage, context)
  }

  #[ inline( always ) ]
  pub fn _vec_1_assign< Former2 >( self ) -> Former2
  where
    Former2 : former::FormerBegin
    <
      former::VectorDefinition< String, Self, Self, Struct1SubformCollectionVec1End< Definition > >,
    >,
    former::VectorDefinition< String, Self, Self, Struct1SubformCollectionVec1End< Definition > > : former::FormerDefinition
    <
      // Storage : former::CollectionAdd< Entry = < collection_tools::Vec< String > as former::Collection >::Entry >,
      Storage = Vec< String >,
      Context = Struct1Former< Definition >,
      End = Struct1SubformCollectionVec1End< Definition >,
    >,
    Struct1SubformCollectionVec1End< Definition > : former::FormingEnd
    <
      < collection_tools::Vec< String > as former::EntityToDefinitionTypes< Self, Self > >::Types
    >,
  {
    Former2::former_begin( None, Some( self ), Struct1SubformCollectionVec1End::< Definition >::default() )
  }

  #[ inline( always ) ]
  pub fn vec_1( self ) -> former::CollectionFormer::
  <
    String,
    former::VectorDefinition< String, Self, Self, Struct1SubformCollectionVec1End< Definition > >,
  >
  where
    former::VectorDefinition< String, Self, Self, Struct1SubformCollectionVec1End< Definition > > : former::FormerDefinition
    <
      // Storage : former::CollectionAdd< Entry = < collection_tools::Vec< String > as former::Collection >::Entry >,
      Storage = Vec< String >,
      Context = Struct1Former< Definition >,
      End = Struct1SubformCollectionVec1End< Definition >,
    >,
    Struct1SubformCollectionVec1End< Definition > : former::FormingEnd
    <
      < collection_tools::Vec< String > as former::EntityToDefinitionTypes< Self, Self > >::Types
    >,
  {
    self._vec_1_assign::< former::CollectionFormer::
    <
      String,
      former::VectorDefinition< String, Self, Self, Struct1SubformCollectionVec1End< Definition > >,
    > > ()
  }

  #[ inline( always ) ]
  pub fn _hashmap_1_assign< Former2 >( self ) -> Former2
  where
    Former2 : former::FormerBegin
    <
      former::HashMapDefinition< String, String, Self, Self, Struct1SubformCollectionHashmap1End< Definition > >,
    >,
    former::HashMapDefinition< String, String, Self, Self, Struct1SubformCollectionHashmap1End< Definition > > : former::FormerDefinition
    <
      // Storage : former::CollectionAdd< Entry = < collection_tools::HashMap< String, String > as former::Collection >::Entry >,
      Storage = collection_tools::HashMap< String, String >,
      Context = Struct1Former< Definition >,
      End = Struct1SubformCollectionHashmap1End< Definition >,
    >,
    Struct1SubformCollectionHashmap1End< Definition > : former::FormingEnd
    <
      < collection_tools::HashMap< String, String > as former::EntityToDefinitionTypes< Self, Self > >::Types
    >,
  {
    Former2::former_begin( None, Some( self ), Struct1SubformCollectionHashmap1End::< Definition >::default() )
  }

  #[ inline( always ) ]
  pub fn hashmap_1( self ) -> former::CollectionFormer::
  <
    ( String, String ),
    former::HashMapDefinition< String, String, Self, Self, Struct1SubformCollectionHashmap1End< Definition > >,
  >
  where
    former::HashMapDefinition< String, String, Self, Self, Struct1SubformCollectionHashmap1End< Definition > > : former::FormerDefinition
    <
      // Storage : former::CollectionAdd< Entry = < collection_tools::HashMap< String, String > as former::Collection >::Entry >,
      Storage = collection_tools::HashMap< String, String >,
      Context = Struct1Former< Definition >,
      End = Struct1SubformCollectionHashmap1End< Definition >,
    >,
    Struct1SubformCollectionHashmap1End< Definition > : former::FormingEnd
    <
      < collection_tools::HashMap< String, String > as former::EntityToDefinitionTypes< Self, Self > >::Types
    >,
  {
    self._hashmap_1_assign::< former::CollectionFormer::
    <
      ( String, String ),
      former::HashMapDefinition< String, String, Self, Self, Struct1SubformCollectionHashmap1End< Definition > >,
    > > ()
  }

  #[ inline( always ) ]
  pub fn _hashset_1_assign< Former2 >( self ) -> Former2
  where
    Former2 : former::FormerBegin
    <
      former::HashSetDefinition< String, Self, Self, Struct1SubformCollectionHashset1End< Definition > >,
    >,
    former::HashSetDefinition< String, Self, Self, Struct1SubformCollectionHashset1End< Definition > > : former::FormerDefinition
    <
      // Storage : former::CollectionAdd< Entry = < collection_tools::HashSet< String > as former::Collection >::Entry >,
      Storage = collection_tools::HashSet< String >,
      Context = Struct1Former< Definition >,
      End = Struct1SubformCollectionHashset1End< Definition >,
    >,
    Struct1SubformCollectionHashset1End< Definition > : former::FormingEnd
    <
      < collection_tools::HashSet< String > as former::EntityToDefinitionTypes< Self, Self > >::Types
    >,
  {
    Former2::former_begin( None, Some( self ), Struct1SubformCollectionHashset1End::< Definition >::default() )
  }

  #[ inline( always ) ]
  pub fn hashset_1( self ) -> former::CollectionFormer::
  <
    String,
    former::HashSetDefinition< String, Self, Self, Struct1SubformCollectionHashset1End< Definition > >,
  >
  where
    former::HashSetDefinition< String, Self, Self, Struct1SubformCollectionHashset1End< Definition > > : former::FormerDefinition
    <
      // Storage : former::CollectionAdd< Entry = < collection_tools::HashSet< String > as former::Collection >::Entry >,
      Storage = collection_tools::HashSet< String >,
      Context = Struct1Former< Definition >,
      End = Struct1SubformCollectionHashset1End< Definition >,
    >,
    Struct1SubformCollectionHashset1End< Definition > : former::FormingEnd
    <
      < collection_tools::HashSet< String > as former::EntityToDefinitionTypes< Self, Self > >::Types
    >,
  {
    self._hashset_1_assign::< former::CollectionFormer::
    <
      String,
      former::HashSetDefinition< String, Self, Self, Struct1SubformCollectionHashset1End< Definition > >,
    > > ()
  }

}

impl< Definition, > Struct1Former< Definition, >
where
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage<>, Formed = Struct1<> >,
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage<> >,
{
  pub fn preform(self) -> < Definition::Types as former::FormerDefinitionTypes >::Formed
  {
    former::StoragePreform::preform(self.storage)
  }
}

impl< Definition, > Struct1Former< Definition, >
where
  Definition : former::FormerDefinition,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage<>, Formed = Struct1<> >,
{

  #[ inline( always ) ]
  pub fn perform(self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed
  {
    let result = self.form();
    return result;
  }
}

impl< Definition > former::FormerBegin< Definition > for Struct1Former< Definition, >
where
  Definition : former::FormerDefinition< Storage = Struct1FormerStorage<> >,
{
  #[ inline( always ) ]
  fn former_begin(storage: core::option::Option<Definition::Storage>, context: core::option::Option<Definition::Context>, on_end: Definition::End,) -> Self
  {
    debug_assert!(storage.is_none());
    Self::begin(None, context, on_end)
  }
}

#[ allow( dead_code ) ]
pub type Struct1AsSubformer< Superformer, End > = Struct1Former
<
  Struct1FormerDefinition< Superformer, Superformer, End, >,
>;

#[ allow( dead_code ) ]
pub trait Struct1AsSubformerEnd<SuperFormer>
where Self : former::FormingEnd< Struct1FormerDefinitionTypes<SuperFormer, SuperFormer>, >
{}

impl<SuperFormer, T> Struct1AsSubformerEnd<SuperFormer> for T
where
  Self : former::FormingEnd< Struct1FormerDefinitionTypes<SuperFormer, SuperFormer>, >,
{}

// = former assign end

pub struct Struct1SubformCollectionVec1End< Definition >
{
  _phantom : core::marker::PhantomData< ( Definition, ) >,
}

impl<Definition> Default for Struct1SubformCollectionVec1End< Definition >
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

// Struct1Former< Definition = Struct1FormerDefinition<(), Struct1<>, former::ReturnPreformed>, >

impl< Definition > former::FormingEnd
<
  former::VectorDefinitionTypes< String, Struct1Former< Definition >, Struct1Former< Definition > >
>
for Struct1SubformCollectionVec1End< Definition >
where
  Definition : former::FormerDefinition< Storage = Struct1FormerStorage >,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage >,
{
  #[ inline( always ) ]
  fn call( &self, storage : collection_tools::Vec< String >, super_former : Option< Struct1Former< Definition > > )
  -> Struct1Former< Definition, >
  {
    let mut super_former = super_former.unwrap();
    if let Some( ref mut field ) = super_former.storage.vec_1
    {
      former::CollectionAssign::assign( field, storage );
    }
    else
    {
      super_former.storage.vec_1 = Some( storage );
    }
    super_former
  }
}

pub struct Struct1SubformCollectionHashmap1End<Definition>
{
  _phantom : core::marker::PhantomData<(Definition,)>,
}

impl<Definition> Default for Struct1SubformCollectionHashmap1End<Definition>
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

impl< Definition, > former::FormingEnd
< former::HashMapDefinitionTypes< String, String, Struct1Former< Definition >, Struct1Former< Definition > > >
for Struct1SubformCollectionHashmap1End< Definition >
where
  Definition : former::FormerDefinition< Storage = Struct1FormerStorage >,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage >,
{
  #[ inline( always ) ]
  fn call( &self, storage : collection_tools::HashMap< String, String >, super_former : Option< Struct1Former< Definition > > )
  -> Struct1Former< Definition, >
  {
    let mut super_former = super_former.unwrap();
    if let Some( ref mut field ) = super_former.storage.hashmap_1
    {
      former::CollectionAssign::assign( field, storage );
    }
    else
    {
      super_former.storage.hashmap_1 = Some( storage );
    }
    super_former
  }
}

pub struct Struct1SubformCollectionHashset1End<Definition>
{
  _phantom : core::marker::PhantomData<(Definition,)>,
}

impl<Definition> Default for Struct1SubformCollectionHashset1End<Definition>
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

impl< Definition, > former::FormingEnd
< former::HashSetDefinitionTypes< String, Struct1Former< Definition >, Struct1Former< Definition > > >
for Struct1SubformCollectionHashset1End< Definition >
where
  Definition : former::FormerDefinition< Storage = Struct1FormerStorage >,
  Definition::Types : former::FormerDefinitionTypes< Storage = Struct1FormerStorage >,
{
  #[ inline( always ) ]
  fn call( &self, storage : collection_tools::HashSet< String >, super_former : Option< Struct1Former< Definition >, > )
  -> Struct1Former< Definition, >
  {
    let mut super_former = super_former.unwrap();
    if let Some( ref mut field ) = super_former.storage.hashset_1
    {
      former::CollectionAssign::assign( field, storage );
    }
    else
    {
      super_former.storage.hashset_1 = Some( storage );
    }
    super_former
  }
}

// == end of generated

include!( "./only_test/collections_with_subformer.rs" );
