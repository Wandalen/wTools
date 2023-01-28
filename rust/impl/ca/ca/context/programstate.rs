
pub( crate ) mod private
{
  use wtools::BasicError;

  /// Execution statement of a program
  #[ derive( Debug, Default ) ]
  pub struct ProgramState
  {
    /// Current instruction number
    current_pos : usize
  }

  impl ProgramState
  {
    /// Sets current instruction
    pub fn set_pos( &mut self, pos : usize )
    {
      self.current_pos = pos;
    }

    /// Get number of current instruction
    pub fn get_pos( &self ) -> usize
    {
      self.current_pos
    }

    /// Returns the number of next instruction
    pub fn next( &self ) -> usize
    {
      self.checked_next().unwrap_or( usize::MAX )
    }

    /// Checked position addition. Computes self + 1, returning None if overflow occurred.
    pub fn checked_next( &self ) -> Option< usize >
    {
      self.current_pos.checked_add( 1 )
    }

    /// Sets current instruction to next one
    pub fn go_next( &mut self )
    {
      // if current instruction is the last available, on next iteration it will be usize::MAX + 1 and it stop safe
      self.checked_go_next().ok();
    }

    /// Checked go to the next instruction. If it was the last instruction - returns Error
    pub fn checked_go_next( &mut self ) -> Result< (), BasicError >
    {
       self.checked_next()
       .ok_or( BasicError::new( "It was the last instruction" ) )
       .map( | pos | self.set_pos( pos ) )
    }

    /// Returns the number of previous instruction
    pub fn prev( &self ) -> usize
    {
      self.checked_prev().unwrap_or( usize::MIN )
    }

    /// Checked position subtraction. Computes self - 1, returning None if overflow occurred.
    pub fn checked_prev( &self ) -> Option< usize >
    {
      self.current_pos.checked_sub( 1 )
    }

    /// Sets current instruction to previous one
    pub fn go_back( &mut self )
    {
      self.checked_go_back().ok();
    }

    /// Checked go to the previous instruction. If it was the first instruction - returns Error
    pub fn checked_go_back( &mut self ) -> Result< (), BasicError >
    {
       self.checked_prev()
       .ok_or( BasicError::new( "It was the first instruction" ) )
       .map( | pos | self.set_pos( pos ) )
    }

    /// Sets current instruction to begin of a program
    pub fn start( &mut self )
    {
      self.set_pos( usize::MIN )
    }

    /// Sets current instruction to end of a program
    /// Last instruction will not be executed
    pub fn finish( &mut self )
    {
      self.set_pos( usize::MAX )
    }
  }
}

//

crate::mod_interface!
{
  prelude use ProgramState;
}
