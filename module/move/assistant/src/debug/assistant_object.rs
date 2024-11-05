use super::*;
use openai_api_rs::v1::assistant;

/// A wrapper for `AssistantObject` to make pretty print.
#[ derive( Debug ) ]
pub struct AssistantObjectWrap( pub assistant::AssistantObject );

impl Clone for AssistantObjectWrap
{
  fn clone( &self ) -> Self
  {
    // Manually clone each field of the wrapped AssistantObject
    AssistantObjectWrap( assistant::AssistantObject
    {
      id : self.0.id.clone(),
      object : self.0.object.clone(),
      created_at : self.0.created_at,
      name : self.0.name.clone(),
      description : self.0.description.clone(),
      model : self.0.model.clone(),
      instructions : self.0.instructions.clone(),
      tools : self.0.tools.clone(),
      tool_resources : self.0.tool_resources.clone(),
      metadata : self.0.metadata.clone(),
      headers : self.0.headers.clone(),
    } )
  }
}

impl TableWithFields for AssistantObjectWrap {}
impl Fields< &'_ str, Option< Cow< '_, str > > >
for AssistantObjectWrap
{
  type Key< 'k > = &'k str;
  type Val< 'v > = Option< Cow< 'v, str > >;

  fn fields( &self ) -> impl format_tools::IteratorTrait< Item = ( &'_ str, Option< Cow< '_, str > > ) >
  {
    use format_tools::ref_or_display_or_debug_multiline::field;
    let mut dst = Vec::new();

    // Use the field! macro for direct field references
    dst.push( field!( &self.0.id ) );
    dst.push( field!( &self.0.object ) );
    dst.push( field!( &self.0.model ) );

    // Manually handle fields that require function calls
    dst.push( ( "created_at", Some( Cow::Owned( self.0.created_at.to_string() ) ) ) );
    dst.push( ( "name", self.0.name.as_deref().map( Cow::Borrowed ) ) );
    dst.push( ( "description", self.0.description.as_deref().map( Cow::Borrowed ) ) );
    dst.push( ( "instructions", self.0.instructions.as_deref().map( Cow::Borrowed ) ) );

    // Handle complex fields like `tools`, `tool_resources`, `metadata`, and `headers`
    if !self.0.tools.is_empty()
    {
      dst.push( ( "tools", Some( Cow::Borrowed( "tools present" ) ) ) );
    }
    else
    {
      dst.push( ( "tools", Option::None ) );
    }

    if let Some( _metadata ) = &self.0.metadata
    {
      dst.push( ( "metadata", Some( Cow::Borrowed( "metadata present" ) ) ) );
    }
    else
    {
      dst.push( ( "metadata", Option::None ) );
    }

    if let Some( _headers ) = &self.0.headers
    {
      dst.push( ( "headers", Some( Cow::Borrowed( "headers present" ) ) ) );
    }
    else
    {
      dst.push( ( "headers", Option::None ) );
    }

    dst.into_iter()
  }
}
