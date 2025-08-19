#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
#![allow(dead_code)]

use super::*;

/// Child
// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[ derive( Debug, Default, PartialEq, the_module::Former ) ]
#[ derive( Debug, Default, PartialEq ) ]
pub struct Child {
  name: String,
  data: bool,
}

/// Parent

// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose

// #[ derive( Debug, Default, PartialEq, the_module::Former ) ]

#[ derive( Debug, Default, PartialEq ) ]
// #[ debug ]
// #[ derive( Debug, Default, PartialEq ) ]
pub struct Parent {
  // #[ scalar( setter = false ) ]
  // #[ scalar_subform ]
  child: Child,
}

impl<Definition> ParentFormer<Definition>
where
  Definition: former::FormerDefinition<Storage = <Parent as former::EntityToStorage>::Storage> + 'static,
{
  #[ inline( always ) ]
  pub fn _child_subform_scalar<Former2, Definition2>(self) -> Former2
  where
    Definition2: former::FormerDefinition<
      End = ParentFormerSubformScalarChildEnd<Definition>,
      Storage = <Child as former::EntityToStorage>::Storage,
      Formed = Self,
      Context = Self,
    > + 'static,
    Definition2::Types:
      former::FormerDefinitionTypes<Storage = <Child as former::EntityToStorage>::Storage, Formed = Self, Context = Self>,
    for<'a> Former2: former::FormerBegin<'a, Definition2>,
    Definition2::Storage: 'static,
    Definition2::Context: 'static,
    Definition2::End: 'static,
  {
    Former2::former_begin(None, Some(self), ParentFormerSubformScalarChildEnd::default())
  }
}

impl<Definition> ParentFormer<Definition>
where
  Definition: former::FormerDefinition<Storage = <Parent as former::EntityToStorage>::Storage> + 'static,
{
  #[ inline( always ) ]
  #[ allow( clippy::used_underscore_items ) ]
  pub fn child(self) -> ChildAsSubformer<Self, impl ChildAsSubformerEnd<Self>> {
    self._child_subform_scalar::<<Child as former::EntityToFormer<_>>::Former, _>()
  }
}

// = end

/// Represents the endpoint for the forming process of a scalar field managed by a subformer within a `Parent` entity.
///
/// This structure is a critical component of the forming process when using a subform scalar setter. It handles
/// the finalization of the scalar field's value that has been configured through its dedicated subformer.
/// Essentially, this end action integrates the individually formed scalar value back into the parent structure.
///
/// ## Type Parameters
///
/// - `Definition`: The type that defines the former setup for the `Parent` entity, influencing storage and behavior during forming.
///
/// ## Parameters of `call`
///
/// - `substorage`: Storage type specific to the `Child`, containing the newly formed scalar value.
/// - `super_former`: An optional context of the `ParentFormer`, which will receive the value. The function ensures
///   that this context is not `None` and inserts the formed value into the designated field within `Parent`'s storage.
pub struct ParentFormerSubformScalarChildEnd<Definition> {
  _phantom: core::marker::PhantomData<fn(Definition)>,
}

impl<Definition> Default for ParentFormerSubformScalarChildEnd<Definition> {
  #[ inline( always ) ]
  fn default() -> Self {
    Self {
      _phantom: core::marker::PhantomData,
    }
  }
}

impl<Types2, Definition> former::FormingEnd<Types2> for ParentFormerSubformScalarChildEnd<Definition>
where
  Definition: former::FormerDefinition<Storage = <Parent as former::EntityToStorage>::Storage>,
  Types2: former::FormerDefinitionTypes<
    Storage = <Child as former::EntityToStorage>::Storage,
    Formed = ParentFormer<Definition>,
    Context = ParentFormer<Definition>,
  >,
{
  #[ inline( always ) ]
  fn call(&self, substorage: Types2::Storage, super_former: core::option::Option<Types2::Context>) -> Types2::Formed {
    let mut super_former = super_former.unwrap();
    debug_assert!(super_former.storage.child.is_none());
    super_former.storage.child = Some(::core::convert::Into::into(former::StoragePreform::preform(substorage)));
    super_former
  }
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
  pub child: core::option::Option<Child>,
}

impl core::default::Default for ParentFormerStorage {
  #[ inline( always ) ]
  fn default() -> Self {
    Self {
      child: core::option::Option::None,
    }
  }
}

impl former::Storage for ParentFormerStorage {
  type Preformed = Parent;
}

impl former::StoragePreform for ParentFormerStorage {
  fn preform(mut self) -> Self::Preformed {
    let child = if self.child.is_some() {
      self.child.take().unwrap()
    } else {
      Default::default()
    };
    
    Parent { child }
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

include!("./only_test/subform_scalar.rs");
