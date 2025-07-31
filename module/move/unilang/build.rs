use std::env;
use std::fs::File;
use std::io::{ BufWriter, Write };
use std::path::Path;

fn main()
{
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=unilang.commands.yaml");

  let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("static_commands.rs");

  // Check if we have a custom manifest path from environment variable
  let manifest_path = env::var("UNILANG_STATIC_COMMANDS_PATH")
    .unwrap_or_else(|_| "unilang.commands.yaml".to_string());

  // Read and parse the YAML manifest
  let yaml_content = match std::fs::read_to_string(&manifest_path)
  {
    Ok(content) => content,
    Err(_) =>
    {
      // If manifest doesn't exist, create empty PHF
      generate_empty_phf(&dest_path);
      return;
    }
  };

  let command_definitions: Vec<serde_yaml::Value> = match serde_yaml::from_str(&yaml_content)
  {
    Ok(definitions) => definitions,
    Err(e) =>
    {
      panic!("Failed to parse YAML manifest: {}", e);
    }
  };

  generate_static_commands(&dest_path, &command_definitions);
}

fn generate_empty_phf(dest_path: &Path)
{
  let mut f = BufWriter::new(File::create(dest_path).unwrap());

  writeln!(f, "// Generated static commands (empty)").unwrap();
  writeln!(f, "use phf::{{phf_map, Map}};").unwrap();
  writeln!(f, "use crate::static_data::StaticCommandDefinition;").unwrap();
  writeln!(f).unwrap();
  writeln!(f, "pub static STATIC_COMMANDS: Map<&'static str, &'static StaticCommandDefinition> = phf_map! {{}};").unwrap();
}

fn generate_static_commands(dest_path: &Path, command_definitions: &[serde_yaml::Value])
{
  let mut f = BufWriter::new(File::create(dest_path).unwrap());

  // Write header and imports
  writeln!(f, "// Generated static commands").unwrap();
  writeln!(f, "use phf::{{phf_map, Map}};").unwrap();
  writeln!(f, "use crate::static_data::{{StaticCommandDefinition, StaticArgumentDefinition, StaticArgumentAttributes, StaticKind, StaticValidationRule}};").unwrap();
  writeln!(f).unwrap();

  // Generate const data for each command
  for (i, cmd_value) in command_definitions.iter().enumerate()
  {
    generate_command_const(&mut f, i, cmd_value);
  }

  // Generate the PHF map
  writeln!(f, "pub static STATIC_COMMANDS: Map<&'static str, &'static StaticCommandDefinition> = phf_map! {{").unwrap();

  for (i, cmd_value) in command_definitions.iter().enumerate()
  {
    let name = cmd_value["name"].as_str().unwrap_or("");
    let namespace = cmd_value["namespace"].as_str().unwrap_or("");
    
    let full_name = if namespace.is_empty()
    {
      format!(".{}", name)
    }
    else
    {
      format!("{}.{}", namespace, name)
    };

    writeln!(f, "  \"{}\" => &CMD_{},", full_name, i).unwrap();
  }

  writeln!(f, "}};").unwrap();
}

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

  // Generate arguments array
  if let Some(arguments) = cmd_value["arguments"].as_sequence()
  {
    if !arguments.is_empty()
    {
      for (arg_i, arg_value) in arguments.iter().enumerate()
      {
        generate_argument_const(f, index, arg_i, arg_value);
      }

      writeln!(f, "const CMD_{}_ARGS: &[StaticArgumentDefinition] = &[", index).unwrap();
      for arg_i in 0..arguments.len()
      {
        writeln!(f, "  CMD_{}_ARG_{},", index, arg_i).unwrap();
      }
      writeln!(f, "];").unwrap();
      writeln!(f).unwrap();
    }
  }

  // Generate arrays for aliases, tags, permissions, examples
  generate_string_array(f, &format!("CMD_{}_ALIASES", index), &cmd_value["aliases"]);
  generate_string_array(f, &format!("CMD_{}_TAGS", index), &cmd_value["tags"]);
  generate_string_array(f, &format!("CMD_{}_PERMISSIONS", index), &cmd_value["permissions"]);
  generate_string_array(f, &format!("CMD_{}_EXAMPLES", index), &cmd_value["examples"]);

  // Generate the main command const
  writeln!(f, "const CMD_{}: StaticCommandDefinition = StaticCommandDefinition {{", index).unwrap();
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
      writeln!(f, "  arguments: CMD_{}_ARGS,", index).unwrap();
    }
  }
  else
  {
    writeln!(f, "  arguments: &[],").unwrap();
  }

  writeln!(f, "  routine_link: None,").unwrap();
  writeln!(f, "  status: \"{}\",", escape_string(status)).unwrap();
  writeln!(f, "  version: \"{}\",", escape_string(version)).unwrap();
  writeln!(f, "  tags: CMD_{}_TAGS,", index).unwrap();
  writeln!(f, "  aliases: CMD_{}_ALIASES,", index).unwrap();
  writeln!(f, "  permissions: CMD_{}_PERMISSIONS,", index).unwrap();
  writeln!(f, "  idempotent: {},", idempotent).unwrap();
  writeln!(f, "  deprecation_message: \"{}\",", escape_string(deprecation_message)).unwrap();
  writeln!(f, "  http_method_hint: \"{}\",", escape_string(http_method_hint)).unwrap();
  writeln!(f, "  examples: CMD_{}_EXAMPLES,", index).unwrap();
  writeln!(f, "}};").unwrap();
  writeln!(f).unwrap();
}

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
      writeln!(f, "const CMD_{}_ARG_{}_VALIDATION: &[StaticValidationRule] = &[", cmd_index, arg_index).unwrap();
      for _rule in validation_rules
      {
        // For now, we'll keep validation rules empty since they're complex to parse
        // This can be expanded later if needed
      }
      writeln!(f, "];").unwrap();
    }
  }

  // Generate aliases and tags arrays
  generate_string_array(f, &format!("CMD_{}_ARG_{}_ALIASES", cmd_index, arg_index), &arg_value["aliases"]);
  generate_string_array(f, &format!("CMD_{}_ARG_{}_TAGS", cmd_index, arg_index), &arg_value["tags"]);

  // Generate attributes
  let attributes = &arg_value["attributes"];
  let optional = attributes["optional"].as_bool().unwrap_or(false);
  let multiple = attributes["multiple"].as_bool().unwrap_or(false);
  let default_value = attributes["default"].as_str();
  let sensitive = attributes["sensitive"].as_bool().unwrap_or(false);
  let interactive = attributes["interactive"].as_bool().unwrap_or(false);

  writeln!(f, "const CMD_{}_ARG_{}_ATTRS: StaticArgumentAttributes = StaticArgumentAttributes {{", cmd_index, arg_index).unwrap();
  writeln!(f, "  optional: {},", optional).unwrap();
  writeln!(f, "  multiple: {},", multiple).unwrap();
  if let Some(default) = default_value
  {
    writeln!(f, "  default: Some(\"{}\"),", escape_string(default)).unwrap();
  }
  else
  {
    writeln!(f, "  default: None,").unwrap();
  }
  writeln!(f, "  sensitive: {},", sensitive).unwrap();
  writeln!(f, "  interactive: {},", interactive).unwrap();
  writeln!(f, "}};").unwrap();

  // Generate kind
  let static_kind = match kind_str
  {
    "String" => "StaticKind::String",
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
    _ => "StaticKind::String", // Default fallback
  };

  // Generate the argument const
  writeln!(f, "const CMD_{}_ARG_{}: StaticArgumentDefinition = StaticArgumentDefinition {{", cmd_index, arg_index).unwrap();
  writeln!(f, "  name: \"{}\",", escape_string(name)).unwrap();
  writeln!(f, "  kind: {},", static_kind).unwrap();
  writeln!(f, "  attributes: CMD_{}_ARG_{}_ATTRS,", cmd_index, arg_index).unwrap();
  writeln!(f, "  hint: \"{}\",", escape_string(hint)).unwrap();
  writeln!(f, "  description: \"{}\",", escape_string(description)).unwrap();
  writeln!(f, "  validation_rules: &[],").unwrap(); // Keep empty for now
  writeln!(f, "  aliases: CMD_{}_ARG_{}_ALIASES,", cmd_index, arg_index).unwrap();
  writeln!(f, "  tags: CMD_{}_ARG_{}_TAGS,", cmd_index, arg_index).unwrap();
  writeln!(f, "}};").unwrap();
  writeln!(f).unwrap();
}

fn generate_string_array(f: &mut BufWriter<File>, const_name: &str, yaml_value: &serde_yaml::Value)
{
  if let Some(array) = yaml_value.as_sequence()
  {
    if array.is_empty()
    {
      writeln!(f, "const {}: &[&str] = &[];", const_name).unwrap();
    }
    else
    {
      writeln!(f, "const {}: &[&str] = &[", const_name).unwrap();
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
    writeln!(f, "const {}: &[&str] = &[];", const_name).unwrap();
  }
}

fn escape_string(s: &str) -> String
{
  s.replace('\\', "\\\\").replace('"', "\\\"")
}