use super::*;

//

tests_impls!
{
  fn basic()
  {
    let parser = Parser;

    // only command
    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::new(),
      },
      parser.parse( [ ".command" ] ).unwrap().commands[ 0 ]
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
      parser.parse( [ ".command", "subject" ] ).unwrap().commands[ 0 ]
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
      parser.parse( [ ".command", "subject1", "subject2", "subject3" ] ).unwrap().commands[ 0 ]
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
      parser.parse( [ ".command", "prop:value" ] ).unwrap().commands[ 0 ]
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
      parser.parse( [ ".command", "prop1:value1", "prop2:value2", "prop3:value3" ] ).unwrap().commands[ 0 ]
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
      parser.parse( [ ".command", "subject", "prop:value" ] ).unwrap().commands[ 0 ]
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
      parser.parse( [ ".command", "subject1", "subject2", "subject3", "prop1:value1", "prop2:value2", "prop3:value3" ] ).unwrap().commands[ 0 ]
    );
  }

  // aaa : the parser must be able to accept a list of arguments(std::env::args())
  // aaa : yep
  fn with_spaces_in_value()
  {
    let parser = Parser;

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![ "value with spaces".into() ],
        properties : HashMap::new(),
      },
      parser.parse( [ ".command", "value with spaces" ] ).unwrap().commands[ 0 ]
    );

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "prop".into(), "value with spaces".into() ) ]),
      },
      parser.parse( [ ".command", "prop:value with spaces" ] ).unwrap().commands[ 0 ]
    );

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "prop".into(), "value with spaces".into() ) ]),
      },
      parser.parse( [ ".command", "prop:", "value with spaces" ] ).unwrap().commands[ 0 ]
    );
    
    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "prop".into(), "value with spaces".into() ) ]),
      },
      parser.parse( [ ".command", "prop", ":value with spaces" ] ).unwrap().commands[ 0 ]
    );

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "prop".into(), "value with spaces".into() ) ]),
      },
      parser.parse( [ ".command", "prop", ":", "value with spaces" ] ).unwrap().commands[ 0 ]
    );
  }

  fn not_only_alphanumeric_symbols()
  {
    let parser = Parser;

    a_id!
    (
      ParsedCommand
      {
        name : "additional_command".into(),
        subjects : vec![],
        properties : HashMap::new(),
      },
      parser.parse( [ ".additional_command" ] ).unwrap().commands[ 0 ]
    );

    a_id!
    (
      ParsedCommand
      {
        name : "command.sub_command".into(),
        subjects : vec![ "subj_ect".into() ],
        properties : HashMap::new(),
      },
      parser.parse( [ ".command.sub_command", "subj_ect" ] ).unwrap().commands[ 0 ]
    );

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "long_prop".into(), "some-value".into() ) ]),
      },
      parser.parse( [ ".command", "long_prop:some-value" ] ).unwrap().commands[ 0 ]
    );
  }

  fn path_in_subject()
  {
    let parser = Parser;

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![ "/absolute/path/to/something".into() ],
        properties : HashMap::new(),
      },
      parser.parse( [ ".command", "/absolute/path/to/something" ] ).unwrap().commands[ 0 ]
    );

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![ "./path/to/something".into() ],
        properties : HashMap::new(),
      },
      parser.parse( [ ".command", "./path/to/something" ] ).unwrap().commands[ 0 ]
    );
  }

  fn path_in_property()
  {
    let parser = Parser;

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "path".into(), "/absolute/path/to/something".into() ) ]),
      },
      parser.parse( [ ".command", "path:/absolute/path/to/something" ] ).unwrap().commands[ 0 ]
    );

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "path".into(), "./path/to/something".into() ) ]),
      },
      parser.parse( [ ".command", "path:./path/to/something" ] ).unwrap().commands[ 0 ]
    );

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "path".into(), "../path/to/something".into() ) ]),
      },
      parser.parse( [ ".command", "path:../path/to/something" ] ).unwrap().commands[ 0 ]
    );
  }

  fn list_in_property()
  {
    let parser = Parser;

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![],
        properties : HashMap::from_iter([ ( "list".into(), "[1,2,3]".into() ) ]),
      },
      parser.parse( [ ".command", "list:[1,2,3]" ] ).unwrap().commands[ 0 ]
    );
  }

  fn string_value()
  {
    let parser = Parser;

    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![ "subject with spaces".into() ],
        properties : HashMap::from_iter([ ( "prop".into(), "property with spaces".into() ) ]),
      },
      parser.parse( [ ".command", "subject with spaces", "prop:property with spaces" ] ).unwrap().commands[ 0 ]
    );

    // command in subject and property
    a_id!
    (
      ParsedCommand
      {
        name : "command".into(),
        subjects : vec![ "\\.command".into() ],
        properties : HashMap::from_iter([ ( "prop".into(), ".command".into() ) ]),
      },
      parser.parse( [ ".command", "\\.command", "prop:.command" ] ).unwrap().commands[ 0 ]
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
      parser.parse( [ ".command", "\' queted \' \\ value", "prop:some \"quetes\" ' \\ in string" ] ).unwrap().commands[ 0 ]
    );
  }

  fn dot_command()
  {
    let parser = Parser;

    a_id!
    (
      ParsedCommand
      {
        name : ".".into(),
        subjects : vec![],
        properties : HashMap::new(),
      },
      parser.parse( [ "." ] ).unwrap().commands[ 0 ]
    );

    a_id!
    (
      ParsedCommand
      {
        name : "command.".into(),
        subjects : vec![],
        properties : HashMap::new(),
      },
      parser.parse( [ ".command." ] ).unwrap().commands[ 0 ]
    );
    
    a_id!
    (
      ParsedCommand
      {
        name : ".?".into(),
        subjects : vec![],
        properties : HashMap::new(),
      },
      parser.parse( [ ".?" ] ).unwrap().commands[ 0 ]
    );
    
    a_id!
    (
      ParsedCommand
      {
        name : "command.?".into(),
        subjects : vec![],
        properties : HashMap::new(),
      },
      parser.parse( [ ".command.?" ] ).unwrap().commands[ 0 ]
    );
  }
}

//

tests_index!
{
  basic,
  with_spaces_in_value,
  not_only_alphanumeric_symbols,
  path_in_subject,
  path_in_property,
  list_in_property,
  string_value,
  dot_command,
}
