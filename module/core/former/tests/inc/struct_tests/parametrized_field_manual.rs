#![allow(dead_code)]
#[allow(unused_imports)]
use super::*;

/// Parameter description.
#[allow(explicit_outlives_requirements)]
#[derive(Debug, PartialEq)]
pub struct Child<'child, T: ?Sized + 'child> {
  name: String,
  arg: &'child T,
}

// Manual implementation to understand what the derive macro should generate
// This will guide the fix for the derive macro

// Storage struct for the former
#[derive(Debug)]
pub struct ChildFormerStorage<'child, T: ?Sized + 'child> {
  name: Option<String>,
  arg: Option<&'child T>,
}

impl<'child, T: ?Sized + 'child> Default for ChildFormerStorage<'child, T> {
  fn default() -> Self {
    Self {
      name: None,
      arg: None,
    }
  }
}

impl<'child, T: ?Sized + 'child> former::Storage for ChildFormerStorage<'child, T> {
  type Preformed = Child<'child, T>;
}

impl<'child, T: ?Sized + 'child> former::StoragePreform for ChildFormerStorage<'child, T> {
  fn preform(self) -> Self::Preformed {
    Child {
      name: self.name.unwrap_or_default(),
      arg: self.arg.expect("arg field is required"),
    }
  }
}

// The former implementation
#[derive(Debug)]
pub struct ChildFormer<'child, T: ?Sized + 'child, Definition = ChildFormerDefinition<'child, T>>
where
  Definition: former::FormerDefinition<Storage = ChildFormerStorage<'child, T>>,
{
  storage: Definition::Storage,
  context: Option<Definition::Context>,
  on_end: Option<Definition::End>,
}

impl<'child, T: ?Sized + 'child> ChildFormer<'child, T, ChildFormerDefinition<'child, T, (), Child<'child, T>, former::ReturnPreformed>>
where
  T: 'child,
{
  pub fn new() -> Self
  {
    ChildFormer::begin(None, None, former::ReturnPreformed)
  }
}

// Generic implementations for ChildFormer
impl<'child, T: ?Sized + 'child, Definition> ChildFormer<'child, T, Definition>
where
  T: 'child,
  Definition: former::FormerDefinition<Storage = ChildFormerStorage<'child, T>>,
{
  pub fn begin(
    storage: Option<Definition::Storage>,
    context: Option<Definition::Context>,
    on_end: Definition::End,
  ) -> Self
  {
    let storage = storage.unwrap_or_default();
    ChildFormer {
      storage,
      context,
      on_end: Some(on_end),
    }
  }

  pub fn name(mut self, value: impl Into<String>) -> Self {
    self.storage.name = Some(value.into());
    self
  }

  pub fn arg(mut self, value: &'child T) -> Self {
    self.storage.arg = Some(value);
    self
  }

  pub fn end(mut self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed {
    let on_end = self.on_end.take().unwrap();
    let context = self.context.take();
    former::FormingEnd::<Definition::Types>::call(&on_end, self.storage, context)
  }

  pub fn form(self) -> <Definition::Types as former::FormerDefinitionTypes>::Formed {
    self.end()
  }
}

// Definition types and traits (simplified for this test)
#[derive(Debug)]
pub struct ChildFormerDefinitionTypes<'child, T: ?Sized + 'child, Context, Formed> {
  _phantom: std::marker::PhantomData<(&'child T, Context, Formed)>,
}

impl<'child, T: ?Sized + 'child, Context, Formed> former::FormerDefinitionTypes
  for ChildFormerDefinitionTypes<'child, T, Context, Formed>
{
  type Storage = ChildFormerStorage<'child, T>;
  type Formed = Formed;
  type Context = Context;
}

impl<'child, T: ?Sized + 'child, Context, Formed> former::FormerMutator
  for ChildFormerDefinitionTypes<'child, T, Context, Formed>
{
}

#[derive(Debug)]
pub struct ChildFormerDefinition<'child, T: ?Sized + 'child, Context = (), Formed = Child<'child, T>, End = former::ReturnPreformed> {
  _phantom: std::marker::PhantomData<(&'child T, Context, Formed, End)>,
}

impl<'child, T: ?Sized + 'child, Context, Formed, End> former::FormerDefinition
  for ChildFormerDefinition<'child, T, Context, Formed, End>
where
  End: former::FormingEnd<ChildFormerDefinitionTypes<'child, T, Context, Formed>>,
{
  type Types = ChildFormerDefinitionTypes<'child, T, Context, Formed>;
  type End = End;
  type Storage = ChildFormerStorage<'child, T>;
  type Formed = Formed;
  type Context = Context;
}

// Add the Child::former() method
impl<'child, T: ?Sized + 'child> Child<'child, T> {
  pub fn former() -> ChildFormer<'child, T, ChildFormerDefinition<'child, T, (), Child<'child, T>, former::ReturnPreformed>> {
    ChildFormer::new()
  }
}

// Add FormerBegin implementation
impl<'a, 'child, T: ?Sized + 'child, Definition> former::FormerBegin<'a, Definition> 
for ChildFormer<'child, T, Definition>
where
  Definition: former::FormerDefinition<Storage = ChildFormerStorage<'child, T>>,
  'child: 'a,
  T: 'a,
  Definition::Context: 'a,
  Definition::End: 'a,
{
  #[inline(always)]
  fn former_begin(
    storage: ::core::option::Option<Definition::Storage>,
    context: ::core::option::Option<Definition::Context>,
    on_end: Definition::End,
  ) -> Self {
    let storage = storage.unwrap_or_default();
    ChildFormer {
      storage,
      context,
      on_end: Some(on_end),
    }
  }
}

include!("./only_test/parametrized_field.rs");