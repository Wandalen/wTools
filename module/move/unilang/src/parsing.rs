//!
//! The parsing components for the Unilang framework, including the lexer and parser.
//!
use core::fmt; // Changed from std::fmt

///
/// Represents a token in the Unilang language.
///
/// Tokens are the smallest individual units of meaning in the language,
/// produced by the `Lexer` and consumed by the `Parser`.
#[ derive( Debug, PartialEq, Clone ) ]
pub enum Token
{
  /// A command or argument name (e.g., `my_command`, `arg1`).
  Identifier( String ),
  /// A string literal (e.g., `"hello world"`).
  String( String ),
  /// An integer literal (e.g., `123`, `-45`).
  Integer( i64 ),
  /// A float literal (e.g., `1.23`).
  Float( f64 ),
  /// A boolean literal (`true` or `false`).
  Boolean( bool ),
  /// The command separator `;;`.
  CommandSeparator,
  /// Represents the end of the input string.
  Eof,
}

impl fmt::Display for Token
{
  fn fmt( &self, f: &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    match self
    {
      Token::Identifier( s ) | Token::String( s ) => write!( f, "{s}" ), // Combined match arms
      Token::Integer( i ) => write!( f, "{i}" ),
      Token::Float( fl ) => write!( f, "{fl}" ),
      Token::Boolean( b ) => write!( f, "{b}" ),
      Token::CommandSeparator => write!( f, ";;" ),
      Token::Eof => write!( f, "EOF" ),
    }
  }
}


///
/// The lexer for the Unilang language.
///
/// The lexer is responsible for breaking the input string into a sequence of tokens.
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
  /// Creates a new `Lexer` from an input string.
  ///
  #[must_use]
  pub fn new( input : &'a str ) -> Self
  {
    let mut lexer = Lexer
    {
      input,
      position : 0,
      read_position : 0,
      ch : 0,
    };
    lexer.read_char();
    lexer
  }

  ///
  /// Reads the next character from the input and advances the position.
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
  /// Peeks at the next character in the input without consuming it.
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

  ///
  /// Skips any whitespace characters.
  ///
  fn skip_whitespace( &mut self )
  {
    while self.ch.is_ascii_whitespace()
    {
      self.read_char();
    }
  }

  ///
  /// Reads a "word" or an unquoted token from the input. A word is any sequence
  /// of characters that is not whitespace and does not contain special separators.
  ///
  fn read_word( &mut self ) -> String
  {
    let position = self.position;
    while !self.ch.is_ascii_whitespace() && self.ch != 0
    {
      // Stop before `;;`
      if self.ch == b';' && self.peek_char() == b';'
      {
        break;
      }
      self.read_char();
    }
    self.input[ position..self.position ].to_string()
  }

  ///
  /// Reads a string literal from the input, handling the enclosing quotes and escapes.
  ///
  fn read_string( &mut self ) -> String
  {
    let quote_char = self.ch;
    self.read_char(); // Consume the opening quote
    let mut s = String::new();
    loop
    {
      if self.ch == 0
      {
        // xxx: Handle unterminated string error
        break;
      }
      if self.ch == b'\\'
      {
        self.read_char(); // Consume '\'
        match self.ch
        {
          b'n' => s.push( '\n' ),
          b't' => s.push( '\t' ),
          b'r' => s.push( '\r' ),
          _ => s.push( self.ch as char ), // Push the escaped character itself
        }
      }
      else if self.ch == quote_char
      {
        break;
      }
      else
      {
        s.push( self.ch as char );
      }
      self.read_char();
    }
    self.read_char(); // Consume the closing quote
    s
  }

  ///
  /// Returns the next token from the input.
  ///
  /// # Panics
  ///
  /// Panics if parsing a float from a string fails, which should only happen
  /// if the string is not a valid float representation.
  pub fn next_token( &mut self ) -> Token
  {
    self.skip_whitespace();

    match self.ch
    {
      b';' =>
      {
        if self.peek_char() == b';'
        {
          self.read_char(); // consume first ;
          self.read_char(); // consume second ;
          Token::CommandSeparator
        }
        else
        {
          // A single semicolon is just part of a word/identifier
          let word = self.read_word();
          Token::Identifier( word )
        }
      }
      b'"' | b'\'' => // Handle both single and double quotes
      {
        let s = self.read_string();
        Token::String( s )
      }
      0 => Token::Eof,
      _ =>
      {
        let word = self.read_word();
        if word == "true"
        {
          Token::Boolean( true )
        }
        else if word == "false"
        {
          Token::Boolean( false )
        }
        else if let Ok( i ) = word.parse::< i64 >()
        {
          if word.contains( '.' )
          {
            // It's a float that happens to parse as an int (e.g. "1.0")
            // so we parse as float
            Token::Float( word.parse::< f64 >().unwrap() )
          }
          else
          {
            Token::Integer( i )
          }
        }
        else if let Ok( f ) = word.parse::< f64 >()
        {
          Token::Float( f )
        }
        else
        {
          Token::Identifier( word )
        }
      }
    }
  }
}

///
/// Represents a single command statement in the AST.
///
#[ derive( Debug, PartialEq, Clone ) ]
pub struct Statement
{
  /// The command identifier.
  pub command : String,
  /// The arguments for the command.
  pub args : Vec< Token >,
}

///
/// Represents a program, which is a series of statements.
///
/// This is the root of the Abstract Syntax Tree (AST).
#[ derive( Debug, Default ) ]
pub struct Program
{
  /// The statements that make up the program.
  pub statements : Vec< Statement >,
}

///
/// The parser for the Unilang language.
///
/// The parser takes a `Lexer` and produces an Abstract Syntax Tree (AST)
/// represented by a `Program` struct.
#[ derive( Debug ) ]
pub struct Parser< 'a >
{
  lexer : Lexer< 'a >,
  current_token : Token,
  peek_token : Token,
}

impl< 'a > Parser< 'a >
{
  ///
  /// Creates a new `Parser` from an input string.
  ///
  #[must_use]
  pub fn new( input: &'a str ) -> Self
  {
    let lexer = Lexer::new( input );
    let mut parser = Parser
    {
      lexer,
      current_token : Token::Eof,
      peek_token : Token::Eof,
    };
    // Prime the parser with the first two tokens.
    parser.next_token();
    parser.next_token();
    parser
  }

  ///
  /// Advances the parser to the next token.
  ///
  fn next_token( &mut self )
  {
    self.current_token = self.peek_token.clone();
    self.peek_token = self.lexer.next_token();
  }

  ///
  /// Parses the entire input and returns a `Program` AST.
  ///
  pub fn parse( &mut self ) -> Program
  {
    let mut program = Program::default();

    while self.current_token != Token::Eof
    {
      if let Some( statement ) = self.parse_statement()
      {
        program.statements.push( statement );
      }
      else
      {
        // If it's not a valid statement, skip the token to avoid infinite loops on invalid input.
        self.next_token();
      }
    }

    program
  }

  ///
  /// Parses a single statement.
  ///
  fn parse_statement( &mut self ) -> Option< Statement >
  {
    if let Token::Identifier( command ) = self.current_token.clone()
    {
      let mut args = Vec::new();
      self.next_token(); // Consume command identifier.
      while self.current_token != Token::CommandSeparator && self.current_token != Token::Eof
      {
        args.push( self.current_token.clone() );
        self.next_token();
      }

      // Consume the separator if it exists, to be ready for the next statement.
      if self.current_token == Token::CommandSeparator
      {
        self.next_token();
      }

      Some( Statement { command, args } )
    }
    else
    {
      None
    }
  }
}