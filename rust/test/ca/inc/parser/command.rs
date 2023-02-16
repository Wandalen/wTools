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
      Ok( Command
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::new(),
      }),
      parser.command( ".command" )
    );

    // command with one subject
    a_id!
    (
      Ok( Command
      {
        name : "command".into(),
        subjects : vec![ "subject".into() ],
        properties : HashMap::new(),
      }),
      parser.command( ".command subject" )
    );

    // command with many subjects
    a_id!
    (
      Ok( Command
      {
        name : "command".into(),
        subjects : vec![ "subject1".into(), "subject2".into(), "subject3".into() ],
        properties : HashMap::new(),
      }),
      parser.command( ".command subject1 subject2 subject3" )
    );

    // command with one property
    a_id!
    (
      Ok( Command
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "prop".into(), "value".into() ) ]),
      }),
      parser.command( ".command prop:value" )
    );

    // command with many properties
    a_id!
    (
      Ok( Command
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter(
        [
          ( "prop1".into(), "value1".into() ),
          ( "prop2".into(), "value2".into() ),
          ( "prop3".into(), "value3".into() )
        ]),
      }),
      parser.command( ".command prop1:value1 prop2:value2 prop3:value3" )
    );

    // command with one subject and one property
    a_id!
    (
      Ok( Command
      {
        name : "command".into(),
        subjects : vec![ "subject".into() ],
        properties : HashMap::from_iter([ ( "prop".into(), "value".into() ) ]),
      }),
      parser.command( ".command subject prop:value" )
    );

    // command with many subjects and many properties
    a_id!
    (
      Ok( Command
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
      }),
      parser.command( ".command subject1 subject2 subject3 prop1:value1 prop2:value2 prop3:value3" )
    );
  }

  fn with_spaces()
  {
    let parser = Parser::former().form();

    a_id!
    (
      Ok( Command
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::new(),
      }),
      parser.command( "     .command   " )
    );

    a_id!
    (
      Ok( Command
      {
        name : "command".into(),
        subjects : vec![ "subject".into() ],
        properties : HashMap::new(),
      }),
      parser.command( "   .command  subject  " )
    );

    a_id!
    (
      Ok( Command
      {
        name : "command".into(),
        subjects : vec![ "subject".into() ],
        properties : HashMap::from_iter([ ( "prop".into(), "value".into() ) ]),
      }),
      parser.command( "   .command  subject  prop:value " )
    );
  }

  fn not_only_alphanumeric_symbols()
  {
    let parser = Parser::former().form();

    a_id!
    (
      Ok( Command
      {
        name : "additional_command".into(),
        subjects : vec![],
        properties : HashMap::new(),
      }),
      parser.command( ".additional_command" )
    );

    a_id!
    (
      Ok( Command
      {
        name : "command.sub_command".into(),
        subjects : vec![ "subj_ect".into() ],
        properties : HashMap::new(),
      }),
      parser.command( ".command.sub_command subj_ect" )
    );

    a_id!
    (
      Ok( Command
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "long_prop".into(), "some-value".into() ) ]),
      }),
      parser.command( ".command long_prop:some-value" )
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
      Ok( Command
      {
        name : "command".into(),
        subjects : vec![ "subject".into() ],
        properties : HashMap::from_iter([ ( "prop".into(), "value".into() ) ]),
      }),
      parser.command( "-command subject prop-value" )
    );
  }

  fn path_in_subject()
  {
    let parser = Parser::former().form();

    a_id!
    (
      Ok( Command
      {
        name : "command".into(),
        subjects : vec![ "/absolute/path/to/something".into() ],
        properties : HashMap::new(),
      }),
      parser.command( ".command /absolute/path/to/something" )
    );

    a_id!
    (
      Ok( Command
      {
        name : "command".into(),
        subjects : vec![ "./path/to/something".into() ],
        properties : HashMap::new(),
      }),
      parser.command( ".command ./path/to/something" )
    );
  }

  fn path_in_property()
  {
    let parser = Parser::former().form();

    a_id!
    (
      Ok( Command
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "path".into(), "/absolute/path/to/something".into() ) ]),
      }),
      parser.command( ".command path:/absolute/path/to/something" )
    );

    a_id!
    (
      Ok( Command
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "path".into(), "./path/to/something".into() ) ]),
      }),
      parser.command( ".command path:./path/to/something" )
    );

    a_id!
    (
      Ok( Command
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "path".into(), "../path/to/something".into() ) ]),
      }),
      parser.command( ".command path:../path/to/something" )
    );

    let parser = Parser::former()
    .command_prefix( '/' )
    .form();

    a_id!
    (
      Ok( Command
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "path".into(), "/absolute/path/to/something".into() ) ]),
      }),
      parser.command( "/command path:/absolute/path/to/something" )
    );
  }

  fn list_in_property()
  {
    let parser = Parser::former().form();

    a_id!
    (
      Ok( Command
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "list".into(), "[1,2,3]".into() ) ]),
      }),
      parser.command( ".command list:[1,2,3]" )
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
}
