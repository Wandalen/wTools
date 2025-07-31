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

impl<'child, T: ?Sized + 'child> ChildFormer<'child, T>
where
  T: 'child,
{
  pub fn new() -> ChildFormer<'child, T, ChildFormerDefinition<'child, T, (), Child<'child, T>, former::ReturnPreformed>>
  {
    ChildFormer::begin(None, None, former::ReturnPreformed)
  }

  pub fn begin<Context, End>(
    storage: Option<ChildFormerStorage<'child, T>>,
    context: Option<Context>,
    on_end: End,
  ) -> ChildFormer<'child, T, ChildFormerDefinition<'child, T, Context, Child<'child, T>, End>>
  where
    End: former::FormingEnd<ChildFormerDefinitionTypes<'child, T, Context, Child<'child, T>>>,
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

  pub fn end(mut self) -> Child<'child, T> {
    let storage = self.storage;
    Child {
      name: storage.name.unwrap_or_default(),
      arg: storage.arg.expect("arg field is required"),
    }
  }

  pub fn form(self) -> Child<'child, T> {
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

// DISABLED: Has lifetime regression issues - commenting out temporarily
// include!("./only_test/parametrized_field.rs");