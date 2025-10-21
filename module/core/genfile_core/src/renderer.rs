/// Template rendering engines
use std ::collections ::BTreeMap;
use crate ::Error;
use serde_json ::Value as JsonValue;

/// Template rendering engine trait.
///
/// Abstracts template processing behind a trait to enable pluggable rendering
/// engines. The default implementation uses Handlebars, but users can implement
/// custom renderers for other template syntaxes.
///
/// # Examples
///
/// ```rust,ignore
/// use genfile_core::{ TemplateRenderer, HandlebarsRenderer, Error, Values, Value };
///
/// let renderer = HandlebarsRenderer::new();
/// let template = "Hello {{name}}!";
/// let mut values = Values::new();
/// values.insert( "name", Value::String( "World".into() ) );
///
/// let result = renderer.render( template, &values.to_serializable() )?;
/// assert_eq!( result, "Hello World!" );
/// # Ok::<(), Error>(())
/// ```
pub trait TemplateRenderer
{
  /// Renders a template with provided values.
  ///
  /// # Parameters
  ///
  /// - `template`: Template content with placeholders
  /// - `values`: Variable values for substitution (with preserved types)
  ///
  /// # Returns
  ///
  /// Rendered string on success, Error on failure
  ///
  /// # Errors
  ///
  /// Returns `Error::Render` if template syntax is invalid or rendering fails.
  fn render
  (
    &self,
    template: &str,
    values: &BTreeMap< String, JsonValue >
  )
  -> Result< String, Error >;
}

/// Handlebars-based template renderer.
///
/// Default renderer implementation using Handlebars 4.5.0 with HTML escaping
/// disabled for safe use with code generation templates.
///
/// # Features
///
/// - Variable substitution: `{{variable}}`
/// - Conditional logic: `{{#if condition}}...{{/if}}`
/// - No HTML escaping (safe for code generation)
///
/// # Examples
///
/// ```rust
/// use genfile_core::{ HandlebarsRenderer, TemplateRenderer, Values, Value };
///
/// let renderer = HandlebarsRenderer::new();
/// let template = "Project: {{name}}";
///
/// let mut values = Values::new();
/// values.insert( "name", Value::String( "genfile".into() ) );
///
/// let result = renderer.render( template, &values.to_serializable() );
/// assert!( result.is_ok() );
/// ```
#[ derive( Debug ) ]
pub struct HandlebarsRenderer
{
  /// Internal Handlebars instance
  handlebars: handlebars ::Handlebars< 'static >,
}

impl HandlebarsRenderer
{
  /// Creates a new Handlebars renderer.
  ///
  /// Initializes Handlebars with HTML escaping disabled for code generation use.
  ///
  /// # Returns
  ///
  /// New `HandlebarsRenderer` instance
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::HandlebarsRenderer;
  ///
  /// let renderer = HandlebarsRenderer::new();
  /// ```
  pub fn new() -> Self
  {
    let mut handlebars = handlebars ::Handlebars ::new();

    // Disable HTML escaping for code generation templates
    handlebars.register_escape_fn( handlebars ::no_escape );

    Self
    {
      handlebars,
    }
  }
}

impl Default for HandlebarsRenderer
{
  fn default() -> Self
  {
    Self ::new()
  }
}

impl TemplateRenderer for HandlebarsRenderer
{
  fn render
  (
    &self,
    template: &str,
    values: &BTreeMap< String, JsonValue >
  )
  -> Result< String, Error >
  {
    // Render template directly without registration
    // This allows one-time template rendering
    // Values are passed as JSON to preserve type information for conditionals
    self
      .handlebars
      .render_template( template, values )
      .map_err( | e | Error ::Render( e.to_string() ) )
  }
}
