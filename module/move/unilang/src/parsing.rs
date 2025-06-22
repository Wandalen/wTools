//!
//! The parsing components for the Unilang framework.
//!

///
/// Represents a token in the Unilang language.
///
#[ derive( Debug, PartialEq, Clone ) ]
pub enum Token
{
  /// A command or argument name.
  Identifier( String ),
  /// A string literal.
  String( String ),
  /// An integer literal.
  Integer( i64 ),
  /// A float literal.
  Float( f64 ),
  /// A boolean literal.
  Boolean( bool ),
  /// The command separator `;;`.
  CommandSeparator,
  /// End of input.
  Eof,
}

///
/// The lexer for the Unilang language.
///
#[ derive( Debug ) ]
pub struct Lexer< 'a >
{
  input : &'a str,
  position : usize,
  read_position : usize,
  ch : u8,
}

impl< 'a > Lexer< 'a >
{
  ///
  /// Creates a new `Lexer`.
  ///
  pub fn new( input : &'a str ) -> Self
  {
    let mut lexer = Lexer {
      input,
      position : 0,
      read_position : 0,
      ch : 0,
    };
    lexer.read_char();
    lexer
  }

  ///
  /// Reads the next character from the input.
  ///
  fn read_char( &mut self )
  {
    if self.read_position >= self.input.len()
    {
      self.ch = 0;
    }
    else
    {
      self.ch = self.input.as_bytes()[ self.read_position ];
    }
    self.position = self.read_position;
    self.read_position += 1;
  }

  ///
  /// Returns the next token from the input.
  ///
  pub fn next_token( &mut self ) -> Token
  {
    self.skip_whitespace();

    let token = match self.ch
    {
      b';' =>
      {
        if self.peek_char() == b';'
        {
          self.read_char();
          Token::CommandSeparator
        }
        else
        {
          // Handle single semicolon as an identifier or error
          let ident = self.read_identifier();
          return Token::Identifier( ident );
        }
      }
      b'a'..=b'z' | b'A'..=b'Z' | b'_' =>
      {
        let ident = self.read_identifier();
        return match ident.as_str()
        {
          "true" => Token::Boolean( true ),
          "false" => Token::Boolean( false ),
          _ => Token::Identifier( ident ),
        };
      }
      b'"' =>
      {
        let string = self.read_string();
        Token::String( string )
      }
      b'0'..=b'9' =>
      {
        return self.read_number();
      }
      0 => Token::Eof,
      _ => Token::Identifier( self.read_identifier() ),
    };

    self.read_char();
    token
  }

  ///
  /// Skips whitespace characters.
  ///
  fn skip_whitespace( &mut self )
  {
    while self.ch.is_ascii_whitespace()
    {
      self.read_char();
    }
  }

  ///
  /// Reads an identifier from the input.
  ///
  fn read_identifier( &mut self ) -> String
  {
    let position = self.position;
    while self.ch.is_ascii_alphanumeric() || self.ch == b'_'
    {
      self.read_char();
    }
    self.input[ position..self.position ].to_string()
  }

  ///
  /// Reads a string literal from the input.
  ///
  fn read_string( &mut self ) -> String
  {
    let position = self.position + 1;
    loop
    {
      self.read_char();
      if self.ch == b'"' || self.ch == 0
      {
        break;
      }
    }
    self.input[ position..self.position ].to_string()
  }

  ///
  /// Reads a number literal from the input.
  ///
  fn read_number( &mut self ) -> Token
  {
    let position = self.position;
    let mut is_float = false;
    while self.ch.is_ascii_digit()
    {
      self.read_char();
    }
    if self.ch == b'.' && self.peek_char().is_ascii_digit()
    {
      is_float = true;
      self.read_char();
      while self.ch.is_ascii_digit()
      {
        self.read_char();
      }
    }

    let number_str = &self.input[ position..self.position ];
    if is_float
    {
      Token::Float( number_str.parse().unwrap() )
    }
    else
    {
      Token::Integer( number_str.parse().unwrap() )
    }
  }

  ///
  /// Peeks at the next character without consuming it.
  ///
  fn peek_char( &self ) -> u8
  {
    if self.read_position >= self.input.len()
    {
      0
    }
    else
    {
      self.input.as_bytes()[ self.read_position ]
    }
  }
}