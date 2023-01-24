use super::*;
use wtools::error::BasicError;
use wca::command::Command;
use wca::string::parse_request::OpType::Primitive;
use wca::
{
  Args,
  NoProperties,
};

//

fn commands_form() -> HashMap< String, Command >
{
  let help_command : Command = wca::Command::former()
  .hint( "Get help." )
  .long_hint( "Get help for command [command]" )
  .phrase( ".help" )
  .subject_hint( "some command" )
  .routine( | _ : Args< String, NoProperties > | { println!( "this is help" ); Ok( () ) } )
  .form();
  let list_command : Command = wca::Command::former()
  .hint( "Get list." )
  .long_hint( "Get list of" )
  .phrase( ".list" )
  .subject_hint( "some subject" )
  .routine( | _ : Args< String, NoProperties > | { println!( "this is list" ); Ok( () ) } )
  .form();
  let err_command : Command = wca::Command::former()
  .hint( "Error." )
  .long_hint( "Throw error" )
  .phrase( ".error" )
  .routine( | _ : Args< String, NoProperties > | { Err( BasicError::new( "err" ) ) } )
  .form();

  let commands : HashMap< String, Command > = std::collections::HashMap::from
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
    a_id!( ca.delimiter, ".".to_string() );
    a_id!( ca.with_help, true );
    a_id!( ca.exit_code_on_error, None );
    a_id!( ca.commands, std::collections::HashMap::new() );
  }

  //

  fn program_perform_basic()
  {
    /* single command, returns Ok */
    let ca = wca::commands_aggregator()
    .commands( commands_form() )
    .form();
    let got = ca.program_perform( ".help" );
    a_id!( got, Ok( () ) );

    /* single command, returns Err */
    let ca = wca::commands_aggregator()
    .commands( commands_form() )
    .form();
    let got = ca.program_perform( ".error" );
    a_id!( got, Err( BasicError::new( "err" ) ) );

    /* */

    /* two commands, explicit delimeter, returns Ok */
    let ca = wca::commands_aggregator()
    .commands( commands_form() )
    .form();
    let got = ca.program_perform( ".help ; .list" );
    a_id!( got, Ok( () ) );

    /* two commands, explicit delimeter, second command returns Err */
    let ca = wca::commands_aggregator()
    .commands( commands_form() )
    .form();
    let got = ca.program_perform( ".list ; .error" );
    a_id!( got, Err( BasicError::new( "err" ) ) );

    /* */

    /* two commands, implicit delimeter, returns Ok */
    let ca = wca::commands_aggregator()
    .commands( commands_form() )
    .form();
    let got = ca.program_perform( ".help .list" );
    a_id!( got, Ok( () ) );

    /* two commands, implicit delimeter, second command returns Err */
    let ca = wca::commands_aggregator()
    .commands( commands_form() )
    .form();
    let got = ca.program_perform( ".list .error" );
    a_id!( got, Err( BasicError::new( "err" ) ) );
  }

  //

  fn program_perform_with_dotted_paths()
  {
    /* single command with subject as single dot */
    let ca = wca::commands_aggregator()
    .commands( commands_form() )
    .form();
    let got = ca.program_perform( ".list ." );
    a_id!( got, Ok( () ) );

    /* single command with subject as double dot */
    let ca = wca::commands_aggregator()
    .commands( commands_form() )
    .form();
    let got = ca.program_perform( ".list .." );
    a_id!( got, Ok( () ) );

    /* single command with subject as dot and slash */
    let ca = wca::commands_aggregator()
    .commands( commands_form() )
    .form();
    let got = ca.program_perform( ".list ./" );
    a_id!( got, Ok( () ) );

    /* single command with subject as double dot and slash */
    let ca = wca::commands_aggregator()
    .commands( commands_form() )
    .form();
    let got = ca.program_perform( ".list ../" );
    a_id!( got, Ok( () ) );

    /* single command with subject as dot and backslash */
    let ca = wca::commands_aggregator()
    .commands( commands_form() )
    .form();
    let got = ca.program_perform( ".list .\\" );
    a_id!( got, Ok( () ) );

    /* single command with subject as double dot and backslash */
    let ca = wca::commands_aggregator()
    .commands( commands_form() )
    .form();
    let got = ca.program_perform( ".list ..\\" );
    a_id!( got, Ok( () ) );

    /* */

    /* two commands with subjects with dots */
    let ca = wca::commands_aggregator()
    .commands( commands_form() )
    .form();
    let got = ca.program_perform( ".list ..\\ .help ./some" );
    a_id!( got, Ok( () ) );
  }

  //

  fn instruction_perform_basic()
  {
    /* no commands in aggregator */
    let ca = wca::commands_aggregator()
    .form();
    let got = ca.instruction_perform( ".help" );
    a_id!( got, Ok( () ) );

    /* command returns Ok */
    let ca = wca::commands_aggregator()
    .commands( commands_form() )
    .form();
    let got = ca.instruction_perform( ".help" );
    a_id!( got, Ok( () ) );

    /* command returns Err */
    let ca = wca::commands_aggregator()
    .commands( commands_form() )
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
    let got = ca.instructions_parse( "" ).unwrap();
    a_id!( got, vec![] );

    /* */

    /* single command without subject and map */
    let got = ca.instructions_parse( ".help" ).unwrap();
    let instruction = wca::instruction::Instruction
    {
      command_name : ".help".to_string(),
      subject : "".to_string(),
      properties_map : HashMap::new(),
    };
    a_id!( got, vec![ instruction ] );

    /* single command with subject */
    let got = ca.instructions_parse( ".help command" ).unwrap();
    let instruction = wca::instruction::Instruction
    {
      command_name : ".help".to_string(),
      subject : "command".to_string(),
      properties_map : HashMap::new(),
    };
    a_id!( got, vec![ instruction ] );

    /* single command with subject and map */
    let got = ca.instructions_parse( ".help command v:3" ).unwrap();
    let instruction = wca::instruction::Instruction
    {
      command_name : ".help".to_string(),
      subject : "command".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Primitive( "3".to_string() ) ) ]),
    };
    a_id!( got, vec![ instruction ] );

    /* */

    /* two commands without subject and map, explicit delimeter */
    let got = ca.instructions_parse( ".help ; .version" ).unwrap();
    let instruction1 = wca::instruction::Instruction
    {
      command_name : ".help".to_string(),
      subject : "".to_string(),
      properties_map : HashMap::new(),
    };
    let instruction2 = wca::instruction::Instruction
    {
      command_name : ".version".to_string(),
      subject : "".to_string(),
      properties_map : HashMap::new(),
    };
    a_id!( got, vec![ instruction1, instruction2 ] );

    /* two commands with subject, explicit delimeter */
    let got = ca.instructions_parse( ".help command ; .version delta" ).unwrap();
    let instruction1 = wca::instruction::Instruction
    {
      command_name : ".help".to_string(),
      subject : "command".to_string(),
      properties_map : HashMap::new(),
    };
    let instruction2 = wca::instruction::Instruction
    {
      command_name : ".version".to_string(),
      subject : "delta".to_string(),
      properties_map : HashMap::new(),
    };
    a_id!( got, vec![ instruction1, instruction2 ] );

    /* two commands with subject and map, explicit delimeter */
    let got = ca.instructions_parse( ".help command v:3 ; .version delta n:5" ).unwrap();
    let instruction1 = wca::instruction::Instruction
    {
      command_name : ".help".to_string(),
      subject : "command".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Primitive( "3".to_string() ) ) ]),
    };
    let instruction2 = wca::instruction::Instruction
    {
      command_name : ".version".to_string(),
      subject : "delta".to_string(),
      properties_map : HashMap::from([ ( "n".to_string(), Primitive( "5".to_string() ) ) ]),
    };
    a_id!( got, vec![ instruction1, instruction2 ] );

    /* */

    /* two commands without subject and map, implicit delimeter */
    let got = ca.instructions_parse( ".help .version" ).unwrap();
    let instruction1 = wca::instruction::Instruction
    {
      command_name : ".help".to_string(),
      subject : "".to_string(),
      properties_map : HashMap::new(),
    };
    let instruction2 = wca::instruction::Instruction
    {
      command_name : ".version".to_string(),
      subject : "".to_string(),
      properties_map : HashMap::new(),
    };
    a_id!( got, vec![ instruction1, instruction2 ] );

    /* two commands with subject, implicit delimeter */
    let got = ca.instructions_parse( ".help command .version delta" ).unwrap();
    let instruction1 = wca::instruction::Instruction
    {
      command_name : ".help".to_string(),
      subject : "command".to_string(),
      properties_map : HashMap::new(),
    };
    let instruction2 = wca::instruction::Instruction
    {
      command_name : ".version".to_string(),
      subject : "delta".to_string(),
      properties_map : HashMap::new(),
    };
    a_id!( got, vec![ instruction1, instruction2 ] );

    /* two commands with subject and map, implicit delimeter */
    let got = ca.instructions_parse( ".help command v:3 .version delta n:5" ).unwrap();
    let instruction1 = wca::instruction::Instruction
    {
      command_name : ".help".to_string(),
      subject : "command".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Primitive( "3".to_string() ) ) ]),
    };
    let instruction2 = wca::instruction::Instruction
    {
      command_name : ".version".to_string(),
      subject : "delta".to_string(),
      properties_map : HashMap::from([ ( "n".to_string(), Primitive( "5".to_string() ) ) ]),
    };
    a_id!( got, vec![ instruction1, instruction2 ] );
  }

  fn instructions_with_paths()
  {
    let ca = wca::commands_aggregator()
    .form();

    let got = ca.instructions_parse( ".help ./tmp/dir v:3 .version delta n:./tmp/dir/" ).unwrap();
    let instruction1 = wca::instruction::Instruction
    {
      command_name : ".help".to_string(),
      subject : "./tmp/dir".to_string(),
      properties_map : HashMap::from([ ( "v".to_string(), Primitive( "3".to_string() ) ) ]),
    };
    let instruction2 = wca::instruction::Instruction
    {
      command_name : ".version".to_string(),
      subject : "delta".to_string(),
      properties_map : HashMap::from([ ( "n".to_string(), Primitive( "./tmp/dir/".to_string() ) ) ]),
    };
    a_id!( got, vec![ instruction1, instruction2 ] );
  }

  fn program_perform_with_context()
  {
    use wtools::Itertools;

    let eat_command : Command = wca::Command::former()
    .hint( "Eat." )
    .long_hint( "Clear a list." )
    .phrase( ".eat" )
    .subject_hint( "some command" )
    .routine_with_ctx( | _ : Args< String, NoProperties >, ctx : wca::Context |
    {
      let mut ctx = ctx.get_mut::< Vec< i32 > >().unwrap();
      ctx.clear();

      Ok( () )
    })
    .form();

    let list_command : Command = wca::Command::former()
    .hint( "List." )
    .long_hint( "Make a list." )
    .phrase( ".list" )
    .subject_hint( "some command" )
    .routine_with_ctx( | _ : Args< String, NoProperties >, ctx : wca::Context |
    {
      let mut ctx = ctx.get_mut::< Vec< i32 > >().unwrap();
      *ctx = vec![ 1, 2, 3 ];

      Ok( () )
    })
    .form();

    let inc_command : Command = wca::Command::former()
    .hint( "Increase values." )
    .long_hint( "Increment list vales." )
    .phrase( ".inc" )
    .subject_hint( "some command" )
    .routine_with_ctx( | _ : Args< String, NoProperties >, ctx : wca::Context |
    {
      let mut ctx = ctx.get_mut::< Vec< i32 > >().unwrap();
      ctx.iter_mut().for_each( | i | { *i += 1; } );

      Ok( () )
    })
    .form();

    let commands = vec![ list_command, eat_command, inc_command ]
    .into_iter()
    .map( | command | ( command.phrase.to_string(), command ) )
    .collect::< HashMap< String, Command > >();

    let ca = wca::commands_aggregator()
    .commands( commands )
    .context( wca::Context::new( Vec::< i32 >::new() ) )
    .form();

    // get a list and increment values
    let got = ca.program_perform( ".list .inc" );
    let ctx = ca.context.as_ref().unwrap();
    let ctx = ctx.get_ref::< Vec< i32 > >().unwrap();
    a_id!( got, Ok( () ) );
    a_id!( vec![ 2, 3, 4 ], *ctx );

    // get a list and increment twice values
    let got = ca.program_perform( ".list .inc .inc" );
    let ctx = ca.context.as_ref().unwrap();
    let ctx = ctx.get_ref::< Vec< i32 > >().unwrap();
    a_id!( got, Ok( () ) );
    a_id!( vec![ 3, 4, 5 ], *ctx );

    // get a list, increment values and clear the list
    let got = ca.program_perform( ".list .inc .eat" );
    let ctx = ca.context.as_ref().unwrap();
    let ctx = ctx.get_ref::< Vec< i32 > >().unwrap();
    a_id!( got, Ok( () ) );
    a_id!( Vec::< i32 >::new(), *ctx );

    // eat an empty list and icrease its values(do nothing)
    let got = ca.program_perform( ".eat .inc" );
    let ctx = ca.context.as_ref().unwrap();
    let ctx = ctx.get_ref::< Vec< i32 > >().unwrap();
    a_id!( got, Ok( () ) );
    a_id!( Vec::< i32 >::new(), *ctx );
  }

  fn program_perform_with_several_context_values()
  {
    use wtools::Itertools;

    let list_command : Command = wca::Command::former()
    .hint( "List." )
    .long_hint( "Make a list." )
    .phrase( ".list" )
    .subject_hint( "some command" )
    .routine_with_ctx( | _ : Args< String, NoProperties >, ctx : wca::Context |
    {
      let mut ctx = ctx.get_mut::< Vec< i32 > >().unwrap();
      *ctx = vec![ 1, 2, 3 ];

      Ok( () )
    })
    .form();

    let inc_command : Command = wca::Command::former()
    .hint( "Increase values." )
    .long_hint( "Add value from context. Grows every call" )
    .phrase( ".inc" )
    .subject_hint( "some command" )
    .routine_with_ctx( | _ : Args< String, NoProperties >, ctx : wca::Context |
    {
      let mut vec = ctx.get_mut::< Vec< i32 > >().unwrap();
      let mut value = ctx.get_mut::< i32 >().unwrap();

      vec.iter_mut().for_each( | i | { *i += *value; } );

      *value += 1;

      Ok( () )
    })
    .form();

    let commands = vec![ list_command, inc_command ]
    .into_iter()
    .map( | command | ( command.phrase.to_string(), command ) )
    .collect::< HashMap< String, Command > >();

    let mut ctx = wca::Context::new( Vec::< i32 >::new() );
    ctx.insert( 1_i32 );

    let mut ca = wca::commands_aggregator()
    .commands( commands )
    .context( ctx )
    .form();

    // get a list and increment values
    let got = ca.program_perform( ".list .inc" );
    let ctx = ca.context.as_ref().unwrap();
    let ctx = ctx.get_ref::< Vec< i32 > >().unwrap();
    a_id!( got, Ok( () ) );
    a_id!( vec![ 2, 3, 4 ], *ctx );

    // reset context
    let mut ctx = wca::Context::new( Vec::< i32 >::new() );
    ctx.insert( 1_i32 );
    ca.context = Some( ctx );

    // get a list and increment twice values
    // [ 1, 2, 3 ] +1 -> [ 2, 3, 4 ] +2 -> [ 4, 5, 6 ]
    let got = ca.program_perform( ".list .inc .inc" );
    let ctx = ca.context.as_ref().unwrap();
    let ctx = ctx.get_ref::< Vec< i32 > >().unwrap();
    a_id!( got, Ok( () ) );
    assert_eq!( vec![ 4, 5, 6 ], *ctx );
  }

  fn program_perform_with_complex_context_types()
  {
    use core::str::FromStr;

    let test_command : Command = wca::Command::former()
    .hint( "Test" )
    .long_hint( "Test getting complex types" )
    .phrase( ".test" )
    .subject_hint( "some command" )
    .routine_with_ctx( | _ : Args< String, NoProperties >, ctx : wca::Context |
    {
      let string = ctx.get_ref::< &str >().unwrap();
      assert_eq!( "Hello, World!", *string );

      let ctx_with_ctx = ctx.get_ref::< wca::Context >().unwrap();
      let path = ctx_with_ctx.get_ref::< std::path::PathBuf >().unwrap();
      assert_eq!( std::path::PathBuf::from_str( "./" ).unwrap(), *path );

      let reference_counter = ctx.get_ref::< std::rc::Rc< i32 > >().unwrap();
      // reference on reference counter
      assert_eq!( 8, **reference_counter );

      Ok( () )
    })
    .form();

    let commands = vec![ test_command ]
    .into_iter()
    .map( | command | ( command.phrase.to_string(), command ) )
    .collect::< HashMap< String, Command > >();

    let path = std::path::PathBuf::from_str( "./" ).unwrap();
    // Context with path
    let ctx = wca::Context::new( path );
    // Context with context with path
    let mut ctx = wca::Context::new( ctx );
    // And with string
    ctx.insert( "Hello, World!" );
    // And with reference counter
    ctx.insert( std::rc::Rc::new( 8 ) );

    let mut ca = wca::commands_aggregator()
    .commands( commands )
    .context( ctx )
    .form();

    assert!( ca.program_perform( ".test" ).is_ok() );
  }

  fn chaining()
  {
    let loop_command : Command = wca::Command::former()
    .hint( "Loop" )
    .long_hint( "Loop block" )
    .phrase( ".loop" )
    .subject_hint( "some command" )
    .routine_with_ctx( | _ : Args< String, NoProperties >, ctx : wca::Context |
    {
      let state = ctx.get_mut::< State >().ok_or_else( || BasicError::new( "Have no State" ) )?;
      let breakpoint = ctx.get_mut::< Breakpoint >().ok_or_else( || BasicError::new( "Have no Breakpoints" ) )?;
      let prog_state = ctx.get_ref::< wca::ProgramState >().ok_or_else( || BasicError::new( "Have no State" ) )?;

      state.current_value = state.iter.next();
      breakpoint.0 = prog_state.current_pos - 1;

      Ok( () )
    })
    .form();

    let inc_command : Command = wca::Command::former()
    .hint( "Increment values." )
    .long_hint( "" )
    .phrase( ".inc" )
    .subject_hint( "some command" )
    .routine_with_ctx( | _ : Args< String, NoProperties >, ctx : wca::Context |
    {
      let state = ctx.get_mut::< State >().ok_or_else( || BasicError::new( "Have no State" ) )?;
      state.current_value = state.current_value.map( | mut v | v + 1 );

      Ok( () )
    })
    .form();

    let print_command : Command = wca::Command::former()
    .hint( "Prints values." )
    .long_hint( "" )
    .phrase( ".print" )
    .subject_hint( "some command" )
    .routine_with_ctx( | _ : Args< String, NoProperties >, ctx : wca::Context |
    {
      let state = ctx.get_mut::< State >().ok_or_else( || BasicError::new( "Have no State" ) )?;
      if let Some( value ) = state.current_value {
        print!( "{value}" );
        state.processed.push( value );
      }

      Ok( () )
    })
    .form();

    let loop_end_command : Command = wca::Command::former()
    .hint( "End of loop" )
    .long_hint( "" )
    .phrase( ".end" )
    .subject_hint( "some command" )
    .routine_with_ctx( | _ : Args< String, NoProperties >, ctx : wca::Context |
    {
      let state = ctx.get_ref::< State >().ok_or_else( || BasicError::new( "Have no State" ) )?;
      let breakpoint = ctx.get_ref::< Breakpoint >().ok_or_else( || BasicError::new( "Have no Breakpoints" ) )?;
      let prog_state = ctx.get_mut::< wca::ProgramState >().ok_or_else( || BasicError::new( "Have no State" ) )?;

      if state.current_value.is_some() {
        prog_state.current_pos = breakpoint.0;
      }

      Ok( () )
    })
    .form();

    let commands = vec![ loop_command, inc_command, print_command, loop_end_command ]
    .into_iter()
    .map( | command | ( command.phrase.to_string(), command ) )
    .collect::< HashMap< String, Command > >();

    struct State
    {
      current_value : Option< i32 >,
      iter : Box< dyn Iterator< Item = i32 > >,
      processed : Vec< i32 >,
    }

    struct Breakpoint( usize );

    let vec = vec![ 1, 2, 3 ];
    let mut ctx = wca::Context::new( State { current_value : None, iter : Box::new( vec.into_iter() ) , processed : vec![] } );
    ctx.insert( Breakpoint( 0 ) );

    let mut ca = wca::commands_aggregator()
    .commands( commands )
    .context( ctx )
    .form();

    let got = ca.program_perform( ".loop .iter .inc .print .end" );

    let ctx = ca.context.as_ref().unwrap();
    let ctx = ctx.get_ref::< State >().unwrap();

    a_id!( got, Ok( () ) );
    a_id!( vec![ 2, 3, 4 ], ctx.processed );
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
  instructions_with_paths,
  program_perform_with_context,
  program_perform_with_several_context_values,
  program_perform_with_complex_context_types,
  chaining,
}

