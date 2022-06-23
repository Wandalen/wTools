use super::*;
use wtools::error::BasicError;
use wca::command::Command;
use wca::instruction::Instruction;
use wca::string::parse_request::OpType::Primitive;

//

fn commands_form() -> std::collections::HashMap< String, Command >
{
  let help_command : Command = wca::CommandOptions::default()
  .hint( "Get help." )
  .long_hint( "Get help for command [command]" )
  .phrase( ".help" )
  .subject_hint( "some command" )
  .routine( &| _i : &Instruction | { println!( "this is help" ); Ok( () ) } )
  .form();
  let list_command : Command = wca::CommandOptions::default()
  .hint( "Get list." )
  .long_hint( "Get list of" )
  .phrase( ".list" )
  .subject_hint( "some subject" )
  .routine( &| _i : &Instruction | { println!( "this is list" ); Ok( () ) } )
  .form();
  let err_command : Command = wca::CommandOptions::default()
  .hint( "Error." )
  .long_hint( "Throw error" )
  .phrase( ".error" )
  .routine( &| _i : &Instruction | { Err( BasicError::new( "err" ) ) } )
  .form();

  let commands : std::collections::HashMap< String, Command > = std::collections::HashMap::from
  ([
    ( ".help".to_string(), help_command ),
    ( ".list".to_string(), list_command ),
    ( ".error".to_string(), err_command ),
  ]);
  commands
}

tests_impls!
{
  fn basic()
  {
    let ca = wca::commands_aggregator()
    .form();
    a_id!( ca.base_path, None );
    a_id!( ca.command_prefix, "".to_string() );
    a_id!( ca.delimeter, vec![ ".".to_string(), " ".to_string() ] );
    a_id!( ca.command_explicit_delimeter, ";".to_string() );
    a_id!( ca.command_implicit_delimeter, " ".to_string() );
    a_id!( ca.commands_explicit_delimiting, true );
    a_id!( ca.commands_implicit_delimiting, false );
    a_id!( ca.properties_map_parsing, false );
    a_id!( ca.several_values, true );
    a_id!( ca.with_help, true );
    a_id!( ca.changing_exit_code, true );
    a_id!( ca.commands, std::collections::HashMap::new() );
  }

  //

  fn program_perform_basic()
  {
    /* single command, returns Ok */
    let ca = wca::commands_aggregator()
    .changing_exit_code( false )
    .commands().replace( commands_form() ).end()
    .form();
    let got = ca.program_perform( ".help" );
    a_id!( got, Ok( () ) );

    /* single command, returns Err */
    let ca = wca::commands_aggregator()
    .changing_exit_code( false )
    .commands().replace( commands_form() ).end()
    .form();
    let got = ca.program_perform( ".error" );
    a_id!( got, Err( BasicError::new( "err" ) ) );

    /* */

    /* two commands, explicit delimeter, returns Ok */
    let ca = wca::commands_aggregator()
    .changing_exit_code( false )
    .commands().replace( commands_form() ).end()
    .form();
    let got = ca.program_perform( ".help ; .list" );
    a_id!( got, Ok( () ) );

    /* two commands, explicit delimeter, second command returns Err */
    let ca = wca::commands_aggregator()
    .changing_exit_code( false )
    .commands().replace( commands_form() ).end()
    .form();
    let got = ca.program_perform( ".list ; .error" );
    a_id!( got, Err( BasicError::new( "err" ) ) );

    /* */

    /* two commands, implicit delimeter, returns Ok */
    let ca = wca::commands_aggregator()
    .changing_exit_code( false )
    .commands().replace( commands_form() ).end()
    .form();
    let got = ca.program_perform( ".help .list" );
    a_id!( got, Ok( () ) );

    /* two commands, implicit delimeter, second command returns Err */
    let ca = wca::commands_aggregator()
    .changing_exit_code( false )
    .commands().replace( commands_form() ).end()
    .form();
    let got = ca.program_perform( ".list .error" );
    a_id!( got, Err( BasicError::new( "err" ) ) );
  }

  //

  fn program_perform_with_dotted_paths()
  {
    /* single command with subject as single dot */
    let ca = wca::commands_aggregator()
    .changing_exit_code( false )
    .commands().replace( commands_form() ).end()
    .form();
    let got = ca.program_perform( ".list ." );
    a_id!( got, Ok( () ) );

    /* single command with subject as double dot */
    let ca = wca::commands_aggregator()
    .changing_exit_code( false )
    .commands().replace( commands_form() ).end()
    .form();
    let got = ca.program_perform( ".list .." );
    a_id!( got, Ok( () ) );

    /* single command with subject as dot and slash */
    let ca = wca::commands_aggregator()
    .changing_exit_code( false )
    .commands().replace( commands_form() ).end()
    .form();
    let got = ca.program_perform( ".list ./" );
    a_id!( got, Ok( () ) );

    /* single command with subject as double dot and slash */
    let ca = wca::commands_aggregator()
    .changing_exit_code( false )
    .commands().replace( commands_form() ).end()
    .form();
    let got = ca.program_perform( ".list ../" );
    a_id!( got, Ok( () ) );

    /* single command with subject as dot and backslash */
    let ca = wca::commands_aggregator()
    .changing_exit_code( false )
    .commands().replace( commands_form() ).end()
    .form();
    let got = ca.program_perform( ".list .\\" );
    a_id!( got, Ok( () ) );

    /* single command with subject as double dot and backslash */
    let ca = wca::commands_aggregator()
    .changing_exit_code( false )
    .commands().replace( commands_form() ).end()
    .form();
    let got = ca.program_perform( ".list ..\\" );
    a_id!( got, Ok( () ) );

    /* */

    /* two commands with subjects with dots */
    let ca = wca::commands_aggregator()
    .changing_exit_code( false )
    .commands().replace( commands_form() ).end()
    .form();
    let got = ca.program_perform( ".list ..\\ .help ./some" );
    a_id!( got, Ok( () ) );
  }

  //

  fn instruction_perform_basic()
  {
    /* no commands in aggregator */
    let ca = wca::commands_aggregator()
    .changing_exit_code( false )
    .form();
    let got = ca.instruction_perform( ".help" );
    a_id!( got, Ok( () ) );

    /* command returns Ok */
    let ca = wca::commands_aggregator()
    .changing_exit_code( false )
    .commands().replace( commands_form() ).end()
    .form();
    let got = ca.instruction_perform( ".help" );
    a_id!( got, Ok( () ) );

    /* command returns Err */
    let ca = wca::commands_aggregator()
    .changing_exit_code( false )
    .commands().replace( commands_form() ).end()
    .form();
    let got = ca.instruction_perform( ".error" );
    a_id!( got, Err( BasicError::new( "err" ) ) );
  }

  //

  fn instructions_parse_basic()
  {
    let ca = wca::commands_aggregator()
    .form();

    /* */

    /* empty program */
    let got = ca.instructions_parse( "" );
    a_id!( got, vec![] );

    /* */

    /* single command without subject and map */
    let got = ca.instructions_parse( ".help" );
    let instruction = wca::instruction::Instruction
    {
      err : None,
      command_name : ".help".to_string(),
      subject : "".to_string(),
      properties_map : HashMap::new(),
    };
    a_id!( got, vec![ instruction ] );

    /* single command with subject */
    let got = ca.instructions_parse( ".help command" );
    let instruction = wca::instruction::Instruction
    {
      err : None,
      command_name : ".help".to_string(),
      subject : "command".to_string(),
      properties_map : HashMap::new(),
    };
    a_id!( got, vec![ instruction ] );

    /* single command with subject and map */
    let got = ca.instructions_parse( ".help command v:3" );
    let instruction = wca::instruction::Instruction
    {
      err : None,
      command_name : ".help".to_string(),
      subject : "command".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Primitive( "3".to_string() ) ) ]),
    };
    a_id!( got, vec![ instruction ] );

    /* */

    /* two commands without subject and map, explicit delimeter */
    let got = ca.instructions_parse( ".help ; .version" );
    let instruction1 = wca::instruction::Instruction
    {
      err : None,
      command_name : ".help".to_string(),
      subject : "".to_string(),
      properties_map : HashMap::new(),
    };
    let instruction2 = wca::instruction::Instruction
    {
      err : None,
      command_name : ".version".to_string(),
      subject : "".to_string(),
      properties_map : HashMap::new(),
    };
    a_id!( got, vec![ instruction1, instruction2 ] );

    /* two commands with subject, explicit delimeter */
    let got = ca.instructions_parse( ".help command ; .version delta" );
    let instruction1 = wca::instruction::Instruction
    {
      err : None,
      command_name : ".help".to_string(),
      subject : "command".to_string(),
      properties_map : HashMap::new(),
    };
    let instruction2 = wca::instruction::Instruction
    {
      err : None,
      command_name : ".version".to_string(),
      subject : "delta".to_string(),
      properties_map : HashMap::new(),
    };
    a_id!( got, vec![ instruction1, instruction2 ] );

    /* two commands with subject and map, explicit delimeter */
    let got = ca.instructions_parse( ".help command v:3 ; .version delta n:5" );
    let instruction1 = wca::instruction::Instruction
    {
      err : None,
      command_name : ".help".to_string(),
      subject : "command".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Primitive( "3".to_string() ) ) ]),
    };
    let instruction2 = wca::instruction::Instruction
    {
      err : None,
      command_name : ".version".to_string(),
      subject : "delta".to_string(),
      properties_map : HashMap::from([ ( "n".to_string(), Primitive( "5".to_string() ) ) ]),
    };
    a_id!( got, vec![ instruction1, instruction2 ] );

    /* */

    /* two commands without subject and map, implicit delimeter */
    let got = ca.instructions_parse( ".help .version" );
    let instruction1 = wca::instruction::Instruction
    {
      err : None,
      command_name : ".help".to_string(),
      subject : "".to_string(),
      properties_map : HashMap::new(),
    };
    let instruction2 = wca::instruction::Instruction
    {
      err : None,
      command_name : ".version".to_string(),
      subject : "".to_string(),
      properties_map : HashMap::new(),
    };
    a_id!( got, vec![ instruction1, instruction2 ] );

    /* two commands with subject, implicit delimeter */
    let got = ca.instructions_parse( ".help command .version delta" );
    let instruction1 = wca::instruction::Instruction
    {
      err : None,
      command_name : ".help".to_string(),
      subject : "command".to_string(),
      properties_map : HashMap::new(),
    };
    let instruction2 = wca::instruction::Instruction
    {
      err : None,
      command_name : ".version".to_string(),
      subject : "delta".to_string(),
      properties_map : HashMap::new(),
    };
    a_id!( got, vec![ instruction1, instruction2 ] );

    /* two commands with subject and map, implicit delimeter */
    let got = ca.instructions_parse( ".help command v:3 .version delta n:5" );
    let instruction1 = wca::instruction::Instruction
    {
      err : None,
      command_name : ".help".to_string(),
      subject : "command".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Primitive( "3".to_string() ) ) ]),
    };
    let instruction2 = wca::instruction::Instruction
    {
      err : None,
      command_name : ".version".to_string(),
      subject : "delta".to_string(),
      properties_map : HashMap::from([ ( "n".to_string(), Primitive( "5".to_string() ) ) ]),
    };
    a_id!( got, vec![ instruction1, instruction2 ] );
  }
}

//

tests_index!
{
  basic,
  program_perform_basic,
  program_perform_with_dotted_paths,
  instruction_perform_basic,
  instructions_parse_basic,
}

