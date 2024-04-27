
// zzz : improve documentation
/// Map type of entity to former.
pub trait EntityToDefinition< Context, Formed, End >
{
  type Definition : FormerDefinition;
}

// zzz : improve documentation
/// Map type of entity to former.
pub trait EntityToFormer< Definition >
where
  // Definition : FormerDefinition< Storage = Self::Storage >,
  Definition : FormerDefinition,
{
  type Former;
  fn f1( _ : &Definition ) {}
}

// zzz : improve documentation
/// Map type of entity to storage.
pub trait EntityToStorage
{
  type Storage;
}

/// zzz : write description
pub trait FormerDefinitionTypes : Sized
{
  type Storage : Default;
  type Formed;
  type Context;
}

/// zzz : write description
pub trait FormerDefinition : Sized
{
  type Types : crate::FormerDefinitionTypes< Storage = Self::Storage, Formed = Self::Formed, Context = Self::Context >;
  type End : crate::FormingEnd< Self::Types >;
  type Storage : Default;
  type Formed;
  type Context;
}
