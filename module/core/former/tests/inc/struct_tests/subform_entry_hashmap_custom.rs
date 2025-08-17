#![allow(dead_code)]

#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use collection_tools::HashMap;

// Child struct with Former derived for builder pattern support
// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[ derive( Clone, Debug, PartialEq, former::Former ) ]
#[ derive( Clone, Debug, PartialEq ) ]
pub struct Child {
  name: String,
  description: String,
}

// Parent struct to hold commands
// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// #[ derive( Debug, PartialEq, former::Former ) ]
#[ derive( Debug, PartialEq ) ]
// #[ debug ]
// #[ derive( Debug, PartialEq ) ]
pub struct Parent {
  // #[ scalar( setter = false ) ]
  command: HashMap< String, Child >,
}

// Use ChildFormer as custom subformer for ParentFormer to add commands by name.
impl<Definition> ParentFormer<Definition>
where
  Definition: former::FormerDefinition<Storage = <Parent as former::EntityToStorage>::Storage> + 'static,
{
  // more generic version
  #[ inline( always ) ]
  pub fn _children_subform_entry_with_closure<Former2, Definition2, Types2>(self) -> Former2
  where
    Types2: former::FormerDefinitionTypes<Storage = ChildFormerStorage, Formed = Self, Context = Self> + 'static,
    Definition2: former::FormerDefinition<
      Types = Types2,
      End = former::FormingEndClosure<Types2>,
      Storage = ChildFormerStorage,
      Formed = Self,
      Context = Self,
    > + 'static,
    Definition2::End: former::FormingEnd<Definition2::Types>,
    for<'a> Former2: former::FormerBegin<'a, Definition2>,
    Definition2::Storage: 'static,
    Definition2::Context: 'static,
    Definition2::End: 'static,
  {
    let on_end = |substorage: ChildFormerStorage, super_former: core::option::Option<Self>| -> Self {
      let mut super_former = super_former.unwrap();
      if super_former.storage.command.is_none() {
        super_former.storage.command = Some(HashMap::default());
      }
      if let Some(ref mut children) = super_former.storage.command {
        former::CollectionAdd::add(
          children,
          <<HashMap< String, Child > as former::Collection>::Val as former::ValToEntry<HashMap< String, Child >>>::val_to_entry(
            former::StoragePreform::preform(substorage),
          ),
        );
      }
      super_former
    };
    Former2::former_begin(None, Some(self), former::FormingEndClosure::new(on_end))
  }

  // reuse _command_subform_entry
  #[ inline( always ) ]
  pub fn command(self, name: &str) -> ChildAsSubformer<Self, impl ChildAsSubformerEnd<Self>> {
    self._command_subform_entry::<ChildFormer<_>, _>().name(name)
  }

  // that's how you should do custom subformer setters if you can't reuse _command_subform_entry
  #[ inline( always ) ]
  pub fn command2(self, name: &str) -> ChildAsSubformer<Self, impl ChildAsSubformerEnd<Self>> {
    let on_end = |substorage: ChildFormerStorage, super_former: core::option::Option<Self>| -> Self {
      let mut super_former = super_former.unwrap();
      let preformed = former::StoragePreform::preform(substorage);

      if super_former.storage.command.is_none() {
        super_former.storage.command = Some(HashMap::default());
      }

      // add instance to the collection
      super_former
        .storage
        .command
        .as_mut()
        .unwrap()
        .entry(preformed.name.clone())
        .or_insert(preformed.clone());

      // custom logic to add two instances to the collection
      super_former
        .storage
        .command
        .as_mut()
        .unwrap()
        .entry(format!("{}_2", preformed.name))
        .or_insert(preformed.clone());

      super_former
    };
    let subformer = ChildAsSubformer::<Self, _>::begin(None, Some(self), former::FormingEndClosure::new(on_end));
    subformer.name(name)
  }
}

impl former::ValToEntry<HashMap< String, Child >> for Child {
  type Entry = (String, Child);
  #[ inline( always ) ]
  fn val_to_entry(self) -> Self::Entry {
    (self.name.clone(), self)
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
  pub command: core::option::Option<HashMap< String, Child >>,
}

impl core::default::Default for ParentFormerStorage {
  #[ inline( always ) ]
  fn default() -> Self {
    Self {
      command: core::option::Option::None,
    }
  }
}

impl former::Storage for ParentFormerStorage {
  type Preformed = Parent;
}

impl former::StoragePreform for ParentFormerStorage {
  fn preform(mut self) -> Self::Preformed {
    let command = if self.command.is_some() {
      self.command.take().unwrap()
    } else {
      Default::default()
    };
    
    Parent { command }
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
  pub fn _command_subform_entry<'a, Former2, Definition2>(self) -> Former2
  where
    Former2: former::FormerBegin<'a, Definition2>,
    Definition2: former::FormerDefinition<
      Storage = <Child as former::EntityToStorage>::Storage,
      Formed = Self,
      Context = Self,
      End = ParentSubformEntryCommandEnd<Definition>,
    >,
    Definition: 'a,
    ParentSubformEntryCommandEnd<Definition>:
      former::FormingEnd<<Child as former::EntityToDefinitionTypes<Self, Self>>::Types>,
  {
    Former2::former_begin(None, Some(self), ParentSubformEntryCommandEnd::<Definition>::default())
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

// ParentSubformEntryCommandEnd implementation
#[ derive( Debug ) ]
pub struct ParentSubformEntryCommandEnd<Definition> {
  _phantom: core::marker::PhantomData<Definition>,
}

impl<Definition> Default for ParentSubformEntryCommandEnd<Definition> {
  fn default() -> Self {
    Self {
      _phantom: core::marker::PhantomData,
    }
  }
}

impl<Definition> former::FormingEnd<ChildFormerDefinitionTypes<ParentFormer<Definition>, ParentFormer<Definition>>>
  for ParentSubformEntryCommandEnd<Definition>
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
    if super_former.storage.command.is_none() {
      super_former.storage.command = Some(HashMap::default());
    }
    if let Some(ref mut command) = super_former.storage.command {
      former::CollectionAdd::add(
        command,
        <<HashMap< String, Child > as former::Collection>::Val as former::ValToEntry<HashMap< String, Child >>>::val_to_entry(
          preformed,
        ),
      );
    }
    super_former
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
  pub description: core::option::Option<String>,
}

impl core::default::Default for ChildFormerStorage {
  #[ inline( always ) ]
  fn default() -> Self {
    Self {
      name: core::option::Option::None,
      description: core::option::Option::None,
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
    let description = if self.description.is_some() {
      self.description.take().unwrap()
    } else {
      Default::default()
    };
    
    Child { name, description }
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
  pub fn description(mut self, src: impl Into<String>) -> Self {
    debug_assert!(self.storage.description.is_none());
    self.storage.description = Some(src.into());
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

#[ test ]
fn custom1() {
  let got = Parent::former()
  .command( "echo" )
    .description( "prints all subjects and properties" ) // sets additional properties using custom subformer
    .end()
  .command( "exit" )
    .description( "just exit" ) // Sets additional properties using using custom subformer
    .end()
  .form();

  let got = got
    .command
    .iter()
    .map(|e| e.0)
    .cloned()
    .collect::<collection_tools::HashSet< String >>();
  let exp = collection_tools::hset!["echo".into(), "exit".into(),];
  a_id!(got, exp);
}

#[ test ]
fn custom2() {
  let got = Parent::former()
  .command2( "echo" )
    .description( "prints all subjects and properties" ) // sets additional properties using custom subformer
    .end()
  .command2( "exit" )
    .description( "just exit" ) // Sets additional properties using using custom subformer
    .end()
  .form();

  let got = got
    .command
    .iter()
    .map(|e| e.0)
    .cloned()
    .collect::<collection_tools::HashSet< String >>();
  let exp = collection_tools::hset!["echo".into(), "echo_2".into(), "exit".into(), "exit_2".into(),];
  a_id!(got, exp);
}
