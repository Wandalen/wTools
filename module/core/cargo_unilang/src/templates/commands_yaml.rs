//! commands.yaml template generator

/// Generate commands.yaml content for unilang project
pub fn commands_yaml_minimal() -> &'static str
{
r#"# Example command definitions for your unilang CLI
#
# This file is automatically discovered and processed by unilang's build.rs
# You DON'T need to:
# - Create your own build.rs
# - Add serde_yaml dependency
# - Write YAML parsing code
# - Generate PHF maps manually
#
# Unilang does ALL of this automatically when you add the dependency.

- name: ".greet"
  namespace: ""
  description: "Greet someone"
  hint: "Says hello to a person"
  status: "stable"
  version: "1.0.0"
  idempotent: true
  arguments:
    - name: "name"
      kind: "String"
      description: "Name of the person to greet"
      hint: "Your name"
      attributes:
        optional: true
        default: "World"
      validation_rules: []
      aliases: [ "n" ]
      tags: []

- name: ".help"
  namespace: ""
  description: "Show help information"
  hint: "Display available commands"
  status: "stable"
  version: "1.0.0"
  idempotent: true
  arguments: []
  validation_rules: []
  aliases: []
  tags: [ "help" ]
"#
}

/// Generate commands.yaml content for full project
pub fn commands_yaml_full() -> &'static str
{
r#"# Full-featured command definitions
#
# Demonstrates more advanced unilang features

- name: ".greet"
  namespace: ""
  description: "Greet someone with customization"
  hint: "Says hello with options"
  status: "stable"
  version: "1.0.0"
  idempotent: true
  arguments:
    - name: "name"
      kind: "String"
      description: "Name of the person to greet"
      hint: "Your name"
      attributes:
        optional: false
      validation_rules:
        - "Must not be empty"
      aliases: [ "n" ]
      tags: []

    - name: "style"
      kind: "String"
      description: "Greeting style: formal or casual"
      hint: "How to greet"
      attributes:
        optional: true
        default: "casual"
      validation_rules:
        - "Must be 'formal' or 'casual'"
      aliases: [ "s" ]
      tags: []

- name: ".echo"
  namespace: ""
  description: "Echo a message"
  hint: "Repeat what you say"
  status: "stable"
  version: "1.0.0"
  idempotent: true
  arguments:
    - name: "message"
      kind: "String"
      description: "Message to echo"
      hint: "Your message"
      attributes:
        optional: false
      validation_rules: []
      aliases: [ "m" ]
      tags: []

- name: ".help"
  namespace: ""
  description: "Show help information"
  hint: "Display available commands"
  status: "stable"
  version: "1.0.0"
  idempotent: true
  arguments: []
  validation_rules: []
  aliases: []
  tags: [ "help" ]
"#
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_commands_yaml_minimal_has_example_command()
  {
    let content = commands_yaml_minimal();
    assert!( content.contains( ".greet" ) );
    assert!( content.contains( ".help" ) );
    assert!( content.contains( "Unilang does ALL of this automatically" ) );
  }

  #[test]
  fn test_commands_yaml_full_has_multiple_commands()
  {
    let content = commands_yaml_full();
    assert!( content.contains( ".greet" ) );
    assert!( content.contains( ".echo" ) );
    assert!( content.contains( ".help" ) );
  }
}
