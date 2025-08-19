#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
#![allow(dead_code)]

use super::*;

/// Parameter description.
// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[ derive( Debug, Default, PartialEq, the_module::Former ) ]
#[ derive( Debug, Default, PartialEq ) ]
pub struct Child {
  name: String,
  data: bool,
}

/// Parent required for the template.
// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[ derive( Debug, Default, PartialEq, the_module::Former ) ]
#[ derive( Debug, Default, PartialEq ) ]
// #[ derive( Debug, Default, PartialEq, the_module::Former ) ] #[ debug ]
// #[ derive( Debug, Default, PartialEq ) ]
pub struct Parent {
  // #[ subform_collection( definition = former::VectorDefinition ) ]
  // #[ scalar( setter = false ) ]
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

impl<Context, Formed> former::EntityToDefinitionTypes<Context, Formed> for Parent {
  type Types = ParentFormerDefinitionTypes<Context, Formed>;
}

impl<Context, Formed, End> former::EntityToDefinition<Context, Formed, End> for Parent
where
  End: former::FormingEnd<ParentFormerDefinitionTypes<Context, Formed>>,
{
  type Definition = ParentFormerDefinition<Context, Formed, End>;
  type Types = ParentFormerDefinitionTypes<Context, Formed>;
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

// FormerBegin implementation for ParentFormer
impl<'storage, Definition> former::FormerBegin<'storage, Definition> for ParentFormer<Definition>
where
  Definition: former::FormerDefinition<Storage = ParentFormerStorage>,
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

// == begin of generated for Parent in context of attribute collection( former::VectorDefinition ) ]

#[ automatically_derived ]
impl<Definition> ParentFormer<Definition>
where
  Definition: former::FormerDefinition<Storage = ParentFormerStorage>,
{
  #[ inline( always ) ]
  pub fn _children_subform_collection<'a, Former2>(self) -> Former2
  where
    Former2: former::FormerBegin<'a, former::VectorDefinition<Child, Self, Self, ParentSubformCollectionChildrenEnd<Definition>>>,
    former::VectorDefinition<Child, Self, Self, ParentSubformCollectionChildrenEnd<Definition>>: former::FormerDefinition<
      Storage = Vec<Child>,
      Context = Self,
      End = ParentSubformCollectionChildrenEnd<Definition>,
    >,
    ParentSubformCollectionChildrenEnd<Definition>: former::FormingEnd<<Vec<Child> as former::EntityToDefinitionTypes<Self, Self>>::Types>,
    Definition: 'a,
  {
    Former2::former_begin(None, Some(self), ParentSubformCollectionChildrenEnd::<Definition>::default())
  }

  #[ inline( always ) ]
  pub fn children(
    self,
  ) -> former::CollectionFormer<Child, former::VectorDefinition<Child, Self, Self, ParentSubformCollectionChildrenEnd<Definition>>>
  where
    former::VectorDefinition<Child, Self, Self, ParentSubformCollectionChildrenEnd<Definition>>: former::FormerDefinition<
      Storage = Vec<Child>,
      Context = Self,
      End = ParentSubformCollectionChildrenEnd<Definition>,
    >,
    ParentSubformCollectionChildrenEnd<Definition>: former::FormingEnd<<Vec<Child> as former::EntityToDefinitionTypes<Self, Self>>::Types>,
  {
    self._children_subform_collection::<former::CollectionFormer<Child, former::VectorDefinition<Child, Self, Self, ParentSubformCollectionChildrenEnd<Definition>>>>()
  }
}

//

#[doc = r"Callback to return original former after forming of collection for `vec_1` is done. Callback replace content of collection assigning new content from subformer's storage."]
pub struct ParentSubformCollectionChildrenEnd<Definition> {
  _phantom: core::marker::PhantomData<(Definition,)>,
}

impl<Definition> Default for ParentSubformCollectionChildrenEnd<Definition> {
  #[ inline( always ) ]
  fn default() -> Self {
    Self {
      _phantom: core::marker::PhantomData,
    }
  }
}

#[ automatically_derived ]
impl<Definition>
  former::FormingEnd<<Vec<Child> as former::EntityToDefinitionTypes<ParentFormer<Definition>, ParentFormer<Definition>>>::Types>
  for ParentSubformCollectionChildrenEnd<Definition>
where
  Definition: former::FormerDefinition<Storage = ParentFormerStorage>,
{
  #[ inline( always ) ]
  fn call(&self, storage: Vec<Child>, super_former: Option<ParentFormer<Definition>>) -> ParentFormer<Definition> {
    let mut super_former = super_former.unwrap();
    if let Some(ref mut field) = super_former.storage.children {
      former::CollectionAssign::assign(field, storage);
    } else {
      super_former.storage.children = Some(storage);
    }
    super_former
  }
}

// == end of generated for Parent in context of attribute collection( former::VectorDefinition ) ]

include!("./only_test/subform_collection.rs");
