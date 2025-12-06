#![ cfg_attr( doc, doc = include_str!( "../readme.md" ) ) ]
#![ deny( missing_docs ) ]

//! Generic hierarchical configuration with 6-level priority and source tracking
//!
//! Implement 3 traits then use `ConfigManager` for complete config handling

// Core modules
#[ cfg( feature = "enabled" ) ]
mod error;
#[ cfg( feature = "enabled" ) ]
mod source;
#[ cfg( feature = "enabled" ) ]
mod traits;
#[ cfg( feature = "enabled" ) ]
mod type_detection;
#[ cfg( feature = "enabled" ) ]
mod conversion;
#[ cfg( feature = "enabled" ) ]
mod path_discovery;
#[ cfg( feature = "enabled" ) ]
mod hierarchy;
#[ cfg( feature = "enabled" ) ]
mod manager;

// Optional modules
#[ cfg( feature = "file_ops" ) ]
mod file_ops;

#[ cfg( feature = "display" ) ]
pub mod display;

// Core exports
#[ cfg( feature = "enabled" ) ]
pub use error::ValidationError;
#[ cfg( feature = "enabled" ) ]
pub use source::ConfigSource;
#[ cfg( feature = "enabled" ) ]
pub use traits::{ ConfigDefaults, ConfigPaths, ConfigValidator, EnvVarCasing };
#[ cfg( feature = "enabled" ) ]
pub use type_detection::detect_and_convert_value;
#[ cfg( feature = "enabled" ) ]
pub use conversion::json_value_to_display_string;
#[ cfg( feature = "enabled" ) ]
pub use path_discovery::{ get_global_config_path, get_global_config_dir, get_local_config_path, discover_local_configs };
#[ cfg( feature = "enabled" ) ]
pub use hierarchy::resolve_config_value;
#[ cfg( feature = "enabled" ) ]
pub use manager::ConfigManager;

// Internal exports (crate-only)
#[ cfg( feature = "file_ops" ) ]
pub( crate ) use path_discovery::discover_local_configs_internal;

// File operations exports
#[ cfg( feature = "file_ops" ) ]
pub use conversion::{ yaml_to_json, json_to_yaml };

#[ cfg( feature = "file_ops" ) ]
pub use file_ops::{ load_config_file, save_config_file, delete_config_file, atomic_config_modify };
