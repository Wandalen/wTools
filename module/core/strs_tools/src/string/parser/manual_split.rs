//! Manual split iterator with per-token validation.

use super::ParseError;

/// Manual split iterator for validation that preserves lifetime references
pub struct ManualSplitIterator< 'a, F >
{
  /// Input string to split
  input: &'a str,
  /// Delimiters to split on
  delimiters: Vec< &'a str >,
  /// Validation function for each token
  validator: F,
  /// Current position in input string
  position: usize,
}

impl< 'a, F > std ::fmt ::Debug for ManualSplitIterator< 'a, F >
{
  fn fmt( &self, f: &mut std ::fmt ::Formatter< '_ > ) -> std ::fmt ::Result
  {
  f.debug_struct( "ManualSplitIterator" )
   .field( "input", &self.input )
   .field( "delimiters", &self.delimiters )
   .field( "position", &self.position )
   .field( "validator", &"< function >" )
   .finish()
 }
}

impl< 'a, F > ManualSplitIterator< 'a, F >
where
  F: Fn( &str ) -> bool,
{
  /// Create a new manual split iterator with validation
  pub fn new( input: &'a str, delimiters: &'a [ &'a str ], validator: F ) -> Self
  {
  Self
  {
   input,
   delimiters: delimiters.to_vec(),
   validator,
   position: 0,
 }
 }

  fn find_next_token( &mut self ) -> Option< &'a str >
  {
  loop
  {
   if self.position >= self.input.len()
   {
  return None;
 }

   let remaining = &self.input[ self.position.. ];
   
   // Find the earliest delimiter match
   let mut earliest_delim_pos = None;
   let mut earliest_delim_len = 0;
   
   for delim in &self.delimiters
   {
  if let Some( pos ) = remaining.find( delim )
  {
   match earliest_delim_pos
   {
  None => 
  {
   earliest_delim_pos = Some( pos );
   earliest_delim_len = delim.len();
 },
  Some( current_pos ) if pos < current_pos =>
  {
   earliest_delim_pos = Some( pos );
   earliest_delim_len = delim.len();
 },
  _ => {} // Keep current earliest
 }
 }
 }
   
   let (token_start, token_end) = if let Some( delim_pos ) = earliest_delim_pos
   {
  // Token is everything before the delimiter
  let token_start = self.position;
  let token_end = self.position + delim_pos;
  self.position += delim_pos + earliest_delim_len;
  (token_start, token_end)
 }
   else
   {
  // No delimiter found, rest of input is the token
  let token_start = self.position;
  let token_end = self.input.len();
  self.position = self.input.len();
  (token_start, token_end)
 };
   
   if token_start < token_end
   {
  return Some( &self.input[ token_start..token_end ] );
 }
   // If token is empty, continue loop to find next non-empty token
 }
 }
}

impl< 'a, F > Iterator for ManualSplitIterator< 'a, F >
where
  F: Fn( &str ) -> bool,
{
  type Item = Result< &'a str, ParseError >;

  fn next( &mut self ) -> Option< Self ::Item >
  {
  let token = self.find_next_token()?;
  
  if ( self.validator )( token )
  {
   Some( Ok( token ) )
 }
  else
  {
   Some( Err( ParseError ::ValidationFailed
   {
  token: token.to_string(),
  position: self.position,
  reason: "Validation failed".to_string(),
 } ) )
 }
 }
}
