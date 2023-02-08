use super::*;

//

tests_impls!
{
  fn basic()
  {
    // TODO: Builder
    let parser = Parser
    {
      command_delimeter : '.',
      prop_delimeter : ':',
      namespace_delimeter : ".also".into(),
    };

    // namespace with only one command
    a_id!
    (
      Ok( Namespace
      {
        commands : vec![ Command
        {
          name : "command".into(),
          subjects : vec![],
          properties : HashMap::new(),
        }]
      }),
      parser.namespace( ".command" )
    );

    // only one command in first namespace
    a_id!
    (
      Ok( Namespace
      {
        commands : vec![ Command
        {
          name : "command".into(),
          subjects : vec![],
          properties : HashMap::new(),
        }]
      }),
      parser.namespace( ".command .also .command2" )
    );

    // many commands in first namespace and some in another
    a_id!
    (
      Ok( Namespace
      {
        commands : vec!
        [
          Command
          {
            name : "command1".into(),
            subjects : vec![],
            properties : HashMap::new(),
          },
          Command
          {
            name : "command2".into(),
            subjects : vec![ "subject".into() ],
            properties : HashMap::from_iter([ ( "prop".into(), "12".into() ) ]),
          }
        ]
      }),
      parser.namespace( ".command1 .command2 subject prop:12 .also .command3" )
    );
  }

  fn same_command_and_prop_and_namespace_delimeter()
  {
    // TODO: Builder
    let parser = Parser
    {
      command_delimeter : '-',
      prop_delimeter : '-',
      namespace_delimeter : "-".into(),
    };

    a_id!
    (
      Ok( Namespace
      {
        commands : vec!
        [
          Command
          {
            name : "command1".into(),
            subjects : vec![ "subject".into() ],
            properties : HashMap::from_iter([ ( "prop".into(), "value".into() ) ]),
          },
          Command
          {
            name : "command2".into(),
            subjects : vec![],
            properties : HashMap::new(),
          }
        ]
      }),
      parser.namespace( "-command1 subject prop-value -command2 - -command3" )
    );
  }
}

//

tests_index!
{
  basic,
  same_command_and_prop_and_namespace_delimeter,
}