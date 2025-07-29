//! This is a basic CLI application for the `unilang` module.
//! It demonstrates how to initialize the command registry,
//! parse command-line arguments, and execute commands.

use std::collections::HashMap;
use unilang::data::ArgumentAttributes;
use unilang::registry::CommandRegistry;
use unilang::data::{CommandDefinition, ArgumentDefinition, OutputData};
use unilang::data::Kind as ArgumentKind;
use unilang_parser::{Parser, UnilangParserOptions};
use unilang::semantic::SemanticAnalyzer;
use unilang::interpreter::{Interpreter, ExecutionContext};

use unilang::help::HelpGenerator;
use unilang::registry::CommandRoutine; // Import CommandRoutine
use unilang::types::Value; // Import Value

fn main() {
  if let Err(err) = run() {
    eprintln!("Error: {err}");
    std::process::exit(1);
  }
}

fn run() -> Result<(), unilang::error::Error> {
  // 1. Initialize Command Registry
  let mut registry = CommandRegistry::new();

  // 2. Define and Register Commands with Routines

  // .math.add command
  let math_add_def = CommandDefinition::former()
    .name("add")
    .namespace(".math".to_string()) // Changed to String
    .description("Adds two numbers.".to_string())
    .hint("Adds two numbers.")
    .status("stable")
    .version("1.0.0".to_string())
    .aliases(vec!["sum".to_string(), "plus".to_string()])
    .tags(vec!["math".to_string(), "calculation".to_string()])
    .permissions(vec![]) // Added
    .idempotent(true) // Added
    .deprecation_message(String::new()) // Added
    .http_method_hint(String::new()) // Added
    .examples(vec![]) // Added
    .arguments(vec![
      ArgumentDefinition::former()
        .name("a")
        .kind(ArgumentKind::Integer)
        .hint("First number.")
        .end(),
      ArgumentDefinition::former()
        .name("b")
        .kind(ArgumentKind::Integer)
        .hint("Second number.")
        .end(),
    ])
    .end();

  let math_add_routine: CommandRoutine = Box::new(|cmd, _ctx| {
    let a = cmd.arguments.get("a").unwrap();
    let b = cmd.arguments.get("b").unwrap();
    if let (Value::Integer(val_a), Value::Integer(val_b)) = (a, b) {
      let result = val_a + val_b;
      println!("Result: {result}");
      return Ok(OutputData {
        content: result.to_string(),
        format: "text".to_string(),
      });
    }

    unreachable!();
  });
  registry.command_add_runtime(&math_add_def, math_add_routine)?;

  // .math.sub command
  let math_sub_def = CommandDefinition::former()
    .name("sub")
    .namespace(".math".to_string()) // Changed to String
    .description("Subtracts two numbers.".to_string())
    .hint("Subtracts two numbers.")
    .status("beta")
    .version("0.9.0".to_string())
    .aliases(vec!["minus".to_string()])
    .permissions(vec![]) // Added
    .idempotent(true) // Added
    .deprecation_message(String::new()) // Added
    .http_method_hint(String::new()) // Added
    .examples(vec![]) // Added
    .arguments(vec![
      ArgumentDefinition::former()
        .name("x")
        .kind(ArgumentKind::Integer)
        .hint("Minuend.")
        .end(),
      ArgumentDefinition::former()
        .name("y")
        .kind(ArgumentKind::Integer)
        .hint("Subtrahend.")
        .end(),
    ])
    .end();

  let math_sub_routine: CommandRoutine = Box::new(|cmd, _ctx| {
    let x = cmd.arguments.get("x").unwrap();

    let y = cmd.arguments.get("y").unwrap();

    if let (Value::Integer(val_x), Value::Integer(val_y)) = (x, y) {
      let result = val_x - val_y;
      println!("Result: {result}");
      return Ok(OutputData {
        content: result.to_string(),
        format: "text".to_string(),
      });
    }
    unreachable!();
  });
  registry.command_add_runtime(&math_sub_def, math_sub_routine)?;

  // .greet command
  let greet_def = CommandDefinition::former()
    .name("greet")
    .namespace(String::new()) // Changed to String (global namespace)
    .description("Greets the specified person.".to_string())
    .hint("Greets the specified person.")
    .status("stable")
    .version("1.0.0".to_string())
    .aliases(vec!["hi".to_string()]) // Added alias for testing
    .permissions(vec![]) // Added
    .idempotent(true) // Added
    .deprecation_message(String::new()) // Added
    .http_method_hint(String::new()) // Added
    .examples(vec!["greet John".to_string(), "greet".to_string()]) // Added
    .arguments(vec![ArgumentDefinition::former()
      .name("name")
      .kind(ArgumentKind::String)
      .hint("Name of the person to greet.")
      .default_value("World".to_string())
      .attributes(ArgumentAttributes::former().optional(true).is_default_arg(true).end())
      .end()])
    .end();

  let greet_routine: CommandRoutine = Box::new(|cmd, _ctx| {
    let name = match cmd.arguments.get("name") {
      Some(Value::String(s)) => s.clone(),
      _ => "World".to_string(),
    };
    let result = format!("Hello, {name}!");

    println!("{result}");
    Ok(OutputData {
      content: result,
      format: "text".to_string(),
    })
  });
  registry.command_add_runtime(&greet_def, greet_routine)?;

  // .config.set command
  let config_set_def = CommandDefinition::former()
    .name("set")
    .namespace(".config".to_string()) // Changed to String
    .description("Sets a configuration value.".to_string())
    .hint("Sets a configuration value.")
    .status("experimental")
    .version("0.1.0".to_string())
    .aliases(vec![]) // Added
    .permissions(vec![]) // Added
    .idempotent(false) // Added
    .deprecation_message(String::new()) // Added
    .http_method_hint(String::new()) // Added
    .examples(vec![]) // Added
    .arguments(vec![
      ArgumentDefinition::former()
        .name("key")
        .kind(ArgumentKind::String)
        .hint("Configuration key.")
        .end(),
      ArgumentDefinition::former()
        .name("value")
        .kind(ArgumentKind::String)
        .hint("Configuration value.")
        .attributes(ArgumentAttributes::former().interactive(true).sensitive(true).end())
        .end(),
    ])
    .end();

  let config_set_routine: CommandRoutine = Box::new(|cmd, _ctx| {
    let key = cmd.arguments.get("key").unwrap();

    let value = cmd.arguments.get("value").unwrap();
    let result = format!("Setting config: {key} = {value}");
    println!("{result}");
    Ok(OutputData {
      content: result,
      format: "text".to_string(),
    })
  });
  registry.command_add_runtime(&config_set_def, config_set_routine)?;

  // .system.echo command
  let echo_def = CommandDefinition::former()
    .name("echo")
    .namespace(".system".to_string()) // Changed to String
    .description("Echoes a message".to_string())
    .hint("Echoes back the provided arguments.".to_string())
    .status("stable".to_string())
    .version("1.0.0".to_string())
    .tags(vec!["utility".to_string()]) // Added tag for testing
    .aliases(vec!["e".to_string()])
    .permissions(vec!["admin".to_string()]) // Added permission for testing
    .idempotent(true)
    .deprecation_message(String::new()) // Added
    .http_method_hint(String::new()) // Added
    .examples(vec!["system.echo \"Hello\"".to_string()]) // Added
    .arguments(vec![
      ArgumentDefinition::former()
        .name("arg1")
        .kind(ArgumentKind::String)
        .hint("The first argument to echo.")
        .attributes(ArgumentAttributes::former().optional(true).form())
        .end(),
    ])
    .routine_link(".system.echo".to_string())
    .form();

  let echo_routine: CommandRoutine = Box::new(|_cmd, _ctx| {
    println!("Echo command executed!");
    Ok(OutputData {
      content: "Echo command executed!\n".to_string(),
      format: "text".to_string(),
    })
  });
  registry.command_add_runtime(&echo_def, echo_routine)?;

  // .files.cat command
  let cat_def = CommandDefinition::former()
    .name("cat")
    .namespace(".files".to_string()) // Changed to String
    .description("Read and display file contents".to_string())
    .hint("Print file contents to stdout".to_string())
    .status("stable".to_string())
    .version("1.0.0".to_string())
    .tags(vec!["filesystem".to_string()]) // Added tag for testing
    .aliases(vec!["type".to_string()]) // Added alias for testing
    .permissions(vec!["read_file".to_string()]) // Added permission for testing
    .idempotent(true)
    .deprecation_message(String::new()) // Added
    .http_method_hint(String::new()) // Added
    .examples(vec!["files.cat path::/etc/hosts".to_string()]) // Added
    .arguments(vec![ArgumentDefinition::former()
      .name("path")
      .description("The path to the file to read".to_string())
      .hint("File path".to_string())
      .kind(ArgumentKind::String)
      .aliases(vec!["p".to_string()]) // Added alias for testing
      .tags(vec!["required".to_string()]) // Added tag for testing
      .attributes(
        ArgumentAttributes::former()
          .optional(false)
          .interactive(false)
          .sensitive(false)
          .form(),
      )
      .form()])
    .routine_link(".files.cat".to_string())
    .form();

  let cat_routine: CommandRoutine = Box::new(|cmd, _ctx| {
    let path = cmd.arguments.get("path").unwrap();
    if let Value::String(path_str) = path {
      if let Ok(contents) = std::fs::read_to_string(path_str) {
        println!("{contents}");
        Ok(OutputData {
          content: contents,
          format: "text".to_string(),
        })
      } else {
        let error_msg = format!("Failed to read file: {path_str}");
        Err(unilang::data::ErrorData {
          code: "FILE_READ_ERROR".to_string(),
          message: error_msg,
        })
      }
    } else {
      Err(unilang::data::ErrorData {
        code: "INVALID_ARGUMENT_TYPE".to_string(),
        message: "Path must be a string".to_string(),
      })
    }
  });
  registry.command_add_runtime(&cat_def, cat_routine)?;

  // 3. Parse Command Line Arguments
  let args: Vec<String> = std::env::args().skip(1).collect();

  // Handle case when no arguments are provided
  if args.is_empty() {
    let help_generator = HelpGenerator::new(&registry);
    let help_text = help_generator.list_commands();
    println!("{help_text}");
    eprintln!("Usage: unilang_cli <command> [args...]");
    eprintln!("Examples:");
    eprintln!("  unilang_cli greet \"Alice\"");
    eprintln!("  unilang_cli math.add 10 20");
    eprintln!("  unilang_cli help greet");
    eprintln!("Note: String arguments must be quoted, e.g., \"Alice\" not Alice");
    return Ok(());
  }

  let parser = Parser::new(UnilangParserOptions::default());

  // Build alias map for CLI resolution
  let mut alias_map: HashMap<String, String> = HashMap::new();
  for (full_name, cmd_def) in &registry.commands {
    for alias in &cmd_def.aliases {
      alias_map.insert(alias.clone(), full_name.clone());
    }
  }

  let mut processed_args = args.clone();
  if let Some(first_arg) = processed_args.first_mut() {
    if let Some(canonical_name) = alias_map.get(first_arg) {
      *first_arg = canonical_name.clone();
    }
  }

  // Handle '--help' flag
  if processed_args.first().is_some_and(|arg| arg == "--help") {
    let help_generator = HelpGenerator::new(&registry);
    println!("{}", help_generator.list_commands());
    return Ok(());
  }

  // Handle 'help' command manually
  if processed_args.first().is_some_and(|arg| arg == "help") {
    let help_generator = HelpGenerator::new(&registry);
    if processed_args.len() > 2 {
      eprintln!("Error: Invalid usage of help command. Use `help` or `help <command_name>`.");
      std::process::exit(1);
    } else if let Some(command_name) = processed_args.get(1) {
      if let Some(help_text) = help_generator.command(command_name) {
        println!("{help_text}");
      } else {
        eprintln!("Error: Command '{command_name}' not found for help.");
        std::process::exit(1);
      }
    } else {
      println!("{}", help_generator.list_commands());
    }
    return Ok(());
  }

  let command_input_str = processed_args.join(" ");
  let instruction = parser.parse_single_instruction(&command_input_str)?;
  let instructions = &[instruction][..];

  // 4. Semantic Analysis
  let semantic_analyzer = SemanticAnalyzer::new(instructions, &registry);
  let commands = semantic_analyzer.analyze()?;

  // 5. Interpret and Execute
  let interpreter = Interpreter::new(&commands, &registry);
  let mut context = ExecutionContext::default();
  interpreter.run(&mut context)?;

  Ok(())
}
