//! Runs unitore command executor.

pub use unitore ::executor;

fn main() -> Result< (), Box< dyn core ::error ::Error + Send + Sync > >
{
  executor ::execute()
}
