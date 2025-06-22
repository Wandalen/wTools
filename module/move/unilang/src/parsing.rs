//!
//! The parsing components for the Unilang framework, including the lexer and parser.
//!

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
  /// Reads a number literal (integer or float) from the input.
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
  /// Creates a new `Parser` from a `Lexer`.
  ///
  pub fn new( lexer : Lexer< 'a > ) -> Self
  {
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
  pub fn parse_program( &mut self ) -> Program
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