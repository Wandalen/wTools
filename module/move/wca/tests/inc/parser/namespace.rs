use super::*;

//

tests_impls!
{
  fn basic()
  {
    let parser = Parser::former().form();

    // namespace with only one command
    a_id!
    (
      Namespace
      {
        commands : vec![ RawCommand
        {
          name : "command".into(),
          subjects : vec![],
          properties : HashMap::new(),
        }]
      },
      parser.namespace( ".command" ).unwrap()
    );

    // only one command in first namespace
    a_id!
    (
      Namespace
      {
        commands : vec![ RawCommand
        {
          name : "command".into(),
          subjects : vec![],
          properties : HashMap::new(),
        }]
      },
      parser.namespace( ".command .also .command2" ).unwrap()
    );

    // many commands in first namespace and some in another
    a_id!
    (
      Namespace
      {
        commands : vec!
        [
          RawCommand
          {
            name : "command1".into(),
            subjects : vec![],
            properties : HashMap::new(),
          },
          RawCommand
          {
            name : "command2".into(),
            subjects : vec![ "subject".into() ],
            properties : HashMap::from_iter([ ( "prop".into(), "12".into() ) ]),
          }
        ]
      },
      parser.namespace( ".command1 .command2 subject prop:12 .also .command3" ).unwrap()
    );
  }

  fn same_command_and_prop_and_namespace_delimeter()
  {
    let parser = Parser::former()
    .command_prefix( '-' )
    .prop_delimeter( '-' )
    .namespace_delimeter( "-" )
    .form();

    a_id!
    (
      Namespace
      {
        commands : vec!
        [
          RawCommand
          {
            name : "command1".into(),
            subjects : vec![ "subject".into() ],
            properties : HashMap::from_iter([ ( "prop".into(), "value".into() ) ]),
          },
          RawCommand
          {
            name : "command2".into(),
            subjects : vec![],
            properties : HashMap::new(),
          }
        ]
      },
      parser.namespace( "-command1 subject prop-value -command2 - -command3" ).unwrap()
    );
  }
}

//

tests_index!
{
  basic,
  same_command_and_prop_and_namespace_delimeter,
}
