use crate::optimization::*;

/// Represents temperature of SA process.
#[ derive( Default, Debug, Display, Clone, Copy, PartialEq, PartialOrd, FromInner, InnerFrom ) ]
#[ derive( Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign ) ]
pub struct Temperature( f64 );

impl Temperature
{
  /// Returns inner value of Temperature struct.
  pub fn unwrap( &self ) -> f64
  {
    self.0
  }
}

/// Transforms f32 value into Temperature.
impl From< f32 > for Temperature
{
  #[ inline ]
  fn from( src : f32 ) -> Self
  {
    Self( src as f64 )
  }
}

/// Struct that represents coefficient to change temperature value.
#[ derive( Debug, Display, Clone, Copy, PartialEq, PartialOrd, FromInner, InnerFrom ) ]
#[ derive( Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign ) ]
pub struct TemperatureFactor( f64 );

impl TemperatureFactor
{
  /// Returns inner value of TemperatureFactor struct.
  pub fn unwrap( &self ) -> f64
  {
    self.0
  }
}

/// Default value of TemperatureFactor struct.
impl Default for TemperatureFactor
{
  fn default() -> Self
  {
    0.001.into()
  }
}

/// Transforms f32 value into TemperatureFactor.
impl From< f32 > for TemperatureFactor
{
  #[ inline ]
  fn from( src : f32 ) -> Self
  {
    Self( src as f64 )
  }
}

/// Represents initial configuration of SA optimization process for sudoku solving.
#[ derive( Clone, Debug ) ]
pub struct SAConfig
{
    /// Max amount of mutations in generation.
    pub n_mutations_per_generation_limit : usize,
    /// Max allowed number of resets.
    pub n_resets_limit : usize,
    /// Max number of generations created during SA process.
    pub n_generations_limit : usize,
    /// Coefficient for lowering SA temperature.
    pub temperature_decrease_factor : TemperatureFactor,
    /// Coefficient for increasing SA temperature during reset.
    pub temperature_increase_factor : TemperatureFactor,
}

impl SAConfig
{
  /// Calculate the initial temperature for the optimization process.
  pub fn initial_temperature( &self, hrng : Hrng, initial_board : &Board, person : &SudokuPerson ) -> Temperature
  {
    use statrs::statistics::Statistics;
    let state = person.clone();
    const N : usize = 16;
    let mut costs : [ f64 ; N ] = [ 0.0 ; N ];
    for i in 0..N
    {
      let state2 = state.mutate_random( initial_board, hrng.clone() );
      costs[ i ] = state2.cost.into();
    }
    costs[..].std_dev().into()
  }
  /// Set temperature increase factor.
  pub fn set_temp_decrease_factor( &mut self, factor : f64 )
  {
    self.temperature_decrease_factor = factor.into();
  }

  /// Set temperature decrease factor.
  pub fn set_temp_increase_factor( &mut self, factor : f64 )
  {
    self.temperature_increase_factor = factor.into();
  }

  /// Set max amount of mutations per one generation.
  pub fn set_mutations_per_generation( &mut self, number : usize )
  {
    self.n_mutations_per_generation_limit = number;
  }
}

impl Default for SAConfig
{
  fn default() -> Self
  {
    Self
    {
      temperature_decrease_factor : Default::default(),
      temperature_increase_factor : 1.0f64.into(),
      n_mutations_per_generation_limit : 2_000,
      n_resets_limit : 1_000,
      n_generations_limit : 1_000_000,
    }
  }
}