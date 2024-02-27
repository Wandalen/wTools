use super::*;

//

tests_impls!
{
  fn basic()
  {
    let parser = Parser::former().form();

    // only command
    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::new(),
      },
      parser.command( ".command" ).unwrap()
    );

    // command with one subject
    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![ "subject".into() ],
        properties : HashMap::new(),
      },
      parser.command( ".command subject" ).unwrap()
    );

    // command with many subjects
    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![ "subject1".into(), "subject2".into(), "subject3".into() ],
        properties : HashMap::new(),
      },
      parser.command( ".command subject1 subject2 subject3" ).unwrap()
    );

    // command with one property
    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "prop".into(), "value".into() ) ]),
      },
      parser.command( ".command prop:value" ).unwrap()
    );

    // command with many properties
    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter(
        [
          ( "prop1".into(), "value1".into() ),
          ( "prop2".into(), "value2".into() ),
          ( "prop3".into(), "value3".into() )
        ]),
      },
      parser.command( ".command prop1:value1 prop2:value2 prop3:value3" ).unwrap()
    );

    // command with one subject and one property
    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![ "subject".into() ],
        properties : HashMap::from_iter([ ( "prop".into(), "value".into() ) ]),
      },
      parser.command( ".command subject prop:value" ).unwrap()
    );

    // command with many subjects and many properties
    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec!
        [
          "subject1".into(),
          "subject2".into(),
          "subject3".into(),
        ],
        properties : HashMap::from_iter(
        [
          ( "prop1".into(), "value1".into() ),
          ( "prop2".into(), "value2".into() ),
          ( "prop3".into(), "value3".into() ),
        ]),
      },
      parser.command( ".command subject1 subject2 subject3 prop1:value1 prop2:value2 prop3:value3" ).unwrap()
    );
  }

  fn with_spaces()
  {
    let parser = Parser::former().form();

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::new(),
      },
      parser.command( "     .command   " ).unwrap()
    );

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![ "subject".into() ],
        properties : HashMap::new(),
      },
      parser.command( "   .command  subject  " ).unwrap()
    );

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![ "subject".into() ],
        properties : HashMap::from_iter([ ( "prop".into(), "value".into() ) ]),
      },
      parser.command( "   .command  subject  prop:value " ).unwrap()
    );
  }

  fn not_only_alphanumeric_symbols()
  {
    let parser = Parser::former().form();

    a_id!
    (
      ParsedCommand
      {
        name : "additional_command".into(),
        subjects : vec![],
        properties : HashMap::new(),
      },
      parser.command( ".additional_command" ).unwrap()
    );

    a_id!
    (
      ParsedCommand
      {
        name : "command.sub_command".into(),
        subjects : vec![ "subj_ect".into() ],
        properties : HashMap::new(),
      },
      parser.command( ".command.sub_command subj_ect" ).unwrap()
    );

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "long_prop".into(), "some-value".into() ) ]),
      },
      parser.command( ".command long_prop:some-value" ).unwrap()
    );
  }

  fn same_command_and_prop_delimeter()
  {
    let parser = Parser::former()
    .command_prefix( '-' )
    .prop_delimeter( '-' )
    .form();

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![ "subject".into() ],
        properties : HashMap::from_iter([ ( "prop".into(), "value".into() ) ]),
      },
      parser.command( "-command subject prop-value" ).unwrap()
    );
  }

  fn path_in_subject()
  {
    let parser = Parser::former().form();

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![ "/absolute/path/to/something".into() ],
        properties : HashMap::new(),
      },
      parser.command( ".command /absolute/path/to/something" ).unwrap()
    );

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![ "./path/to/something".into() ],
        properties : HashMap::new(),
      },
      parser.command( ".command ./path/to/something" ).unwrap()
    );
  }

  fn path_in_property()
  {
    let parser = Parser::former().form();

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "path".into(), "/absolute/path/to/something".into() ) ]),
      },
      parser.command( ".command path:/absolute/path/to/something" ).unwrap()
    );

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "path".into(), "./path/to/something".into() ) ]),
      },
      parser.command( ".command path:./path/to/something" ).unwrap()
    );

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "path".into(), "../path/to/something".into() ) ]),
      },
      parser.command( ".command path:../path/to/something" ).unwrap()
    );

    let parser = Parser::former()
    .command_prefix( '/' )
    .form();

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "path".into(), "/absolute/path/to/something".into() ) ]),
      },
      parser.command( "/command path:/absolute/path/to/something" ).unwrap()
    );
  }

  fn list_in_property()
  {
    let parser = Parser::former().form();

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "list".into(), "[1,2,3]".into() ) ]),
      },
      parser.command( ".command list:[1,2,3]" ).unwrap()
    );
  }

  fn string_value()
  {
    let parser = Parser::former().form();

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![ "subject with spaces".into() ],
        properties : HashMap::from_iter([ ( "prop".into(), "property with spaces".into() ) ]),
      },
      parser.command( r#".command "subject with spaces" prop:"property with spaces""# ).unwrap()
    );

    // command in subject and property
    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![ ".command".into() ],
        properties : HashMap::from_iter([ ( "prop".into(), ".command".into() ) ]),
      },
      parser.command( r#".command ".command" prop:".command""# ).unwrap()
    );

    // with escaped quetes
    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![ "' queted ' \\ value".into() ],
        properties : HashMap::from_iter([ ( "prop".into(), "some \"quetes\" ' \\ in string".into() ) ]),
      },
      parser.command( r#".command '\' queted \' \\ value' prop:"some \"quetes\" ' \\ in string""# ).unwrap()
    );
  }

  fn dot_command()
  {
    let parser = Parser::former().form();

    // single dot
    a_id!
    (
      ParsedCommand
      {
        name : "".into(),
        subjects : vec![],
        properties : HashMap::from_iter([( "command_prefix".into(), ".".into() )]),
      },
      parser.command( "." ).unwrap()
    );

    // command .
    a_id!
    (
      ParsedCommand
      {
        name : "".into(),
        subjects : vec![ "command.".into() ],
        properties : HashMap::from_iter([( "command_prefix".into(), ".".into() )]),
      },
      parser.command( ".command." ).unwrap()
    );

    // command . with subjects
    a_id!
    (
      ParsedCommand
      {
        name : "".into(),
        subjects : vec![ "command.".into() ],
        properties : HashMap::from_iter([( "command_prefix".into(), ".".into() )]),
      },
      parser.command( ".command. <this will be ignored>" ).unwrap()
    );
  }
}

//

tests_index!
{
  basic,
  with_spaces,
  not_only_alphanumeric_symbols,
  same_command_and_prop_delimeter,
  path_in_subject,
  path_in_property,
  list_in_property,
  string_value,
  dot_command,
}
