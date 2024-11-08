
use super::*;
use openai_api_rs::v1::run::RunObject;

// Assuming the `format_tools` module and `field!` macro are defined elsewhere

/// A wrapper for `RunObject` to make pretty print.
#[ derive( Debug ) ]
pub struct RunObjectWrap( pub RunObject );

/// Manually implemented `Clone`, as `RunObject` does not implement it.
impl Clone for RunObjectWrap {
  fn clone(&self) -> Self {
    RunObjectWrap(RunObject {
      id : self.0.id.clone(),
      object : self.0.object.clone(),
      created_at : self.0.created_at,
      thread_id : self.0.thread_id.clone(),
      assistant_id : self.0.assistant_id.clone(),
      status : self.0.status.clone(),
      required_action : self.0.required_action.clone(),
      last_error : self.0.last_error.clone(),
      expires_at : self.0.expires_at,
      started_at : self.0.started_at,
      cancelled_at : self.0.cancelled_at,
      failed_at : self.0.failed_at,
      completed_at : self.0.completed_at,
      model : self.0.model.clone(),
      instructions : self.0.instructions.clone(),
      tools : self.0.tools.clone(),
      metadata : self.0.metadata.clone(),
      headers : self.0.headers.clone(),
    })
  }
}

impl TableWithFields for RunObjectWrap {}
impl Fields< &'_ str, Option< Cow< '_, str > > >
for RunObjectWrap
{
  type Key< 'k > = &'k str;
  type Val< 'v > = Option< Cow< 'v, str > >;

  fn fields( &self ) -> impl format_tools::IteratorTrait< Item = ( &'_ str, Option< Cow< '_, str > > ) >
  {
    use format_tools::ref_or_display_or_debug_multiline::field;
    let mut dst = Vec::new();

    dst.push( field!( &self.0.id ) );
    dst.push( field!( &self.0.object ) );
    dst.push( ( "created_at", Some( Cow::Owned( self.0.created_at.to_string() ) ) ) );
    dst.push( field!( &self.0.thread_id ) );
    dst.push( field!( &self.0.assistant_id ) );
    dst.push( field!( &self.0.status ) );

    dst.push( ( "required_action", self.0.required_action.as_ref().map( |ra| Cow::Owned( format!( "{:?}", ra ) ) ) ) );
    dst.push( ( "last_error", self.0.last_error.as_ref().map( |le| Cow::Owned( format!( "{:?}", le ) ) ) ) );
    dst.push( ( "expires_at", self.0.expires_at.map( |ea| Cow::Owned( ea.to_string() ) ) ) );
    dst.push( ( "started_at", self.0.started_at.map( |sa| Cow::Owned( sa.to_string() ) ) ) );
    dst.push( ( "cancelled_at", self.0.cancelled_at.map( |ca| Cow::Owned( ca.to_string() ) ) ) );
    dst.push( ( "failed_at", self.0.failed_at.map( |fa| Cow::Owned( fa.to_string() ) ) ) );
    dst.push( ( "completed_at", self.0.completed_at.map( |ca| Cow::Owned( ca.to_string() ) ) ) );

    dst.push( field!( &self.0.model ) );
    dst.push( ( "instructions", self.0.instructions.as_ref().map( |i| Cow::Owned( i.clone() ) ) ) );

    dst.push( ( "tools", Some( Cow::Owned( format!( "{:?}", self.0.tools ) ) ) ) );
    dst.push( ( "metadata", Some( Cow::Owned( format!( "{:?}", self.0.metadata ) ) ) ) );
    dst.push( ( "headers", self.0.headers.as_ref().map( |h| Cow::Owned( format!( "{:?}", h ) ) ) ) );

    dst.into_iter()
  }
}
