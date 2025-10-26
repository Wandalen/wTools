//! Cargo.toml template generator

/// Generate Cargo.toml content for unilang project
pub fn cargo_toml( project_name : &str, author : Option< &str >, license : Option< &str > ) -> String
{
  let author_line = author
    .map( |a| format!( "authors = [ \"{}\" ]\n", a ) )
    .unwrap_or_default();

  let license_name = license.unwrap_or( "MIT" );

  format!(
r#"[package]
name = "{project_name}"
version = "0.1.0"
edition = "2021"
{author_line}license = "{license_name}"

[dependencies]
# Unilang with default features (Approach #2: Multi-YAML Build-Time Static)
unilang = "0.32"

# ⚠️  IMPORTANT: Do NOT add these - unilang already provides them:
# ❌ serde_yaml (via yaml_parser feature)
# ❌ walkdir (via multi_file feature)
# ❌ phf (via static_registry feature)
#
# ⚠️  IMPORTANT: Do NOT create build.rs
# Unilang already provides build.rs that handles everything.
#
# If you see warnings during cargo build, that's unilang working!
# It's processing your YAML files at compile-time.
"#,
    project_name = project_name,
    author_line = author_line,
    license_name = license_name
  )
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_cargo_toml_minimal()
  {
    let content = cargo_toml( "my-cli", None, None );
    assert!( content.contains( "name = \"my-cli\"" ) );
    assert!( content.contains( "unilang = \"0.32\"" ) );
    assert!( content.contains( "Do NOT create build.rs" ) );
    assert!( content.contains( "license = \"MIT\"" ) );
  }

  #[test]
  fn test_cargo_toml_with_author()
  {
    let content = cargo_toml( "my-cli", Some( "John Doe <john@example.com>" ), None );
    assert!( content.contains( "authors = [ \"John Doe <john@example.com>\" ]" ) );
  }

  #[test]
  fn test_cargo_toml_with_license()
  {
    let content = cargo_toml( "my-cli", None, Some( "Apache-2.0" ) );
    assert!( content.contains( "license = \"Apache-2.0\"" ) );
  }
}
