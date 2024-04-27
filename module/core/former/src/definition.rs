
// zzz : improve documentation
/// Map type of entity to storage and former.
pub trait EntityToFormer
{
  /// Storage to store fields of entity during formign process.
  type Storage;
  /// Former with default definition
  type Former;
  // type Formed;
  // type Context;
  // type Types;
  // type Definition;
}

// zzz : improve documentation
/// Map type of entity to storage and former.
pub trait EntityToFormer2< Definition >
{
  /// Storage to store fields of entity during formign process.
  type Storage;
  /// Former with default definition
  type Former;
  // type Definition = Definition;
  // type Formed;
  // type Context;
  // type Types;
  // type Definition;

  fn f1( _ : &Definition ) {}

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
