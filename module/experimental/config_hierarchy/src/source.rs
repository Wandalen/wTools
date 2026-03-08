use std::path::PathBuf;

/// Configuration source tracking where values come from in the hierarchy
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum ConfigSource
{
  /// Runtime parameter override (highest priority)
  Runtime,
  /// Environment variable
  Environment,
  /// Local config in current directory (path to config file)
  LocalCurrent( PathBuf ),
  /// Local config in parent directory (path to config file)
  LocalParent( PathBuf ),
  /// Global configuration file (path to config file)
  Global( PathBuf ),
  /// Built-in default value (lowest priority)
  Default,
}

impl ConfigSource
{
  /// Get display name for config source
  #[ inline ]
  #[ must_use ]
  pub fn display_name( &self ) -> String
  {
    match self
    {
      ConfigSource::Runtime => "runtime".to_string(),
      ConfigSource::Environment => "env".to_string(),
      ConfigSource::LocalCurrent( path ) | ConfigSource::LocalParent( path ) | ConfigSource::Global( path ) => path.display().to_string(),
      ConfigSource::Default => "default".to_string(),
    }
  }
}
