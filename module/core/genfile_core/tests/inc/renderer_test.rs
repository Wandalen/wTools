/// Tests for `TemplateRenderer` trait and `HandlebarsRenderer` (FR8, FR9)
use super :: *;

//

#[ test ]
fn handlebars_renderer_variable_substitution()
{
  // FR9: Must support variable substitution: {{variable_name}}
  let renderer = HandlebarsRenderer ::new();
  let template = "Hello {{name}}!";

  let mut values = Values ::new();
  values.insert( "name", Value ::String( "World".into() ) );

  let result = renderer.render( template, &values.to_serializable() );
  assert!( result.is_ok() );
  assert_eq!( result.unwrap(), "Hello World!" );
}

#[ test ]
fn handlebars_renderer_multiple_variables()
{
  // FR9: Must support multiple variable substitutions
  let renderer = HandlebarsRenderer ::new();
  let template = "Project: {{name}}, Version: {{version}}";

  let mut values = Values ::new();
  values.insert( "name", Value ::String( "genfile".into() ) );
  values.insert( "version", Value ::Number( 1 ) );

  let result = renderer.render( template, &values.to_serializable() );
  assert!( result.is_ok() );
  assert_eq!( result.unwrap(), "Project: genfile, Version: 1" );
}

#[ test ]
fn handlebars_renderer_conditional_logic()
{
  // FR9: Must support conditional logic: {{#if condition}}
  let renderer = HandlebarsRenderer ::new();
  let template = "{{#if enabled}}Feature is on{{else}}Feature is off{{/if}}";

  let mut values = Values ::new();
  values.insert( "enabled", Value ::Bool( true ) );

  let result = renderer.render( template, &values.to_serializable() );
  assert!( result.is_ok() );
  assert_eq!( result.unwrap(), "Feature is on" );

  // Test with false
  let mut values2 = Values ::new();
  values2.insert( "enabled", Value ::Bool( false ) );

  let result2 = renderer.render( template, &values2.to_serializable() );
  assert!( result2.is_ok() );
  assert_eq!( result2.unwrap(), "Feature is off" );
}

#[ test ]
fn handlebars_renderer_no_html_escaping()
{
  // FR9: Must disable HTML escaping (use no_escape)
  let renderer = HandlebarsRenderer ::new();
  let template = "{{content}}";

  let mut values = Values ::new();
  values.insert( "content", Value ::String( "<div>HTML</div>".into() ) );

  let result = renderer.render( template, &values.to_serializable() );
  assert!( result.is_ok() );
  // Should NOT escape HTML
  assert_eq!( result.unwrap(), "<div>HTML</div>" );
}

#[ test ]
fn handlebars_renderer_invalid_template_returns_error()
{
  // FR9: Must return Error::Render on invalid template syntax
  let renderer = HandlebarsRenderer ::new();
  let template = "{{unclosed";

  let values = Values ::< Value >::new();

  let result = renderer.render( template, &values.to_serializable() );
  assert!( result.is_err() );

  // Should be a Render error
  match result.unwrap_err()
  {
    Error ::Render( _ ) => {},
    other => panic!( "Expected Error::Render, got {other:?}" ),
  }
}

#[ test ]
fn handlebars_renderer_missing_variable_renders_empty()
{
  // Handlebars renders missing variables as empty strings by default
  let renderer = HandlebarsRenderer ::new();
  let template = "Value: {{missing}}";

  let values = Values ::< Value >::new();

  let result = renderer.render( template, &values.to_serializable() );
  assert!( result.is_ok() );
  assert_eq!( result.unwrap(), "Value: " );
}

#[ test ]
fn template_renderer_trait_is_implementable()
{
  // FR8: Multiple renderer implementations must be possible

  // Simple custom renderer for testing
  struct CustomRenderer;

  impl TemplateRenderer for CustomRenderer
  {
    fn render
    (
      &self,
      template: &str,
      _values: &std ::collections ::BTreeMap< String, serde_json ::Value >
    )
    -> Result< String, Error >
    {
      Ok( format!( "custom:{template}" ) )
    }
  }

  let renderer = CustomRenderer;
  let values = std ::collections ::BTreeMap ::new();

  let result = renderer.render( "test", &values );
  assert_eq!( result.unwrap(), "custom:test" );
}

#[ test ]
fn handlebars_renderer_handles_numbers()
{
  // FR9: Must correctly render number values
  let renderer = HandlebarsRenderer ::new();
  let template = "Count: {{count}}, Age: {{age}}";

  let mut values = Values ::new();
  values.insert( "count", Value ::Number( 42 ) );
  values.insert( "age", Value ::Number( -10 ) );

  let result = renderer.render( template, &values.to_serializable() );
  assert!( result.is_ok() );
  assert_eq!( result.unwrap(), "Count: 42, Age: -10" );
}

#[ test ]
fn handlebars_renderer_handles_lists()
{
  // FR9: Must correctly render list values
  let renderer = HandlebarsRenderer ::new();
  let template = "Items: {{items}}";

  let mut values = Values ::new();
  values.insert( "items", Value ::List( vec![ "a".into(), "b".into(), "c".into() ] ) );

  let result = renderer.render( template, &values.to_serializable() );
  assert!( result.is_ok() );
  assert_eq!( result.unwrap(), "Items: a, b, c" );
}
