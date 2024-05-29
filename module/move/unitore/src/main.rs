//! Runs unitore command executor.
//! qqq : ? aaa: added documantation.

pub use unitore::executor;

fn main() -> Result< (), Box< dyn std::error::Error + Send + Sync > >
{
  executor::execute()
}
