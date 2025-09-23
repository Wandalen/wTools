use unilang::data::{ ArgumentDefinition, CommandDefinition, Kind };
use unilang::interpreter::{ ExecutionContext, Interpreter };
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang::types::Value;
use unilang_parser::{ Parser, UnilangParserOptions };

#[ test ]
fn basic_integration_test()
{
  // Test Matrix Row: T3.1
  // Placeholder for a basic integration test
  // This test will call a public function from the unilang crate.
  // assert_eq!( unilang::some_public_function(), expected_value );
}

#[ test ]
fn basic_integration_test_with_new_parser()
{
  // Test Matrix Row: T3.1
  #[allow(deprecated)]
  #[allow(deprecated)]
    let mut registry = CommandRegistry::new();
  registry.register( CommandDefinition
  {
    name : "add".to_string(),
    description : "Adds two numbers".to_string(),
    arguments : vec!
    [
      ArgumentDefinition
      {
        name : "a".to_string(),
        description : "First number".to_string(),
        kind : Kind::Integer,
        optional : false,
        multiple : false,
        validation_rules : vec![],
        hint : "".to_string(),
        default_value : None,
        aliases : vec![],
        tags : vec![],
        attributes : unilang::data::ArgumentAttributes::former().form(),
      },
      ArgumentDefinition
      {
        name : "b".to_string(),
        description : "Second number".to_string(),
        kind : Kind::Integer,
        optional : false,
        multiple : false,
        validation_rules : vec![],
        hint : "".to_string(),
        default_value : None,
        aliases : vec![],
        tags : vec![],
        attributes : unilang::data::ArgumentAttributes::former().form(),
      },
    ],
    routine_link : Some( "add_routine".to_string() ),
    namespace : "".to_string(),
    hint : "".to_string(),
    status : "".to_string(),
    version : "".to_string(),
    tags : vec![],
    aliases : vec![],
    permissions : vec![],
    idempotent : false,
    deprecation_message : "".to_string(),
    http_method_hint : "".to_string(),
    examples : vec![],
  });

  let add_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx : ExecutionContext | -> Result< unilang::data::OutputData, unilang::data::ErrorData >
  {
    let a = cmd.arguments.get( "a" ).unwrap().as_integer().unwrap();
    let b = cmd.arguments.get( "b" ).unwrap().as_integer().unwrap();
    Ok( unilang::data::OutputData { content : ( a + b ).to_string(), format : "text".to_string() } )
  });
  #[allow(deprecated)]
    #[allow(deprecated)]
    registry.command_add_runtime( &registry.get( "add" ).unwrap(), add_routine ).unwrap();

  let parser = Parser::new( UnilangParserOptions::default() );
  let input = "add 5 3";
  let instructions = parser.parse_single_str( input ).unwrap();
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let verified = analyzer.analyze().unwrap();
  let interpreter = Interpreter::new( &verified, &registry );
  let mut context = ExecutionContext::default();
  let result = interpreter.run( &mut context ).unwrap();

  assert_eq!( result.len(), 1 );
  assert_eq!( result[ 0 ].content, "8" );
}