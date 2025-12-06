//! Build script for unilang crate.
//!
//! Generates static command definitions from YAML/JSON manifests using Perfect Hash Functions (PHF)
//! for zero-overhead command lookup at runtime.
//!
//! Supports both YAML and JSON formats with complete parity:
//! - Single-file mode: `.yaml`, `.yml`, or `.json` files
//! - Multi-file mode: Discovers all `.yaml`, `.yml`, and `.json` files
//!
//! ## Design Rules Compliance for PHF Build Process
//!
//! **✅ CORRECT Build-Time Optimization:**
//! - PHF generation during build for zero runtime overhead
//! - Static command definitions compiled into binary
//! - YAML-driven configuration for maintainability
//!
//! **❌ TESTING VIOLATIONS TO AVOID:**
//! - Do NOT create build-time performance tests comparing PHF vs `HashMap`
//! - Do NOT add timing measurements to verify PHF generation speed
//! - Do NOT create benchmark tests for PHF lookup performance in `tests/` directory
//!
//! **Performance Testing Rules:**
//! - PHF vs dynamic lookup comparisons belong in `benchkit` framework
//! - Build script should focus on correctness, not performance measurement
//! - Static command functionality testing goes in `tests/` (correctness only)
//!
//! ## Critical: Three-Layer Data Integrity Chain
//!
//! **Adding a new field to `StaticCommandDefinition` requires updates in THREE locations:**
//!
//! 1. **Struct Definition** (`src/static_data.rs`) - Add field to `StaticCommandDefinition`
//! 2. **Build Script Extraction** (this file) - Extract field from YAML in `generate_command_const()`
//! 3. **Conversion** (`src/static_data.rs`) - Map field in `From<&StaticCommandDefinition>`
//!
//! **Missing any location = silent data loss.** YAML values will be read but never reach runtime.
//!
//! **Example (Issue-088)**: The `auto_help_enabled` field was missing from steps 1 and 2,
//! causing all static commands to have `auto_help_enabled: false` regardless of YAML configuration.
//! This broke `.command.help` generation for all users.
//!
//! **Prevention**: When adding fields, update `generate_command_const()` to extract from YAML
//! and include in generated const, then add conversion tests in `tests/data/static_data.rs`.

#![allow(clippy::useless_format)]

use std::env;
use std::fs::File;
use std::io::Write;
#[cfg(feature = "static_registry")]
use std::io::BufWriter;
use std::path::Path;

// Type hint analysis modules (inlined for build-time use)
#[cfg(feature = "static_registry")]
mod type_hints
{
  use serde_yaml::Value;

  /// Analyzes argument definitions for potential type issues
  pub struct TypeAnalyzer
  {
    suppress_warnings : bool,
  }

  impl TypeAnalyzer
  {
    pub fn new() -> Self
    {
      Self
      {
        suppress_warnings : std::env::var( "UNILANG_SUPPRESS_TYPE_HINTS" )
          .map( | v | v == "1" )
          .unwrap_or( false ),
      }
    }

    pub fn analyze_argument( &self, arg : &Value ) -> Vec< TypeHint >
    {
      let mut hints = Vec::new();

      let name = arg[ "name" ].as_str().unwrap_or( "" );
      let kind = arg[ "kind" ].as_str().unwrap_or( "" );

      let default = arg[ "attributes" ][ "default" ].as_str()
        .or_else( || arg[ "default" ].as_str() );

      let suppress = arg[ "attributes" ][ "suppress_type_hint" ]
        .as_bool()
        .unwrap_or( false );

      if self.suppress_warnings || suppress
      {
        return hints;
      }

      // Check 1: Boolean-like default with String kind
      if kind == "String"
      {
        if let Some( def ) = default
        {
          if ( def == "true" || def == "false" ) && Self::context_suggests_boolean( name, arg )
          {
            hints.push( TypeHint::BooleanAsString
            {
              argument_name : name.to_string(),
              default_value : def.to_string(),
            } );
          }
        }
      }

      // Check 2: Integer-like default with String kind
      if kind == "String"
      {
        if let Some( def ) = default
        {
          if def.parse::< i64 >().is_ok() &&
             !def.starts_with( '0' ) &&
             !def.contains( '.' ) &&
             !def.is_empty() &&
             def.chars().all( | c | c.is_ascii_digit() ) &&
             Self::context_suggests_integer( name )
          {
            hints.push( TypeHint::IntegerAsString
            {
              argument_name : name.to_string(),
              default_value : def.to_string(),
            } );
          }
        }
      }

      hints
    }

    fn context_suggests_boolean( name : &str, arg : &Value ) -> bool
    {
      let name_lower = name.to_lowercase();
      let desc = arg[ "description" ].as_str().unwrap_or( "" );

      let boolean_keywords = [
        "enable", "disable", "flag", "is_", "has_", "can_",
        "should_", "dry_run", "dry-run", "force", "quiet", "verbose",
        "clone", "parallel", "recursive", "skip", "ignore",
      ];

      let name_suggests = boolean_keywords.iter()
        .any( | kw | name_lower.contains( kw ) );

      let desc_suggests = desc.contains( "true/false" ) ||
                         desc.contains( "(true|false)" ) ||
                         desc.contains( "true or false" );

      name_suggests || desc_suggests
    }

    fn context_suggests_integer( name : &str ) -> bool
    {
      let name_lower = name.to_lowercase();

      let integer_keywords = [
        "count", "limit", "max", "min", "size", "length",
        "verbosity", "level", "timeout", "retry", "retries",
        "attempts", "depth", "width", "height", "num",
      ];

      integer_keywords.iter().any( | kw | name_lower.contains( kw ) )
    }
  }

  #[derive(Debug, Clone)]
  pub enum TypeHint
  {
    BooleanAsString
    {
      argument_name : String,
      default_value : String,
    },
    IntegerAsString
    {
      argument_name : String,
      default_value : String,
    },
  }

  pub struct HintGenerator;

  impl HintGenerator
  {
    pub fn generate_warning( hint : &TypeHint ) -> String
    {
      match hint
      {
        TypeHint::BooleanAsString { argument_name, default_value } =>
        {
          format!(
            "💡 Type Hint: Argument '{argument_name}' might be better as Boolean kind\n\
             \n\
             Current:\n\
             - name: \"{argument_name}\"\n\
               kind: \"String\"\n\
               attributes:\n\
                 default: \"{default_value}\"  # String literal\n\
             \n\
             Suggestion:\n\
             - name: \"{argument_name}\"\n\
               kind: \"Boolean\"\n\
               attributes:\n\
                 default: {default_value}  # Boolean value (no quotes)\n\
             \n\
             Benefits:\n\
             - Automatic validation (rejects invalid values like 'yes', '1')\n\
             - Type-safe: cmd.get_boolean(\"{argument_name}\") instead of manual parsing\n\
             - Better error messages for users\n\
             \n\
             If intentional (e.g., code template): Add suppress_type_hint: true\n\
             To suppress all hints: export UNILANG_SUPPRESS_TYPE_HINTS=1\n"
          )
        },

        TypeHint::IntegerAsString { argument_name, default_value } =>
        {
          format!(
            "💡 Type Hint: Argument '{argument_name}' might be better as Integer kind\n\
             \n\
             Current:\n\
             - name: \"{argument_name}\"\n\
               kind: \"String\"\n\
               attributes:\n\
                 default: \"{default_value}\"  # String literal\n\
             \n\
             Suggestion:\n\
             - name: \"{argument_name}\"\n\
               kind: \"Integer\"\n\
               attributes:\n\
                 default: {default_value}  # Integer value (no quotes)\n\
               validation_rules:\n\
                 - Min: 0  # Add appropriate range\n\
                 - Max: 100\n\
             \n\
             Benefits:\n\
             - Automatic range validation\n\
             - Type-safe: cmd.get_integer(\"{argument_name}\") instead of manual parsing\n\
             \n\
             If intentional (version/ID/code): Add suppress_type_hint: true\n\
             To suppress all hints: export UNILANG_SUPPRESS_TYPE_HINTS=1\n"
          )
        },
      }
    }

    pub fn emit_hints( hints : Vec< TypeHint > )
    {
      if hints.is_empty()
      {
        return;
      }

      eprintln!();
      eprintln!( "{}", "=".repeat( 80 ) );
      eprintln!( "📋 Unilang Type Hints ({} suggestion{})",
        hints.len(),
        if hints.len() == 1 { "" } else { "s" }
      );
      eprintln!( "{}", "=".repeat( 80 ) );
      eprintln!();

      for hint in hints
      {
        eprintln!( "{}", Self::generate_warning( &hint ) );
        eprintln!( "{}", "-".repeat( 80 ) );
        eprintln!();
      }

      eprintln!(
        "ℹ️  Type hints help you choose appropriate argument types.\n\
         These are suggestions, not errors. Your build continues normally.\n"
      );
    }
  }
}

// Build-time validation module
// Fix(H37): build.rs does NOT use validation functions
// Root cause: validation was only implemented in runtime, not build-time
// Pitfall: Build scripts can silently generate bad code without validation
#[cfg(feature = "static_registry")]
mod build_validation
{
  /// Validates version string is non-empty.
  pub fn validate_version( version : &str, name : &str, file_path : &str ) -> Result< (), String >
  {
    if version.is_empty()
    {
      return Err( format!(
        "In file '{file_path}': Command '{name}' has empty version. Version string cannot be empty."
      ));
    }

    Ok(())
  }

  /// Computes full command name from namespace and name.
  /// Handles both YAML formats (per FR-REG-6):
  /// - Format 1: name: ".version" (compound name with dot)
  /// - Format 2: namespace: "system", name: "status" (separate, dots added)
  ///
  /// This mirrors the logic in `generate_static_commands` for PHF key generation.
  pub fn compute_full_name( namespace : &str, name : &str ) -> String
  {
    if namespace.is_empty()
    {
      // If name already has dot, use as-is; otherwise add dot
      if name.starts_with( '.' )
      {
        name.to_string()
      }
      else
      {
        format!( ".{name}" )
      }
    }
    else
    {
      // Namespace present: add dot if missing
      let ns = if namespace.starts_with( '.' )
      {
        namespace.to_string()
      }
      else
      {
        format!( ".{namespace}" )
      };
      format!( "{ns}.{name}" )
    }
  }

  /// Validates a complete command definition.
  /// This validates the FINAL `full_name` after dots are properly added.
  /// Supports both YAML formats (FR-REG-6).
  ///
  /// This is called for each command during build to ensure the From conversion
  /// will not panic at runtime.
  pub fn validate_command(
    name : &str,
    namespace : &str,
    version : &str,
    file_path : &str,
  ) -> Result< (), String >
  {
    // Name must not be empty
    if name.is_empty()
    {
      return Err( format!(
        "In file '{file_path}': Command name cannot be empty"
      ));
    }

    // Validate version
    validate_version( version, name, file_path )?;

    // Compute and validate full name (after dot normalization)
    let full_name = compute_full_name( namespace, name );
    if !full_name.starts_with( '.' )
    {
      return Err( format!(
        "In file '{file_path}': Invalid command '{name}'. Final full name '{full_name}' must start with dot prefix."
      ));
    }

    Ok(())
  }
}

#[cfg(feature = "static_registry")]
use type_hints::{ TypeAnalyzer, HintGenerator };

#[cfg(feature = "static_registry")]
use build_validation::validate_command;

fn main()
{
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=unilang.commands.yaml");

  // Only generate static registry if static_registry feature is enabled
  #[cfg(feature = "static_registry")]
  {
    generate_static_registry();
  }

  // If static_registry not enabled, create empty file
  #[cfg(not(feature = "static_registry"))]
  {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("static_commands.rs");
    let mut file = File::create(dest_path).unwrap();
    writeln!(file, "// Static registry not enabled").unwrap();
  }
}

#[cfg(feature = "static_registry")]
#[allow(clippy::too_many_lines)]
fn generate_static_registry()
{
  use std::path::PathBuf;

  let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("static_commands.rs");

  // Support both single file and multi-file discovery modes
  let yaml_discovery_paths = env::var("UNILANG_YAML_DISCOVERY_PATHS")
    .map_or_else(|_| vec!["./".to_string()], |paths| paths.split(':').map(String::from).collect::<Vec<_>>());

  // Track discovered files for build summary
  let mut discovered_files : Vec< PathBuf > = Vec::new();

  // Check if we have a custom manifest path from environment variable (single file mode)
  if let Ok(manifest_path) = env::var("UNILANG_STATIC_COMMANDS_PATH")
  {
    // Single file mode - supports both YAML and JSON
    let manifest_path_buf = Path::new(&manifest_path);

    let command_definitions = match parse_command_file(manifest_path_buf)
    {
      Ok(definitions) =>
      {
        discovered_files.push( manifest_path_buf.to_path_buf() );
        definitions
      },
      Err(e) =>
      {
        eprintln!("Warning: {e}");
        generate_empty_phf(&dest_path);
        return;
      }
    };

    // NEW: Analyze command definitions for type hints
    analyze_command_types( &command_definitions );

    generate_static_commands(&dest_path, &command_definitions);
    print_build_summary( &discovered_files, command_definitions.len() );
  }
  else
  {
    // Multi-file discovery mode using walkdir
    let mut all_command_definitions = Vec::new();

    // Multi-file discovery using walkdir
    {
      use walkdir::WalkDir;

      for discovery_path in &yaml_discovery_paths
      {
        // Add discovery path to rerun conditions
        println!("cargo:rerun-if-changed={discovery_path}");

        if Path::new(discovery_path).exists()
        {
          for entry in WalkDir::new(discovery_path)
            .into_iter()
            .filter_map(core::result::Result::ok)
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
              // Exclude test and example directories from static command discovery using proper path handling
              let path = e.path();

              // Convert to canonical form and check path components
              let should_exclude = path.components().any(|component| {
                if let std::path::Component::Normal(os_str) = component {
                  let name = os_str.to_string_lossy();
                  name == "tests" || name == "test_data" || name == "examples"
                } else {
                  false
                }
              });

              // Debug output to see what files are being processed (disabled in production)
              // if should_exclude {
              //   eprintln!("Excluding YAML file from static commands: {}", path.display());
              // }

              !should_exclude
            })
            .filter(|e| {
              if let Some(extension) = e.path().extension()
              {
                extension == "yaml" || extension == "yml" || extension == "json"
              }
              else
              {
                false
              }
            })
          {
            match parse_command_file(entry.path())
            {
              Ok(mut definitions) =>
              {
                discovered_files.push( entry.path().to_path_buf() );
                all_command_definitions.append(&mut definitions);
              },
              Err(e) =>
              {
                eprintln!("Warning: {e}");
              }
            }
          }
        }
      }
    }

    // If no YAML files found, try the default single file
    if all_command_definitions.is_empty()
    {
      let default_manifest = "unilang.commands.yaml";
      if let Ok(yaml_content) = std::fs::read_to_string(default_manifest)
      {
        match serde_yaml::from_str(&yaml_content)
        {
          Ok(definitions) =>
          {
            discovered_files.push( PathBuf::from( default_manifest ) );
            all_command_definitions = definitions;
          },
          Err(e) =>
          {
            eprintln!("Warning: Failed to parse default YAML manifest: {e}");
          }
        }
      }
    }

    // NEW: Analyze command definitions for type hints before generation
    analyze_command_types( &all_command_definitions );

    generate_static_commands(&dest_path, &all_command_definitions);
    print_build_summary( &discovered_files, all_command_definitions.len() );
  }
}

#[cfg(feature = "static_registry")]
fn analyze_command_types( command_definitions : &[ serde_yaml::Value ] )
{
  let analyzer = TypeAnalyzer::new();
  let mut all_hints = Vec::new();

  for cmd_def in command_definitions
  {
    // Analyze arguments if present
    if let Some( args ) = cmd_def.get( "arguments" ).and_then( | a | a.as_sequence() )
    {
      for arg in args
      {
        let hints = analyzer.analyze_argument( arg );
        all_hints.extend( hints );
      }
    }
  }

  // Emit all hints at end of build (after success message from cargo)
  HintGenerator::emit_hints( all_hints );
}

#[cfg(feature = "static_registry")]
fn generate_empty_phf(dest_path: &Path)
{
  let mut f = BufWriter::new(File::create(dest_path).unwrap());

  writeln!(f, "// Generated static commands (empty)").unwrap();
  writeln!(f, "use phf::phf_map;").unwrap();
  writeln!(f, "use ::unilang::static_data::{{StaticCommandDefinition, StaticCommandMap}};").unwrap();
  writeln!(f).unwrap();
  writeln!(f, "/// Static command registry (compile-time generated).").unwrap();
  writeln!(f, "/// ").unwrap();
  writeln!(f, "/// This provides zero-overhead lookup of compile-time registered commands.").unwrap();
  writeln!(f, "/// No PHF dependency required for crates using this registry.").unwrap();
  writeln!(f).unwrap();

  // Generate internal PHF map as const (not pub)
  writeln!(f, "const STATIC_COMMANDS_PHF: phf::Map<&'static str, &'static StaticCommandDefinition> = phf_map! {{}};").unwrap();
  writeln!(f).unwrap();

  // Generate public wrapper
  writeln!(f, "pub static STATIC_COMMANDS: StaticCommandMap = StaticCommandMap::from_phf_internal(&STATIC_COMMANDS_PHF);").unwrap();
}

#[cfg(feature = "static_registry")]
#[allow(clippy::too_many_lines)]
fn generate_static_commands(dest_path: &Path, command_definitions: &[serde_yaml::Value])
{
  let mut f = BufWriter::new(File::create(dest_path).unwrap());

  // Write header and imports
  writeln!(f, "// Generated static commands").unwrap();
  writeln!(f, "use phf::phf_map;").unwrap();

  // Import StaticCommandMap wrapper (absolute path works in both library and examples)
  writeln!(f, "use ::unilang::static_data::StaticCommandMap;").unwrap();

  // Only import types we'll actually use (absolute path works in both library and examples)
  if command_definitions.is_empty() {
    writeln!(f, "use ::unilang::static_data::StaticCommandDefinition;").unwrap();
  } else {
    // Check if we have any commands with arguments
    let has_arguments = command_definitions.iter()
      .any(|cmd| cmd["arguments"].as_sequence().is_some_and(|args| !args.is_empty()));

    if has_arguments {
      writeln!(f, "use ::unilang::static_data::{{StaticCommandDefinition, StaticArgumentDefinition, StaticArgumentAttributes, StaticKind}};").unwrap();
    } else {
      writeln!(f, "use ::unilang::static_data::StaticCommandDefinition;").unwrap();
    }
  }
  writeln!(f).unwrap();

  // Task 085 Item #3: Detect duplicate command names at build time
  // Track seen command names to prevent duplicates
  let mut seen_command_names : std::collections::HashMap< String, usize > = std::collections::HashMap::new();

  // Validate and generate const data for each command
  // Fix(H27, H37): Validate commands at build time to prevent runtime panics
  for (i, cmd_value) in command_definitions.iter().enumerate()
  {
    let name = cmd_value["name"].as_str().unwrap_or("");
    let namespace = cmd_value["namespace"].as_str().unwrap_or("");
    let version = cmd_value["version"].as_str().unwrap_or("");

    // Validate command definition before generating code
    // This ensures From<StaticCommandDefinition> cannot panic at runtime
    if let Err(e) = validate_command(name, namespace, version, "unilang.commands.yaml")
    {
      panic!(
        "\n╔══════════════════════════════════════════════════════════════════════════════╗\n\
         ║ BUILD ERROR: Invalid command definition                                       ║\n\
         ╟──────────────────────────────────────────────────────────────────────────────╢\n\
         ║ {e:<76} ║\n\
         ╟──────────────────────────────────────────────────────────────────════════────╢\n\
         ║ Fix: Ensure command names start with '.' (e.g., '.help', '.chat')             ║\n\
         ║      Ensure non-empty namespaces start with '.' (e.g., '.session')            ║\n\
         ║      Ensure version is not empty (e.g., '1.0.0')                              ║\n\
         ╚══════════════════════════════════════════════════════════════════════════════╝\n"
      );
    }

    // Task 085 Item #3: Check for duplicate command names
    // Compute full name using same logic as PHF generation
    use build_validation::compute_full_name;
    let full_name = compute_full_name(namespace, name);

    if let Some(first_index) = seen_command_names.get(&full_name)
    {
      panic!(
        "\n╔══════════════════════════════════════════════════════════════════════════════╗\n\
         ║ BUILD ERROR: Duplicate command name detected                                  ║\n\
         ╟──────────────────────────────────────────────────────────────────────────────╢\n\
         ║ Command '{}' is defined multiple times in YAML manifest{}║\n\
         ║                                                                                ║\n\
         ║ First occurrence: command index {}{}║\n\
         ║ Duplicate found:  command index {}{}║\n\
         ╟──────────────────────────────────────────────────────────────────────────────╢\n\
         ║ Fix: Rename one of the commands or remove the duplicate entry.                ║\n\
         ║      All command names must be unique across the entire manifest.             ║\n\
         ║                                                                                ║\n\
         ║ Task 085 Item #3: Prevents silent overwrites and confusing behavior           ║\n\
         ╚══════════════════════════════════════════════════════════════════════════════╝\n",
        full_name,
        " ".repeat(51 - full_name.len().min(51)),
        first_index,
        " ".repeat(67 - first_index.to_string().len()),
        i,
        " ".repeat(67 - i.to_string().len())
      );
    }

    seen_command_names.insert(full_name.clone(), i);

    // Task 085 Item #5: Validate parameter storage types (prevent wplan bug)
    // Check that multiple:true parameters use List storage type
    if let Some(arguments) = cmd_value["arguments"].as_sequence()
    {
      for arg in arguments
      {
        let arg_name = arg["name"].as_str().unwrap_or("");
        let multiple = arg["attributes"]["multiple"].as_bool().unwrap_or(false);

        if multiple
        {
          // Check if kind is a List
          let is_list = if let Some(kind_str) = arg["kind"].as_str()
          {
            // Simple string kind - check if it contains "List"
            kind_str.contains("List")
          }
          else if let Some(_kind_map) = arg["kind"].as_mapping()
          {
            // Complex kind structure like {List: ["String", null]}
            // If it has a "List" key or contains List, it's valid
            arg["kind"].as_mapping()
              .and_then(|m| m.keys().next())
              .and_then(|k| k.as_str())
              .is_some_and(|k| k == "List")
          }
          else
          {
            false
          };

          if !is_list
          {
            let kind_debug = format!("{:?}", arg["kind"]);
            panic!(
              "\n╔══════════════════════════════════════════════════════════════════════════════╗\n\
               ║ BUILD ERROR: Invalid parameter definition (wplan bug pattern)                 ║\n\
               ╟──────────────────────────────────────────────────────────────────────────────╢\n\
               ║ Command:   {}{}║\n\
               ║ Parameter: {}{}║\n\
               ║ Problem:   multiple:true but storage type is NOT List                         ║\n\
               ║                                                                                ║\n\
               ║ Current kind: {}{}║\n\
               ║                                                                                ║\n\
               ║ This causes silent data loss when multiple values overwrite each other.       ║\n\
               ╟──────────────────────────────────────────────────────────────────────────────╢\n\
               ║ Fix: Change parameter kind to List storage:                                   ║\n\
               ║                                                                                ║\n\
               ║   kind: {{List: [\"String\", null]}}  # For string values                        ║\n\
               ║   kind: {{List: [\"Integer\", null]}} # For integer values                       ║\n\
               ║                                                                                ║\n\
               ║ Task 085 Item #5: Prevents the wplan bug pattern                              ║\n\
               ╚══════════════════════════════════════════════════════════════════════════════╝\n",
              full_name,
              " ".repeat(68 - full_name.len().min(68)),
              arg_name,
              " ".repeat(68 - arg_name.len().min(68)),
              kind_debug,
              " ".repeat(63 - kind_debug.len().min(63))
            );
          }
        }
      }
    }

    generate_command_const(&mut f, i, cmd_value);
  }

  // Generate internal PHF map as const (not pub)
  writeln!(f, "const STATIC_COMMANDS_PHF: phf::Map<&'static str, &'static StaticCommandDefinition> = phf_map! {{").unwrap();

  for (i, cmd_value) in command_definitions.iter().enumerate()
  {
    let name = cmd_value["name"].as_str().unwrap_or("");
    let namespace = cmd_value["namespace"].as_str().unwrap_or("");

    let full_name = if namespace.is_empty()
    {
      // Command name may already have a leading dot, don't duplicate it
      if name.starts_with('.')
      {
        name.to_string()
      }
      else
      {
        format!(".{name}")
      }
    }
    else
    {
      // Strip leading dot from name to avoid double dots like ".system..status"
      format!("{namespace}.{}", name.trim_start_matches('.'))
    };

    writeln!(f, "  \"{full_name}\" => &CMD_{i},").unwrap();
  }

  writeln!(f, "}};").unwrap();
  writeln!(f).unwrap();

  // Generate public wrapper
  writeln!(f, "/// Static command registry (compile-time generated).").unwrap();
  writeln!(f, "/// ").unwrap();
  writeln!(f, "/// This map provides zero-overhead lookup of compile-time registered commands.").unwrap();
  writeln!(f, "/// Commands are keyed by their full name (e.g., \".help\" or \"namespace.command\").").unwrap();
  writeln!(f, "/// ").unwrap();
  writeln!(f, "/// No PHF dependency required for crates using this registry.").unwrap();
  writeln!(f, "pub static STATIC_COMMANDS: StaticCommandMap = StaticCommandMap::from_phf_internal(&STATIC_COMMANDS_PHF);").unwrap();
}

#[cfg(feature = "static_registry")]
fn generate_command_const(f: &mut BufWriter<File>, index: usize, cmd_value: &serde_yaml::Value)
{
  let name = cmd_value["name"].as_str().unwrap_or("");
  let namespace = cmd_value["namespace"].as_str().unwrap_or("");
  let description = cmd_value["description"].as_str().unwrap_or("");
  let hint = cmd_value["hint"].as_str().unwrap_or("");
  let status = cmd_value["status"].as_str().unwrap_or("stable");
  let version = cmd_value["version"].as_str().unwrap_or("1.0.0");
  let idempotent = cmd_value["idempotent"].as_bool().unwrap_or(false);
  let deprecation_message = cmd_value["deprecation_message"].as_str().unwrap_or("");
  let http_method_hint = cmd_value["http_method_hint"].as_str().unwrap_or("");
  // Fix(issue-088): Extract auto_help_enabled from YAML (defaults to true)
  let auto_help_enabled = cmd_value["auto_help_enabled"].as_bool().unwrap_or(true);
  // Fix(issue-089): Extract category from YAML (defaults to empty string)
  let category = cmd_value["category"].as_str().unwrap_or("");

  // Generate arguments array
  if let Some(arguments) = cmd_value["arguments"].as_sequence()
  {
    if !arguments.is_empty()
    {
      for (arg_i, arg_value) in arguments.iter().enumerate()
      {
        generate_argument_const(f, index, arg_i, arg_value);
      }

      writeln!(f, "const CMD_{index}_ARGS: &[StaticArgumentDefinition] = &[").unwrap();
      for arg_i in 0..arguments.len()
      {
        writeln!(f, "  CMD_{index}_ARG_{arg_i},").unwrap();
      }
      writeln!(f, "];").unwrap();
      writeln!(f).unwrap();
    }
  }

  // Generate arrays for aliases, tags, permissions, examples
  generate_string_array(f, &format!("CMD_{index}_ALIASES"), &cmd_value["aliases"]);
  generate_string_array(f, &format!("CMD_{index}_TAGS"), &cmd_value["tags"]);
  generate_string_array(f, &format!("CMD_{index}_PERMISSIONS"), &cmd_value["permissions"]);
  generate_string_array(f, &format!("CMD_{index}_EXAMPLES"), &cmd_value["examples"]);

  // Generate the main command const
  writeln!(f, "const CMD_{index}: StaticCommandDefinition = StaticCommandDefinition {{").unwrap();
  writeln!(f, "  name: \"{}\",", escape_string(name)).unwrap();
  writeln!(f, "  namespace: \"{}\",", escape_string(namespace)).unwrap();
  writeln!(f, "  description: \"{}\",", escape_string(description)).unwrap();
  writeln!(f, "  hint: \"{}\",", escape_string(hint)).unwrap();

  // Arguments
  if let Some(arguments) = cmd_value["arguments"].as_sequence()
  {
    if arguments.is_empty()
    {
      writeln!(f, "  arguments: &[],").unwrap();
    }
    else
    {
      writeln!(f, "  arguments: CMD_{index}_ARGS,").unwrap();
    }
  }
  else
  {
    writeln!(f, "  arguments: &[],").unwrap();
  }

  writeln!(f, "  routine_link: None,").unwrap();
  writeln!(f, "  status: \"{}\",", escape_string(status)).unwrap();
  writeln!(f, "  version: \"{}\",", escape_string(version)).unwrap();
  writeln!(f, "  tags: CMD_{index}_TAGS,").unwrap();
  writeln!(f, "  aliases: CMD_{index}_ALIASES,").unwrap();
  writeln!(f, "  permissions: CMD_{index}_PERMISSIONS,").unwrap();
  writeln!(f, "  idempotent: {idempotent},").unwrap();
  writeln!(f, "  deprecation_message: \"{}\",", escape_string(deprecation_message)).unwrap();
  writeln!(f, "  http_method_hint: \"{}\",", escape_string(http_method_hint)).unwrap();
  writeln!(f, "  examples: CMD_{index}_EXAMPLES,").unwrap();
  // Fix(issue-088): Include auto_help_enabled field in generated PHF const
  writeln!(f, "  auto_help_enabled: {auto_help_enabled},").unwrap();
  // Fix(issue-089): Include category field in generated PHF const
  writeln!(f, "  category: \"{}\",", escape_string(category)).unwrap();
  writeln!(f, "}};").unwrap();
  writeln!(f).unwrap();
}

#[cfg(feature = "static_registry")]
fn generate_argument_const(f: &mut BufWriter<File>, cmd_index: usize, arg_index: usize, arg_value: &serde_yaml::Value)
{
  let name = arg_value["name"].as_str().unwrap_or("");
  let description = arg_value["description"].as_str().unwrap_or("");
  let hint = arg_value["hint"].as_str().unwrap_or("");
  let kind_str = arg_value["kind"].as_str().unwrap_or("String");

  // Generate validation rules array
  if let Some(validation_rules) = arg_value["validation_rules"].as_sequence()
  {
    if !validation_rules.is_empty()
    {
      writeln!(f, "const CMD_{cmd_index}_ARG_{arg_index}_VALIDATION: &[StaticValidationRule] = &[").unwrap();
      for _rule in validation_rules
      {
        // For now, we'll keep validation rules empty since they're complex to parse
        // This can be expanded later if needed
      }
      writeln!(f, "];").unwrap();
    }
  }

  // Generate aliases and tags arrays
  generate_string_array(f, &format!("CMD_{cmd_index}_ARG_{arg_index}_ALIASES"), &arg_value["aliases"]);
  generate_string_array(f, &format!("CMD_{cmd_index}_ARG_{arg_index}_TAGS"), &arg_value["tags"]);

  // Generate attributes
  let attributes = &arg_value["attributes"];
  let optional = attributes["optional"].as_bool().unwrap_or(false);
  let multiple = attributes["multiple"].as_bool().unwrap_or(false);
  let default_value = attributes["default"].as_str();
  let sensitive = attributes["sensitive"].as_bool().unwrap_or(false);
  let interactive = attributes["interactive"].as_bool().unwrap_or(false);

  writeln!(f, "const CMD_{cmd_index}_ARG_{arg_index}_ATTRS: StaticArgumentAttributes = StaticArgumentAttributes {{").unwrap();
  writeln!(f, "  optional: {optional},").unwrap();
  writeln!(f, "  multiple: {multiple},").unwrap();
  if let Some(default) = default_value
  {
    writeln!(f, "  default: Some(\"{}\"),", escape_string(default)).unwrap();
  }
  else
  {
    writeln!(f, "  default: None,").unwrap();
  }
  writeln!(f, "  sensitive: {sensitive},").unwrap();
  writeln!(f, "  interactive: {interactive},").unwrap();
  writeln!(f, "}};").unwrap();

  // Generate kind
  let static_kind = match kind_str
  {
    "Integer" => "StaticKind::Integer",
    "Float" => "StaticKind::Float",
    "Boolean" => "StaticKind::Boolean",
    "Path" => "StaticKind::Path",
    "File" => "StaticKind::File",
    "Directory" => "StaticKind::Directory",
    "Url" => "StaticKind::Url",
    "DateTime" => "StaticKind::DateTime",
    "Pattern" => "StaticKind::Pattern",
    "JsonString" => "StaticKind::JsonString",
    "Object" => "StaticKind::Object",
    _ => "StaticKind::String", // Default fallback, includes "String"
  };

  // Generate the argument const
  writeln!(f, "const CMD_{cmd_index}_ARG_{arg_index}: StaticArgumentDefinition = StaticArgumentDefinition {{").unwrap();
  writeln!(f, "  name: \"{}\",", escape_string(name)).unwrap();
  writeln!(f, "  kind: {static_kind},").unwrap();
  writeln!(f, "  attributes: CMD_{cmd_index}_ARG_{arg_index}_ATTRS,").unwrap();
  writeln!(f, "  hint: \"{}\",", escape_string(hint)).unwrap();
  writeln!(f, "  description: \"{}\",", escape_string(description)).unwrap();
  writeln!(f, "  validation_rules: &[],").unwrap(); // Keep empty for now
  writeln!(f, "  aliases: CMD_{cmd_index}_ARG_{arg_index}_ALIASES,").unwrap();
  writeln!(f, "  tags: CMD_{cmd_index}_ARG_{arg_index}_TAGS,").unwrap();
  writeln!(f, "}};").unwrap();
  writeln!(f).unwrap();
}

#[cfg(feature = "static_registry")]
fn generate_string_array(f: &mut BufWriter<File>, const_name: &str, yaml_value: &serde_yaml::Value)
{
  if let Some(array) = yaml_value.as_sequence()
  {
    if array.is_empty()
    {
      writeln!(f, "const {const_name}: &[&str] = &[];").unwrap();
    }
    else
    {
      writeln!(f, "const {const_name}: &[&str] = &[").unwrap();
      for item in array
      {
        if let Some(s) = item.as_str()
        {
          writeln!(f, "  \"{}\",", escape_string(s)).unwrap();
        }
      }
      writeln!(f, "];").unwrap();
    }
  }
  else
  {
    writeln!(f, "const {const_name}: &[&str] = &[];").unwrap();
  }
}

#[cfg(feature = "static_registry")]
fn escape_string(s: &str) -> String
{
  s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Parse command definitions from a file based on its extension.
///
/// Supports:
/// - `.yaml`, `.yml` → `serde_yaml` parsing
/// - `.json` → `serde_json` parsing (converted to `serde_yaml::Value` for consistency)
#[cfg(feature = "static_registry")]
fn parse_command_file(file_path: &Path) -> Result<Vec<serde_yaml::Value>, String>
{
  let content = std::fs::read_to_string(file_path)
    .map_err(|e| format!("Failed to read file {}: {e}", file_path.display()))?;

  let extension = file_path
    .extension()
    .and_then(|ext| ext.to_str())
    .ok_or_else(|| format!("File has no extension: {}", file_path.display()))?;

  match extension
  {
    "yaml" | "yml" =>
    {
      serde_yaml::from_str(&content)
        .map_err(|e| format!("Failed to parse YAML file {}: {e}", file_path.display()))
    }
    "json" =>
    {
      #[cfg(feature = "json_parser")]
      {
        // Parse JSON first, then convert to YAML Value for unified processing
        let json_value: serde_json::Value = serde_json::from_str(&content)
          .map_err(|e| format!("Failed to parse JSON file {}: {e}", file_path.display()))?;

        // Convert JSON Value to YAML Value via intermediate JSON string
        // This works because both implement serde Serialize/Deserialize
        let json_str = serde_json::to_string(&json_value)
          .map_err(|e| format!("Failed to serialize JSON: {e}"))?;

        serde_yaml::from_str(&json_str)
          .map_err(|e| format!("Failed to convert JSON to YAML representation: {e}"))
      }
      #[cfg(not(feature = "json_parser"))]
      {
        Err(format!("JSON support requires the 'json_parser' feature. File: {}", file_path.display()))
      }
    }
    other => Err(format!("Unsupported file extension '{other}' for file: {}", file_path.display()))
  }
}

/// Print build summary showing what unilang did automatically.
///
/// Makes the invisible visible - shows developers that unilang processed their YAML files
/// and generated the command registry, so they don't need to write build.rs themselves.
///
/// Suppressible via `UNILANG_QUIET_BUILD` environment variable.
#[cfg(feature = "static_registry")]
fn print_build_summary( yaml_files : &[ std::path::PathBuf ], command_count : usize )
{
  // Don't print if no files discovered
  if yaml_files.is_empty() { return; }

  // Allow suppression for CI builds or when output is unwanted
  if env::var( "UNILANG_QUIET_BUILD" ).is_ok() { return; }

  eprintln!();
  eprintln!( "╔══════════════════════════════════════════════════════════╗" );
  eprintln!( "║  Unilang: Compile-Time Command Registry                 ║" );
  eprintln!( "╟──────────────────────────────────────────────────────────╢" );

  let file_word = if yaml_files.len() == 1 { "file" } else { "files" };
  eprintln!( "║  Found {} YAML {:<46}║", yaml_files.len(), file_word );

  // Show up to 5 files, then "... and N more"
  let files_to_show = yaml_files.iter().take( 5 );
  for file in files_to_show
  {
    let name = file.file_name()
      .and_then( |n| n.to_str() )
      .unwrap_or( "unknown" );

    eprintln!( "║    - {name:<50} ║" );
  }

  if yaml_files.len() > 5
  {
    let remaining = yaml_files.len() - 5;
    eprintln!( "║    ... and {} more {:<38}║", remaining, "" );
  }

  let command_word = if command_count == 1 { "command" } else { "commands" };
  eprintln!( "║  Generated PHF map with {command_count} {command_word:<32}║" );
  eprintln!( "║  Lookup time: ~80ns (zero runtime overhead)             ║" );
  eprintln!( "║                                                          ║" );
  eprintln!( "║  ✅ You did NOT need to write build.rs                  ║" );
  eprintln!( "║  ✅ YAML parsed at compile-time                         ║" );
  eprintln!( "║  ✅ Command registry ready                              ║" );
  eprintln!( "║                                                          ║" );
  eprintln!( "║  Docs: https://docs.rs/unilang                          ║" );
  eprintln!( "╚══════════════════════════════════════════════════════════╝" );
  eprintln!();
}