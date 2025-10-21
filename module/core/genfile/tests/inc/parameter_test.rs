/// Tests for Parameters system (FR3, FR4)
use super :: *;

//

#[ test ]
fn parameter_descriptor_stores_name()
{
  // FR3: ParameterDescriptor must store parameter name
  let param = ParameterDescriptor
  {
    parameter: "project_name".into(),
    is_mandatory: false,
    default_value: None,
    description: None,
  };

  // Should be able to access parameter name
  assert_eq!( param.parameter, "project_name" );
  assert!( !param.parameter.is_empty() );
}

#[ test ]
fn parameter_descriptor_mandatory_flag()
{
  // FR3: Must support mandatory flag (bool)
  let mandatory = ParameterDescriptor
  {
    parameter: "required_param".into(),
    is_mandatory: true,
    default_value: None,
    description: None,
  };

  let optional = ParameterDescriptor
  {
    parameter: "optional_param".into(),
    is_mandatory: false,
    default_value: None,
    description: None,
  };

  assert!( mandatory.is_mandatory );
  assert!( !optional.is_mandatory );
}

#[ test ]
fn parameter_descriptor_default_value()
{
  // FR3: Must support optional default value
  let with_default = ParameterDescriptor
  {
    parameter: "region".into(),
    is_mandatory: false,
    default_value: Some( "us-east-1".into() ),
    description: None,
  };

  // Default value should be Some
  assert!( with_default.default_value.is_some() );
  assert_eq!( with_default.default_value.unwrap(), "us-east-1" );
}

#[ test ]
fn parameter_descriptor_description()
{
  // FR3: Must support optional description
  let with_desc = ParameterDescriptor
  {
    parameter: "api_key".into(),
    is_mandatory: false,
    default_value: None,
    description: Some( "API key for authentication".into() ),
  };

  assert!( with_desc.description.is_some() );
  assert_eq!( with_desc.description.unwrap(), "API key for authentication" );
}

#[ test ]
fn parameters_collection_stores_multiple()
{
  // FR4: Must store multiple ParameterDescriptor instances
  let params = Parameters
  {
    descriptors: vec!
    [
      ParameterDescriptor
      {
        parameter: "name".into(),
        is_mandatory: true,
        default_value: None,
        description: None,
      },
      ParameterDescriptor
      {
        parameter: "version".into(),
        is_mandatory: true,
        default_value: None,
        description: None,
      },
      ParameterDescriptor
      {
        parameter: "description".into(),
        is_mandatory: false,
        default_value: None,
        description: None,
      },
    ],
  };

  // Should have 3 parameters
  assert_eq!( params.descriptors.len(), 3 );
}

#[ test ]
fn parameters_list_mandatory()
{
  // FR4: Must provide list_mandatory() method returning only mandatory parameter names
  let params = Parameters
  {
    descriptors: vec!
    [
      ParameterDescriptor
      {
        parameter: "name".into(),
        is_mandatory: true,
        default_value: None,
        description: None,
      },
      ParameterDescriptor
      {
        parameter: "url".into(),
        is_mandatory: true,
        default_value: None,
        description: None,
      },
      ParameterDescriptor
      {
        parameter: "description".into(),
        is_mandatory: false,
        default_value: None,
        description: None,
      },
      ParameterDescriptor
      {
        parameter: "tags".into(),
        is_mandatory: false,
        default_value: None,
        description: None,
      },
    ],
  };

  let mandatory = params.list_mandatory();

  assert_eq!( mandatory.len(), 2 );
  assert!( mandatory.contains( &"name" ) );
  assert!( mandatory.contains( &"url" ) );
  assert!( !mandatory.contains( &"description" ) );
  assert!( !mandatory.contains( &"tags" ) );
}
