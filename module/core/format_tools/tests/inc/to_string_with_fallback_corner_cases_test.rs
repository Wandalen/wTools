//! Corner case tests for `to_string_with_fallback` functionality
//!
//! Tests edge cases including empty strings, unicode, long strings,
//! and special characters to ensure robust formatting behavior.

#![ allow( clippy ::no_effect_underscore_binding ) ]

#[ allow( unused_imports ) ]
use super :: *;
use test_tools :: { a_id, a_true };

use the_module ::
{
  WithDebug,
  WithDisplay,
  to_string_with_fallback
};

use std :: { fmt, borrow ::Cow };

//

#[ test ]
fn empty_string_display()
{
  struct EmptyDisplay;
  impl fmt ::Display for EmptyDisplay
  {
    fn fmt( &self, f: &mut fmt ::Formatter< '_ > ) -> fmt ::Result
    {
      write!( f, "" )
    }
  }
  impl fmt ::Debug for EmptyDisplay
  {
    fn fmt( &self, f: &mut fmt ::Formatter< '_ > ) -> fmt ::Result
    {
      write!( f, "debug" )
    }
  }

  let src = EmptyDisplay;
  let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  let exp: Cow< '_, str > = Cow ::Owned( String ::new() );
  a_id!( got, exp );
  a_true!( got.is_empty() );
}

//

#[ test ]
fn empty_string_debug()
{
  struct EmptyDebug;
  impl fmt ::Debug for EmptyDebug
  {
    fn fmt( &self, f: &mut fmt ::Formatter< '_ > ) -> fmt ::Result
    {
      write!( f, "" )
    }
  }

  let src = EmptyDebug;
  let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  let exp: Cow< '_, str > = Cow ::Owned( String ::new() );
  a_id!( got, exp );
  a_true!( got.is_empty() );
}

//

#[ test ]
fn unicode_characters()
{
  struct UnicodeDisplay;
  impl fmt ::Display for UnicodeDisplay
  {
    fn fmt( &self, f: &mut fmt ::Formatter< '_ > ) -> fmt ::Result
    {
      write!( f, "Hello 世界 🌍 مرحبا Привіт" )
    }
  }
  impl fmt ::Debug for UnicodeDisplay
  {
    fn fmt( &self, f: &mut fmt ::Formatter< '_ > ) -> fmt ::Result
    {
      write!( f, "debug" )
    }
  }

  let src = UnicodeDisplay;
  let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  let exp: Cow< '_, str > = Cow ::Owned( "Hello 世界 🌍 مرحبا Привіт".to_string() );
  a_id!( got, exp );
}

//

#[ test ]
fn newline_characters()
{
  struct NewlineDisplay;
  impl fmt ::Display for NewlineDisplay
  {
    fn fmt( &self, f: &mut fmt ::Formatter< '_ > ) -> fmt ::Result
    {
      write!( f, "Line 1\nLine 2\nLine 3" )
    }
  }
  impl fmt ::Debug for NewlineDisplay
  {
    fn fmt( &self, f: &mut fmt ::Formatter< '_ > ) -> fmt ::Result
    {
      write!( f, "debug" )
    }
  }

  let src = NewlineDisplay;
  let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  let exp: Cow< '_, str > = Cow ::Owned( "Line 1\nLine 2\nLine 3".to_string() );
  a_id!( got, exp );
  a_true!( got.contains( '\n' ) );
}

//

#[ test ]
fn tab_characters()
{
  struct TabDisplay;
  impl fmt ::Display for TabDisplay
  {
    fn fmt( &self, f: &mut fmt ::Formatter< '_ > ) -> fmt ::Result
    {
      write!( f, "Col1\tCol2\tCol3" )
    }
  }
  impl fmt ::Debug for TabDisplay
  {
    fn fmt( &self, f: &mut fmt ::Formatter< '_ > ) -> fmt ::Result
    {
      write!( f, "debug" )
    }
  }

  let src = TabDisplay;
  let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  let exp: Cow< '_, str > = Cow ::Owned( "Col1\tCol2\tCol3".to_string() );
  a_id!( got, exp );
  a_true!( got.contains( '\t' ) );
}

//

#[ test ]
fn very_long_string()
{
  struct LongDisplay;
  impl fmt ::Display for LongDisplay
  {
    fn fmt( &self, f: &mut fmt ::Formatter< '_ > ) -> fmt ::Result
    {
      write!( f, "{}", "x".repeat( 1000 ) )
    }
  }
  impl fmt ::Debug for LongDisplay
  {
    fn fmt( &self, f: &mut fmt ::Formatter< '_ > ) -> fmt ::Result
    {
      write!( f, "debug" )
    }
  }

  let src = LongDisplay;
  let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  a_true!( got.len() == 1000 );
  a_true!( got.chars().all( | c | c == 'x' ) );
}

//

#[ test ]
fn carriage_return_characters()
{
  struct CrDisplay;
  impl fmt ::Display for CrDisplay
  {
    fn fmt( &self, f: &mut fmt ::Formatter< '_ > ) -> fmt ::Result
    {
      write!( f, "Line 1\r\nLine 2\r\nLine 3" )
    }
  }
  impl fmt ::Debug for CrDisplay
  {
    fn fmt( &self, f: &mut fmt ::Formatter< '_ > ) -> fmt ::Result
    {
      write!( f, "debug" )
    }
  }

  let src = CrDisplay;
  let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  let exp: Cow< '_, str > = Cow ::Owned( "Line 1\r\nLine 2\r\nLine 3".to_string() );
  a_id!( got, exp );
  a_true!( got.contains( "\r\n" ) );
}

//
