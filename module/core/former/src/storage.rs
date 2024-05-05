
/// zzz : write description
pub trait Storage : ::core::default::Default
{
  type Formed;
}

/// zzz : write description
pub trait StoragePreform
{
  type Preformed;
  fn preform( self ) -> Self::Preformed;
}
