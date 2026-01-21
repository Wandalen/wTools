//! Runs unitore command executor.
//! qqq: ? aaa: added documantation.

pub use unitore ::executor;

fn main() -> Result< (), Box< dyn core ::error ::Error + Send + Sync > >
{
  executor ::execute()
}
