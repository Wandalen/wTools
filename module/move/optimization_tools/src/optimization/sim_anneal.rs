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

pub trait TemperatureSchedule
{
  fn calculate_next_temp( &self, prev_temp : f64 ) -> f64;
  fn reset_temperature( &self, prev_temp : f64 ) -> f64;
}

pub struct LinearTempSchedule
{
  pub constant : f64,
  pub coefficient : f64,
  pub reset_coefficient : f64,
}

impl TemperatureSchedule for LinearTempSchedule
{
  fn calculate_next_temp( &self, prev_temp : f64 ) -> f64 
  {
    prev_temp * self.coefficient + self.constant
  }

  fn reset_temperature( &self, prev_temp : f64 ) -> f64 
  {
    prev_temp + self.reset_coefficient
  }
}