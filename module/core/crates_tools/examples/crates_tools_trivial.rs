#![allow(missing_docs)]
#![allow(clippy ::std_instead_of_core)]
use crates_tools :: *;

fn main() -> Result< (), Box< dyn std ::error ::Error > >
{
  #[ cfg(feature = "enabled") ]
  {
    // Download a package with specific version from `crates.io`
    let crate_archive = CrateArchive ::download_crates_io("test_experimental_c", "0.1.0")?;

    println!("Downloaded crate successfully. Files:");
    println!();

    for path in crate_archive.list()
    {
      // Get content from a specific file from the archive
      let bytes = crate_archive.content_bytes(path)
        .ok_or_else(|| format!("File not found: {}", path.display()))?;

      // Handle both text and binary files gracefully
      if let Ok(string) = core ::str ::from_utf8(bytes)
      {
        println!("# {}", path.display());
        println!("```");
        println!("{string}");
        println!("```");
        println!();
      }
      else
      {
        println!("# {} [BINARY]", path.display());
        println!("Binary file, {} bytes", bytes.len());
        println!();
      }
    }
  }

  Ok(())
}
