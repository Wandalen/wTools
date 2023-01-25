
pub( crate ) mod private
{
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

    /// Returns the number of next instruction
    pub fn next( &self ) -> usize
    {
      self.current_pos + 1
    }

    /// Sets current instruction to next one
    pub fn go_next( &mut self )
    {
      self.current_pos += 1
    }

    /// Returns the number of previous instruction
    pub fn prev( &self ) -> usize
    {
      self.current_pos - 1
    }

    /// Sets current instruction to previous one
    pub fn go_back( &mut self )
    {
      self.current_pos -= 1
    }

    /// Get number of current instruction
    pub fn get_pos( &self ) -> usize
    {
      self.current_pos
    }

    /// Sets current instruction to end of program
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
