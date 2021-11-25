#![allow(dead_code)]

use chrono::prelude::*;

//

pub fn now() -> i64
{
  Utc::now().timestamp_millis()
}

//

pub mod s
{
  use chrono::prelude::*;

  pub fn now() -> i64
  {
    Utc::now().timestamp()
  }

}

//

pub mod ms
{
  use chrono::prelude::*;

  pub fn now() -> i64
  {
    Utc::now().timestamp_millis()
  }

}

//

pub mod ns
{
  use chrono::prelude::*;

  pub fn now() -> i64
  {
    Utc::now().timestamp_nanos()
  }

}
