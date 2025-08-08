#![deny(missing_docs)]
#![allow(dead_code)]

use super::*;

/// Parameter description.
#[ derive( Debug, Default, PartialEq ) ]
pub struct Child {
  name: String,
  data: bool,
}

/// Parent required for the template.
#[ derive( Debug, Default, PartialEq ) ]
pub struct Parent {
  children: Vec<Child>,
}

// == Manual implementations for Parent ==

// Parent struct implementations
impl Parent {
  #[ inline( always ) ]
  pub fn former() -> ParentFormer<ParentFormerDefinition<(), Parent, former::ReturnPreformed>> {
    ParentFormer::<ParentFormerDefinition<(), Parent, former::ReturnPreformed>>::new_coercing(former::ReturnPreformed)
  }
}

impl<Definition> former::EntityToFormer<Definition> for Parent
where
  Definition: former::FormerDefinition<Storage = ParentFormerStorage>,
{
  type Former = ParentFormer<Definition>;
}

impl former::EntityToStorage for Parent {
  type Storage = ParentFormerStorage;
}

// Parent former definition types
#[ derive( Debug ) ]
pub struct ParentFormerDefinitionTypes<Context = (), Formed = Parent> {
  _phantom: core::marker::PhantomData<(Context, Formed)>,
}

impl<Context, Formed> core::default::Default for ParentFormerDefinitionTypes<Context, Formed> {
  fn default() -> Self {
    Self {
      _phantom: core::marker::PhantomData,
    }
  }
}

impl<Context, Formed> former::FormerDefinitionTypes for ParentFormerDefinitionTypes<Context, Formed> {
  type Storage = ParentFormerStorage;
  type Formed = Formed;
  type Context = Context;
}

impl<Context, Formed> former::FormerMutator for ParentFormerDefinitionTypes<Context, Formed> {}

// Parent former definition
#[ derive( Debug ) ]
pub struct ParentFormerDefinition<Context = (), Formed = Parent, End = former::ReturnPreformed> {
  _phantom: core::marker::PhantomData<(Context, Formed, End)>,
}

impl<Context, Formed, End> core::default::Default for ParentFormerDefinition<Context, Formed, End> {
  fn default() -> Self {
    Self {
      _phantom: core::marker::PhantomData,
    }
  }
}

impl<Context, Formed, End> former::FormerDefinition for ParentFormerDefinition<Context, Formed, End>
where
  End: former::FormingEnd<ParentFormerDefinitionTypes<Context, Formed>>,
{
  type Types = ParentFormerDefinitionTypes<Context, Formed>;
  type End = End;
  type Storage = ParentFormerStorage;
  type Formed = Formed;
  type Context = Context;
}

// Parent storage
pub struct ParentFormerStorage {
  pub children: core::option::Option<Vec<Child>>,
}

impl core::default::Default for ParentFormerStorage {
  #[ inline( always ) ]
  fn default() -> Self {
    Self {
      children: core::option::Option::None,
    }
  }
}

impl former::Storage for ParentFormerStorage {
  type Preformed = Parent;
}

impl former::StoragePreform for ParentFormerStorage {
  fn preform(mut self) -> Self::Preformed {
    let children = if self.children.is_some() {
      self.children.take().unwrap()
    } else {
      Default::default()
    };
    
    Parent { children }
  }
}

// Parent former
pub struct ParentFormer<Definition = ParentFormerDefinition<(), Parent, former::ReturnPreformed>>
where
  Definition: former::FormerDefinition<Storage = ParentFormerStorage>,
  Definition::Types: former::FormerDefinitionTypes<Storage = ParentFormerStorage>,
{
  pub storage: Definition::Storage,
  pub context: core::option::Option<Definition::Context>,
  pub on_end: core::option::Option<Definition::End>,
}

impl<Definition> ParentFormer<Definition>
where
  Definition: former::FormerDefinition<Storage = ParentFormerStorage>,
  Definition::Types: former::FormerDefinitionTypes<Storage = ParentFormerStorage>,
{
  #[ inline( always ) ]
  pub fn new(on_end: Definition::End) -> Self {
    Self::begin_coercing(None, None, on_end)
  }

  #[ inline( always ) ]
  pub fn new_coercing<IntoEnd>(end: IntoEnd) -> Self
  where
    IntoEnd: core::convert::Into<Definition::End>,
  {
    Self::begin_coercing(None, None, end)
  }

  #[ inline( always ) ]
  pub fn begin(
    mut storage: core::option::Option<Definition::Storage>,
    context: core::option::Option<Definition::Context>,
    on_end: <Definition as former::FormerDefinition>::End,
  ) -> Self {
    if storage.is_none() {
      storage = Some(Default::default());
    }
    Self {
      storage: storage.unwrap(),
      context,
      on_end: Some(on_end),
    }
  }

  #[ inline( always ) ]
  pub fn begin_coercing<IntoEnd>(
    mut storage: core::option::Option<Definition::Storage>,
    context: core::option::Option<Definition::Context>,
    on_end: IntoEnd,
  ) -> Self
  where
    IntoEnd: core::convert::Into<<Definition as former::FormerDefinition>::End>,
  {
    if storage.is_none() {
      storage = Some(Default::default());
    }
    Self {
      storage: storage.unwrap(),
      context,
      on_end: Some(on_end.into()),
    }
  }

  #[ inline( always ) ]
  pub fn form(self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed {
    self.end()
  }

  #[ inline( always ) ]
  pub fn end(mut self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed {
    let on_end = self.on_end.take().unwrap();
    let mut context = self.context.take();
    <Definition::Types as former::FormerMutator>::form_mutation(&mut self.storage, &mut context);
    former::FormingEnd::<Definition::Types>::call(&on_end, self.storage, context)
  }

  #[ inline( always ) ]
  pub fn children(mut self, src: Vec<Child>) -> Self {
    debug_assert!(self.storage.children.is_none());
    self.storage.children = Some(src);
    self
  }

  #[ inline( always ) ]
  pub fn _children_subform_entry<'a, Former2, Definition2>(self) -> Former2
  where
    Former2: former::FormerBegin<'a, Definition2>,
    Definition2: former::FormerDefinition<
      Storage = <Child as former::EntityToStorage>::Storage,
      Formed = Self,
      Context = Self,
      End = ParentSubformEntryChildrenEnd<Definition>,
    >,
    Definition: 'a,
    ParentSubformEntryChildrenEnd<Definition>:
      former::FormingEnd<<Child as former::EntityToDefinitionTypes<Self, Self>>::Types>,
  {
    Former2::former_begin(None, Some(self), ParentSubformEntryChildrenEnd::<Definition>::default())
  }

  #[ inline( always ) ]
  pub fn child(self, name: &str) -> ChildAsSubformer<Self, impl ChildAsSubformerEnd<Self>> {
    self._children_subform_entry::<ChildFormer<_>, _>().name(name)
  }

  #[ inline( always ) ]
  pub fn _child(
    self,
  ) -> <<Vec<Child> as former::Collection>::Entry as former::EntityToFormer<
    <<Vec<Child> as former::Collection>::Entry as former::EntityToDefinition<
      Self,
      Self,
      ParentSubformEntryChildrenEnd<Definition>,
    >>::Definition,
  >>::Former {
    self._children_subform_entry::<<<Vec<Child> as former::Collection>::Entry as former::EntityToFormer<_>>::Former, _>()
  }
}

impl<Definition> ParentFormer<Definition>
where
  Definition: former::FormerDefinition<Storage = ParentFormerStorage, Formed = Parent>,
  Definition::Types: former::FormerDefinitionTypes<Storage = ParentFormerStorage, Formed = Parent>,
{
  pub fn preform(self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed {
    former::StoragePreform::preform(self.storage)
  }
}

impl<Definition> ParentFormer<Definition>
where
  Definition: former::FormerDefinition<Storage = ParentFormerStorage, Formed = Parent>,
  Definition::Types: former::FormerDefinitionTypes<Storage = ParentFormerStorage, Formed = Parent>,
{
  #[ inline( always ) ]
  pub fn perform(self) -> Definition::Formed {
    
    self.form()
  }
}

// ParentSubformEntryChildrenEnd implementation
#[ derive( Debug ) ]
pub struct ParentSubformEntryChildrenEnd<Definition> {
  _phantom: core::marker::PhantomData<Definition>,
}

impl<Definition> Default for ParentSubformEntryChildrenEnd<Definition> {
  fn default() -> Self {
    Self {
      _phantom: core::marker::PhantomData,
    }
  }
}

impl<Definition> former::FormingEnd<ChildFormerDefinitionTypes<ParentFormer<Definition>, ParentFormer<Definition>>>
  for ParentSubformEntryChildrenEnd<Definition>
where
  Definition: former::FormerDefinition<Storage = ParentFormerStorage>,
{
  #[ inline( always ) ]
  fn call(
    &self,
    storage: ChildFormerStorage,
    super_former: core::option::Option<ParentFormer<Definition>>,
  ) -> ParentFormer<Definition> {
    let mut super_former = super_former.unwrap();
    let preformed = former::StoragePreform::preform(storage);
    if super_former.storage.children.is_none() {
      super_former.storage.children = Some(Vec::new());
    }
    super_former.storage.children.as_mut().unwrap().push(preformed);
    super_former
  }
}

// == Manual implementations for Child ==

// Child struct implementations
impl Child {
  #[ inline( always ) ]
  pub fn former() -> ChildFormer<ChildFormerDefinition<(), Child, former::ReturnPreformed>> {
    ChildFormer::<ChildFormerDefinition<(), Child, former::ReturnPreformed>>::new_coercing(former::ReturnPreformed)
  }
}

impl<Definition> former::EntityToFormer<Definition> for Child
where
  Definition: former::FormerDefinition<Storage = ChildFormerStorage>,
{
  type Former = ChildFormer<Definition>;
}

impl former::EntityToStorage for Child {
  type Storage = ChildFormerStorage;
}

impl<Context, Formed> former::EntityToDefinitionTypes<Context, Formed> for Child {
  type Types = ChildFormerDefinitionTypes<Context, Formed>;
}

impl<Context, Formed, End> former::EntityToDefinition<Context, Formed, End> for Child
where
  End: former::FormingEnd<ChildFormerDefinitionTypes<Context, Formed>>,
{
  type Definition = ChildFormerDefinition<Context, Formed, End>;
  type Types = ChildFormerDefinitionTypes<Context, Formed>;
}

// Child former definition types
#[ derive( Debug ) ]
pub struct ChildFormerDefinitionTypes<Context = (), Formed = Child> {
  _phantom: core::marker::PhantomData<(Context, Formed)>,
}

impl<Context, Formed> core::default::Default for ChildFormerDefinitionTypes<Context, Formed> {
  fn default() -> Self {
    Self {
      _phantom: core::marker::PhantomData,
    }
  }
}

impl<Context, Formed> former::FormerDefinitionTypes for ChildFormerDefinitionTypes<Context, Formed> {
  type Storage = ChildFormerStorage;
  type Formed = Formed;
  type Context = Context;
}

impl<Context, Formed> former::FormerMutator for ChildFormerDefinitionTypes<Context, Formed> {}

// Child former definition
#[ derive( Debug ) ]
pub struct ChildFormerDefinition<Context = (), Formed = Child, End = former::ReturnPreformed> {
  _phantom: core::marker::PhantomData<(Context, Formed, End)>,
}

impl<Context, Formed, End> core::default::Default for ChildFormerDefinition<Context, Formed, End> {
  fn default() -> Self {
    Self {
      _phantom: core::marker::PhantomData,
    }
  }
}

impl<Context, Formed, End> former::FormerDefinition for ChildFormerDefinition<Context, Formed, End>
where
  End: former::FormingEnd<ChildFormerDefinitionTypes<Context, Formed>>,
{
  type Types = ChildFormerDefinitionTypes<Context, Formed>;
  type End = End;
  type Storage = ChildFormerStorage;
  type Formed = Formed;
  type Context = Context;
}

// Child storage
pub struct ChildFormerStorage {
  pub name: core::option::Option<String>,
  pub data: core::option::Option<bool>,
}

impl core::default::Default for ChildFormerStorage {
  #[ inline( always ) ]
  fn default() -> Self {
    Self {
      name: core::option::Option::None,
      data: core::option::Option::None,
    }
  }
}

impl former::Storage for ChildFormerStorage {
  type Preformed = Child;
}

impl former::StoragePreform for ChildFormerStorage {
  fn preform(mut self) -> Self::Preformed {
    let name = if self.name.is_some() {
      self.name.take().unwrap()
    } else {
      Default::default()
    };
    let data = if self.data.is_some() {
      self.data.take().unwrap()
    } else {
      Default::default()
    };
    
    Child { name, data }
  }
}

// Child former
pub struct ChildFormer<Definition = ChildFormerDefinition<(), Child, former::ReturnPreformed>>
where
  Definition: former::FormerDefinition<Storage = ChildFormerStorage>,
  Definition::Types: former::FormerDefinitionTypes<Storage = ChildFormerStorage>,
{
  pub storage: Definition::Storage,
  pub context: core::option::Option<Definition::Context>,
  pub on_end: core::option::Option<Definition::End>,
}

impl<Definition> ChildFormer<Definition>
where
  Definition: former::FormerDefinition<Storage = ChildFormerStorage>,
  Definition::Types: former::FormerDefinitionTypes<Storage = ChildFormerStorage>,
{
  #[ inline( always ) ]
  pub fn new(on_end: Definition::End) -> Self {
    Self::begin_coercing(None, None, on_end)
  }

  #[ inline( always ) ]
  pub fn new_coercing<IntoEnd>(end: IntoEnd) -> Self
  where
    IntoEnd: core::convert::Into<Definition::End>,
  {
    Self::begin_coercing(None, None, end)
  }

  #[ inline( always ) ]
  pub fn begin(
    mut storage: core::option::Option<Definition::Storage>,
    context: core::option::Option<Definition::Context>,
    on_end: <Definition as former::FormerDefinition>::End,
  ) -> Self {
    if storage.is_none() {
      storage = Some(Default::default());
    }
    Self {
      storage: storage.unwrap(),
      context,
      on_end: Some(on_end),
    }
  }

  #[ inline( always ) ]
  pub fn begin_coercing<IntoEnd>(
    mut storage: core::option::Option<Definition::Storage>,
    context: core::option::Option<Definition::Context>,
    on_end: IntoEnd,
  ) -> Self
  where
    IntoEnd: core::convert::Into<<Definition as former::FormerDefinition>::End>,
  {
    if storage.is_none() {
      storage = Some(Default::default());
    }
    Self {
      storage: storage.unwrap(),
      context,
      on_end: Some(on_end.into()),
    }
  }

  #[ inline( always ) ]
  pub fn form(self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed {
    self.end()
  }

  #[ inline( always ) ]
  pub fn end(mut self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed {
    let on_end = self.on_end.take().unwrap();
    let mut context = self.context.take();
    <Definition::Types as former::FormerMutator>::form_mutation(&mut self.storage, &mut context);
    former::FormingEnd::<Definition::Types>::call(&on_end, self.storage, context)
  }

  #[ inline( always ) ]
  pub fn name(mut self, src: impl Into<String>) -> Self {
    debug_assert!(self.storage.name.is_none());
    self.storage.name = Some(src.into());
    self
  }

  #[ inline( always ) ]
  pub fn data(mut self, src: bool) -> Self {
    debug_assert!(self.storage.data.is_none());
    self.storage.data = Some(src);
    self
  }
}

impl<Definition> ChildFormer<Definition>
where
  Definition: former::FormerDefinition<Storage = ChildFormerStorage, Formed = Child>,
  Definition::Types: former::FormerDefinitionTypes<Storage = ChildFormerStorage, Formed = Child>,  
{
  pub fn preform(self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed {
    former::StoragePreform::preform(self.storage)
  }
}

impl<Definition> ChildFormer<Definition>
where
  Definition: former::FormerDefinition<Storage = ChildFormerStorage, Formed = Child>,
  Definition::Types: former::FormerDefinitionTypes<Storage = ChildFormerStorage, Formed = Child>,
{
  #[ inline( always ) ]
  pub fn perform(self) -> Definition::Formed {
    
    self.form()
  }
}

// Type aliases for subformer functionality
pub type ChildAsSubformer<Superformer, End> = ChildFormer<ChildFormerDefinition<Superformer, Superformer, End>>;

pub trait ChildAsSubformerEnd<SuperFormer>: former::FormingEnd<ChildFormerDefinitionTypes<SuperFormer, SuperFormer>> {}

impl<SuperFormer, T> ChildAsSubformerEnd<SuperFormer> for T
where
  T: former::FormingEnd<ChildFormerDefinitionTypes<SuperFormer, SuperFormer>>,
{}

// FormerBegin implementation for ChildFormer
impl<'storage, Definition> former::FormerBegin<'storage, Definition> for ChildFormer<Definition>
where
  Definition: former::FormerDefinition<Storage = ChildFormerStorage>,
  Definition::Context: 'storage,
  Definition::End: 'storage,
{
  #[ inline( always ) ]
  fn former_begin(
    storage: core::option::Option<Definition::Storage>,
    context: core::option::Option<Definition::Context>,
    on_end: Definition::End,
  ) -> Self {
    Self::begin(storage, context, on_end)
  }
}

include!("./only_test/subform_entry_child.rs");
