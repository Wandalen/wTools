use std::borrow::Cow;

/// Helper function to unescape common escape sequences in a string.
/// Returns a `Cow::Borrowed` if no unescaping is needed, otherwise `Cow::Owned`.
pub fn test_unescape_str( input : &str ) -> Cow< '_, str >
{
  if !input.contains( '\\' )
  {
    return Cow::Borrowed( input );
  }

  let mut output = String::with_capacity( input.len() );
  let mut chars = input.chars();

  while let Some( ch ) = chars.next()
  {
    if ch == '\\'
    {
      if let Some( next_ch ) = chars.next()
      {
        match next_ch
        {
          '"' => output.push( '"' ),
          '\\' => output.push( '\\' ),
          'n' => output.push( '\n' ),
          't' => output.push( '\t' ),
          'r' => output.push( '\r' ),
          _ =>
          {
            output.push( '\\' );
            output.push( next_ch );
          }
        }
      }
      else
      {
        output.push( '\\' );
      }
    }
    else
      {
        output.push( ch );
      }
    }

  Cow::Owned( output )
}