//! Runs unitore command executor.

pub use unitore::executor;

fn main() -> Result< (), Box< dyn std::error::Error + Send + Sync > >
{
  executor::execute()
}
