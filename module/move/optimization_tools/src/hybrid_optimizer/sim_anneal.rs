//! Implementation of Simulated Annealing for Hybrid Optimizer.

use derive_tools::{ From, InnerFrom, exposed::Display };
/// Represents temperature of SA process.
#[ derive( Default, Debug, Display, Clone, Copy, PartialEq, PartialOrd, From, InnerFrom ) ]
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

// use derive_tools::{ Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign };

/// Struct that represents coefficient to change temperature value.
#[ derive( Debug, Display, Clone, Copy, PartialEq, PartialOrd, From, InnerFrom ) ]
// #[ derive( Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign ) ]
pub struct TemperatureFactor( pub f64 );

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

/// Functionality of temperature schedule for SA responsible for updating temperature value.
pub trait TemperatureSchedule : std::fmt::Debug
{
  /// Calculate next temperature value from current value.
  fn calculate_next_temp( &self, prev_temp : Temperature ) -> Temperature;

  /// Update temperature for reset in SA.
  fn reset_temperature( &self, prev_temp : Temperature ) -> Temperature;
}

/// Temperature schedule for SA that uses linear function for calculation of new temperature value.
#[ derive( Debug, Clone ) ]
pub struct LinearTempSchedule
{
  /// Constant term of linear function.
  pub constant : Temperature,
  /// Slope coefficient of linear function.
  pub coefficient : TemperatureFactor,
  /// Value for increasing temperature for reset.
  pub reset_increase_value : Temperature,
}

impl TemperatureSchedule for LinearTempSchedule
{
  fn calculate_next_temp( &self, prev_temp : Temperature ) -> Temperature
  {
    Temperature::from( prev_temp.unwrap() * self.coefficient.unwrap() + self.constant.unwrap() )
  }

  fn reset_temperature( &self, prev_temp : Temperature ) -> Temperature
  {
    Temperature( prev_temp.unwrap() + self.reset_increase_value.unwrap() )
  }
}