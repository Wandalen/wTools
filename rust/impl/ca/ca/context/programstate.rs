
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

    /// Get number of current instruction
    pub fn get_pos( &self ) -> usize
    {
      self.current_pos
    }

    /// Change the current instruction position to the next one
    /// Returns Some( position ) if it can be done
    /// And None when can not increment position value
    pub fn next( &mut self ) -> Option< usize >
    {
      self.current_pos.checked_add( 1 )
      .map( | pos |
      {
        self.set_pos( pos );
        pos
      })
    }

    /// Change the current instruction position to the previous one
    /// Returns Some( position ) if it can be done
    /// And None when can not increment position value
    pub fn prev( &mut self ) -> Option< usize >
    {
      self.current_pos.checked_sub( 1 )
      .map( | pos |
      {
        self.set_pos( pos );
        pos
      })
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
