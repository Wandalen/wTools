mod private
{
  pub struct Own;
  pub struct Orphan;
  pub struct Exposed;
  pub struct Prelude;
}

crate::mod_interface!
{
  own use Own;
  orphan use Orphan;
  exposed use Exposed;
  prelude use Prelude;
}
