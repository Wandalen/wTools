mod private
{
  /// The `ProgressBar` structure is used to display progress indicators in the terminal.
  /// It wraps the functionality of the `indicatif` library.
  ///
  /// This structure is only available when the `progress_bar` feature is enabled.
  #[ cfg( feature = "progress_bar" ) ]
  pub struct ProgressBar< 'a >
  {
    /// A reference to the `MultiProgress` object from the `indicatif` library, which
    /// allows managing multiple progress bars simultaneously. This object is necessary
    /// for coordinating the display of multiple progress bars.
    pub( crate ) multi_progress: &'a indicatif::MultiProgress,
    /// The `ProgressBar` object from the `indicatif` library, which represents
    /// an individual progress indicator. It is used to update the progress state
    /// and display it in the terminal.
    pub( crate ) progress_bar: indicatif::ProgressBar,
  }

  #[ cfg( feature = "progress_bar" ) ]
  impl < 'a > std::fmt::Debug for ProgressBar< 'a >
  {
    fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
    {
      f.debug_struct( "ProgressBar" )
        .finish()
    }
  }

  /// The `MultiProgress` structure is used to manage and display multiple progress
  /// indicators simultaneously in the terminal. It utilizes the `indicatif` library.
  ///
  /// This structure is only available when the `progress_bar` feature is enabled.
  #[ cfg( feature = "progress_bar" ) ]
  pub struct MultiProgress
  {
    multi_progress: indicatif::MultiProgress,
    progress_style: indicatif::ProgressStyle,
  }

  #[ cfg( feature = "progress_bar" ) ]
  impl MultiProgress
  {
    /// Creates a new `ProgressBar` instance tied to the `MultiProgress` manager.
    /// This function initializes a new progress bar with a specified length and applies
    /// the defined style to it.
    ///
    /// # Parameters
    ///
    /// - `variants_len`: The total length or count that the progress bar will track.
    ///
    /// # Returns
    ///
    /// A `ProgressBar` instance that can be used to update and display progress.
    pub fn progress_bar< 'a >( &'a self, variants_len : u64 ) -> ProgressBar< 'a >
    {
      let progress_bar =
        {
          let pb = self.multi_progress.add( indicatif::ProgressBar::new( variants_len ) );
          pb.set_style( self.progress_style.clone() );
          pb.inc( 0 );
          pb
        };
      ProgressBar
      {
        multi_progress : &self.multi_progress,
        progress_bar,
      }
    }
  }

  #[ cfg( feature = "progress_bar" ) ]
  impl std::fmt::Debug for MultiProgress
  {
    fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
    {
      f.debug_struct( "MultiprogressProgress" )
        .finish()
    }
  }


  #[ cfg( feature = "progress_bar" ) ]
  impl Default for MultiProgress
  {
    fn default() -> Self
    {
      Self
      {
        multi_progress: indicatif::MultiProgress::new(),
        progress_style: indicatif::ProgressStyle::with_template
          (
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
          )
          .unwrap()
          .progress_chars( "##-" ),
      }
    }
  }
}

crate::mod_interface!
{
  #[ cfg( feature = "progress_bar" ) ]
  protected use ProgressBar;
  #[ cfg( feature = "progress_bar" ) ]
  protected use MultiProgress;
}