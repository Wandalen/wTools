<!-- {{# generate.module_header{} #}} -->

# Module :: unilang
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_unilang_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_unilang_push.yml) [![docs.rs](https://img.shields.io/docsrs/unilang?color=e3e8f0&logo=docs.rs)](https://docs.rs/unilang) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fmove%2Funilang%2Fexamples%2Ffull_cli_example.rs,RUN_POSTFIX=--example%20full_cli_example/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

A universal command framework that lets you define command-line interfaces once and deploy them across multiple interaction paradigms — CLI, TUI, GUI, Web APIs, and more.

## Why unilang?

When building command-line tools, you often face these challenges:
- **Repetitive Code**: Defining argument parsing, validation, and help generation for each command
- **Inconsistent APIs**: Different interaction modes (CLI vs Web API) require separate implementations
- **Limited Extensibility**: Hard to add new commands or change existing ones without major refactoring
- **Poor User Experience**: Inconsistent help messages, error handling, and command organization

**unilang** solves these problems by providing:
- 📝 **Single Definition**: Define commands once, use everywhere
- 🔧 **Multiple Modalities**: Same commands work as CLI, Web API, or programmatic API
- 🏗️ **Modular Architecture**: Easy to add, modify, or remove commands
- 🎯 **Type Safety**: Strong typing with comprehensive validation
- 📚 **Auto Documentation**: Help text and command discovery built-in
- 🔍 **Rich Validation**: Built-in validators for common patterns

## Quick Start

### Installation

```sh
cargo add unilang
```

### Basic Example

Here's a simple "Hello World" command:

```rust,ignore
use unilang::prelude::*;

fn main() -> Result< (), unilang::Error >
{
  // Create a command registry
  let mut registry = CommandRegistry::new();
  
  // Define a simple greeting command
  let greet_cmd = CommandDefinition
  {
    name : "greet".to_string(),
    namespace : String::new(),  // Global namespace
    description : "A friendly greeting command".to_string(),
    hint : "Says hello to someone".to_string(),
    arguments : vec!
    [
      ArgumentDefinition
      {
        name : "name".to_string(),
        description : "Name of the person to greet".to_string(),
        kind : Kind::String,
        hint : "Your name".to_string(),
        attributes : ArgumentAttributes
        {
          optional : true,
          default : Some( "World".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "n".to_string() ],
        tags : vec![],
      }
    ],
    // ... other fields with defaults
    aliases : vec![ "hello".to_string() ],
    status : "stable".to_string(),
    version : "1.0.0".to_string(),
    ..Default::default()
  };
  
  // Define the command's execution logic
  let greet_routine = Box::new( | cmd : VerifiedCommand, _ctx : ExecutionContext |
  {
    let name = match cmd.arguments.get( "name" )
    {
      Some( Value::String( s ) ) => s.clone(),
      _ => "World".to_string(),
    };
    
    println!( "Hello, {}!", name );
    
    Ok( OutputData
    {
      content : format!( "Hello, {}!", name ),
      format : "text".to_string(),
    })
  });
  
  // Register the command
  registry.command_add_runtime( &greet_cmd, greet_routine )?;
  
  // Use the Pipeline API to execute commands
  let pipeline = Pipeline::new( registry );
  
  // Execute a command
  let result = pipeline.process_command_simple( "greet name::Alice" );
  println!( "Success: {}", result.success );
  println!( "Output: {}", result.outputs[ 0 ].content );
  
  Ok(())
}
```

Run this example:
```sh
cargo run --example 01_basic_command_registration
```

## Core Concepts

### 1. Command Registry
The central hub that stores and manages all command definitions and their execution routines.

```rust
use unilang::prelude::*;
let mut registry = CommandRegistry::new();
// registry is now ready to use
```

### 2. Command Definition
Describes a command's metadata, arguments, and behavior.

```rust
use unilang::prelude::*;
let command = CommandDefinition
{
  name : "my-command".to_string(),
  namespace : ".tools".to_string(),  // Hierarchical namespace
  description : "Does something useful".to_string(),
  arguments : vec![],
  routine_link : None,
  hint : String::new(),
  status : "stable".to_string(),
  version : "1.0.0".to_string(),
  tags : vec![],
  aliases : vec![],
  permissions : vec![],
  idempotent : false,
  deprecation_message : String::new(),
  http_method_hint : String::new(),
  examples : vec![],
};
// command definition is complete
assert_eq!(command.name, "my-command");
```

### 3. Argument Types
unilang supports rich argument types with automatic parsing and validation:

- **Basic Types**: `String`, `Integer`, `Float`, `Boolean`
- **Path Types**: `Path`, `File`, `Directory`
- **Complex Types**: `Url`, `DateTime`, `Pattern` (regex)
- **Collections**: `List<T>`, `Map<K,V>`
- **Special Types**: `Enum` (choices), `JsonString`, `Object`

### 4. Validation Rules
Built-in validators ensure arguments meet requirements:

```rust
use unilang::prelude::*;
use unilang::ValidationRule;
let validation_rules : Vec<ValidationRule> = vec!
[
  ValidationRule::Min( 0.0 ),      // Minimum value
  ValidationRule::Max( 100.0 ),    // Maximum value
  ValidationRule::MinLength( 3 ),  // Minimum string length
  ValidationRule::Pattern( "^[A-Z]".to_string() ), // Regex pattern
];
assert_eq!(validation_rules.len(), 4);
```

### 5. Command Execution Pipeline
The execution flow: Parse → Validate → Execute

```rust
use unilang::prelude::*;
let registry = CommandRegistry::new();
let pipeline = Pipeline::new( registry );
let result = pipeline.process_command_simple( "my-command arg1::value" );
// result contains the execution outcome
```

## Examples

### Working with Different Argument Types

```rust
use unilang::prelude::*;
use unilang::ValidationRule;
// See examples/02_argument_types.rs for the full example
let command = CommandDefinition
{
  name : "demo".to_string(),
  description : "Demo command with various argument types".to_string(),
  arguments : vec!
  [
    // String with validation
    ArgumentDefinition
    {
      name : "username".to_string(),
      kind : Kind::String,
      attributes : ArgumentAttributes::default(),
      hint : "User identifier".to_string(),
      description : "Username for the operation".to_string(),
      validation_rules : vec!
      [
        ValidationRule::MinLength( 3 ),
        ValidationRule::Pattern( "^[a-zA-Z0-9_]+$".to_string() ),
      ],
      aliases : vec![],
      tags : vec![],
    },
    // Optional integer with range
    ArgumentDefinition
    {
      name : "age".to_string(),
      kind : Kind::Integer,
      attributes : ArgumentAttributes
      {
        optional : true,
        ..ArgumentAttributes::default()
      },
      hint : "Age in years".to_string(),
      description : "Person's age".to_string(),
      validation_rules : vec!
      [
        ValidationRule::Min( 0.0 ),
        ValidationRule::Max( 150.0 ),
      ],
      aliases : vec![],
      tags : vec![],
    },
    // File path that must exist
    ArgumentDefinition
    {
      name : "config".to_string(),
      kind : Kind::File,
      attributes : ArgumentAttributes::default(),
      hint : "Configuration file".to_string(),
      description : "Path to config file".to_string(),
      validation_rules : vec![],
      aliases : vec![],
      tags : vec![],
    },
  ],
  routine_link : None,
  namespace : String::new(),
  hint : "Demonstration command".to_string(),
  status : "stable".to_string(),
  version : "1.0.0".to_string(),
  tags : vec![],
  aliases : vec![],
  permissions : vec![],
  idempotent : false,
  deprecation_message : String::new(),
  http_method_hint : String::new(),
  examples : vec![],
};
assert_eq!(command.name, "demo");
```

Run the argument types demo:
```sh
cargo run --example 02_argument_types
```

### Using Collections

```rust
use unilang::prelude::*;
// See examples/03_collection_types.rs for the full example
// List of strings with custom delimiter
let _tags_arg = ArgumentDefinition
{
  name : "tags".to_string(),
  kind : Kind::List( Box::new( Kind::String ), Some( ',' ) ), // comma-separated
  attributes : ArgumentAttributes::default(),
  hint : "Comma-separated tags".to_string(),
  description : "List of tags".to_string(),
  validation_rules : vec![],
  aliases : vec![],
  tags : vec![],
};

// Map with custom delimiters
let _options_arg = ArgumentDefinition
{
  name : "options".to_string(),
  kind : Kind::Map
  (
    Box::new( Kind::String ),  // key type
    Box::new( Kind::String ),  // value type
    Some( ',' ),               // entry delimiter
    Some( '=' )                // key-value delimiter
  ),
  // Usage: options::debug=true,verbose=false
  attributes : ArgumentAttributes::default(),
  hint : "Key-value options".to_string(),
  description : "Configuration options".to_string(),
  validation_rules : vec![],
  aliases : vec![],
  tags : vec![],
};
assert_eq!(_tags_arg.name, "tags");
```

Run the collections demo:
```sh
cargo run --example 03_collection_types
```

### Namespaces and Command Organization

```rust
use unilang::prelude::*;
// See examples/05_namespaces_and_aliases.rs for the full example
// Commands can be organized hierarchically
let commands = vec!
[
  CommandDefinition
  {
    name : "list".to_string(),
    namespace : ".file".to_string(),  // Access as: file.list
    description : "List files".to_string(),
    arguments : vec![],
    routine_link : None,
    hint : "List files".to_string(),
    status : "stable".to_string(),
    version : "1.0.0".to_string(),
    tags : vec![],
    aliases : vec![],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : "GET".to_string(),
    examples : vec![],
  },
  CommandDefinition
  {
    name : "create".to_string(),
    namespace : ".file".to_string(),  // Access as: file.create
    description : "Create files".to_string(),
    arguments : vec![],
    routine_link : None,
    hint : "Create files".to_string(),
    status : "stable".to_string(),
    version : "1.0.0".to_string(),
    tags : vec![],
    aliases : vec![],
    permissions : vec![],
    idempotent : false,
    deprecation_message : String::new(),
    http_method_hint : "POST".to_string(),
    examples : vec![],
  },
];
assert_eq!(commands.len(), 2);
```

### Loading Commands from YAML/JSON

```rust,ignore
// See examples/07_yaml_json_loading.rs for the full example
use unilang::loader::{ load_from_yaml_file, load_from_json_str };
use unilang::prelude::*;

// Load from YAML file
let mut registry = CommandRegistry::new();
let commands = load_from_yaml_file( "commands.yaml" )?;
for cmd in commands
{
  registry.commands.insert( cmd.name.clone(), cmd );
}

// Or from JSON string
let json = r#"[
{
  "name" : "test",
  "description" : "Test command",
  "arguments" : []
}]"#;
let commands = load_from_json_str( json )?;
```

## Command-Line Usage Patterns

unilang supports flexible command-line syntax:

```sh
# Named arguments (recommended)
command arg1::value1 arg2::value2

# Positional arguments
command value1 value2

# Mixed (positional first, then named)
command value1 arg2::value2

# With namespaces
namespace.command arg::value

# Using aliases
cmd arg::value  # If 'cmd' is an alias for 'command'
```

## Advanced Features

### Custom Validation

```rust
use unilang::prelude::*;
use unilang::ValidationRule;
// Create complex validation rules
let password_arg = ArgumentDefinition
{
  name : "password".to_string(),
  kind : Kind::String,
  attributes : ArgumentAttributes
  {
    sensitive : true,  // Won't be logged or shown in history
    ..ArgumentAttributes::default()
  },
  hint : "Secure password".to_string(),
  description : "User password with complexity requirements".to_string(),
  validation_rules : vec!
  [
    ValidationRule::MinLength( 8 ),
    ValidationRule::Pattern( r"^(?=.*[A-Za-z])(?=.*\d)".to_string() ), // Letters and numbers
  ],
  aliases : vec![],
  tags : vec![],
};
assert!(password_arg.attributes.sensitive);
```

### Batch Processing

```rust
use unilang::prelude::*;
let registry = CommandRegistry::new();
let pipeline = Pipeline::new(registry);
// Process multiple commands efficiently
let commands = vec!
[
  "file.create name::test.txt",
  "file.write name::test.txt content::'Hello'",
  "file.list pattern::*.txt",
];

let batch_result = pipeline.process_batch( &commands, ExecutionContext::default() );
// Success rate will be 0% since no commands are registered
assert_eq!(batch_result.success_rate(), 0.0);
```

### Help System

```rust
use unilang::prelude::*;
let registry = CommandRegistry::new();
// Automatic help generation
let help_gen = HelpGenerator::new( &registry );

// List all commands (will be empty for new registry)
let commands_list = help_gen.list_commands();
assert!(commands_list.len() > 0); // Always contains header

// Get help for specific command (returns None if not found)
let help = help_gen.command( "greet" );
assert!(help.is_none()); // No commands registered yet
```

## Full CLI Example

For a complete example showing all features, check out:

```sh
# Run the full CLI example
cargo run --example full_cli_example -- greet name::Alice

# See available commands
cargo run --example full_cli_example -- help

# Get help for a specific command
cargo run --example full_cli_example -- help greet
```

## API Modes

unilang can be used in different ways:

### 1. Pipeline API (Recommended)
High-level API that handles the full command execution pipeline:

```rust
use unilang::prelude::*;
let registry = CommandRegistry::new();
let pipeline = Pipeline::new( registry );
let result = pipeline.process_command_simple( "command arg::value" );
// Result will indicate command not found since no commands are registered
assert!(!result.success);
```

### 2. Component API
Lower-level access to individual components:

```rust,ignore
use unilang::prelude::*;
# let registry = CommandRegistry::new();
# let input = "example";
# let mut context = ExecutionContext::default();
// Parse
let parser = Parser::new( Default::default() );
let instruction = parser.parse_single_instruction( input )?;

// Analyze
let analyzer = SemanticAnalyzer::new( &[ instruction ], &registry );
let commands = analyzer.analyze()?;

// Execute
let interpreter = Interpreter::new( &commands, &registry );
interpreter.run( &mut context )?;
```

### 3. Direct Integration
For maximum control:

```rust,ignore
use unilang::prelude::*;
# let registry = CommandRegistry::new();
# let verified_command = todo!();
# let context = ExecutionContext::default();
// Direct command execution
let routine = registry.routines.get( ".namespace.command" ).unwrap();
let result = routine( verified_command, context )?;
```

## Error Handling

unilang provides comprehensive error handling:

```rust
use unilang::prelude::*;
let registry = CommandRegistry::new();
let pipeline = Pipeline::new(registry);
let input = "example";
match pipeline.process_command_simple( input )
{
  result if result.success =>
  {
    println!( "Output: {}", result.outputs[ 0 ].content );
  }
  result =>
  {
    if let Some( _error ) = result.error
    {
      // Error handling - command not found since no commands registered
      assert!(!result.success);
    }
  }
}
```

## More Examples

Explore the `examples/` directory for more detailed examples:

- `01_basic_command_registration.rs` - Getting started
- `02_argument_types.rs` - All supported argument types
- `03_collection_types.rs` - Lists and maps
- `04_validation_rules.rs` - Input validation
- `05_namespaces_and_aliases.rs` - Command organization
- `06_help_system.rs` - Automatic help generation
- `07_yaml_json_loading.rs` - Loading commands from files
- `08_semantic_analysis_simple.rs` - Understanding the analysis phase
- `09_command_execution.rs` - Execution patterns
- `10_full_pipeline.rs` - Complete pipeline example
- `11_pipeline_api.rs` - Pipeline API features
- `full_cli_example.rs` - Full-featured CLI application

## Contributing

See [CONTRIBUTING.md](https://github.com/Wandalen/wTools/blob/master/CONTRIBUTING.md) for details.

## License

Licensed under MIT license ([LICENSE](LICENSE) or https://opensource.org/licenses/MIT)