
#![ allow( non_camel_case_types ) ]

use std::cmp::PartialEq;

//

pub struct append_vectors_once<T>
{
  dst : Vec<T>,
  src : Vec<Vec<T>>,
  start_from : usize,
  on_equalize : Option<fn(&T, &T) -> bool>,
  on_evaluate1 : Option<fn(&T) -> T>,
  on_evaluate2 : Option<fn(&T) -> T>,
  formed : u8,
}

//

impl<T> Default for append_vectors_once<T>
{
  fn default() -> Self
  {
    let opts = Self
    {
      src : vec![],
      dst : vec![],
      start_from : 0,
      on_evaluate1 : None,
      on_evaluate2 : None,
      on_equalize : None,
      formed : 0,
    };
    return opts;
  }
}

impl<T : PartialEq + Copy +  Clone + std::default::Default> append_vectors_once<T>
{
  pub fn src( &mut self, src : Vec<Vec<T>> ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.src = src;
    self
  }

  //

  pub fn dst( &mut self, dst : Vec<T> ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.dst= dst;
    self
  }

  //

  pub fn start_from( &mut self, start_from : usize ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.start_from = start_from;
    self
  }

  //

  pub fn on_equalize( &mut self, on_equalize : fn( &T, &T ) -> bool ) -> &mut Self
  {
    assert!( self.on_evaluate1.is_none() && self.on_evaluate2.is_none(), "Expects no evaluation callbacks." );
    assert!( self.formed == 0, "Context is already formed" );
    self.on_equalize = Some( on_equalize );
    self
  }

  //

  pub fn on_evaluate1( &mut self, on_evaluate1 : fn( &T ) -> T ) -> &mut Self
  {
    assert!( self.on_equalize.is_none(), "Expects no equalizer callback." );
    assert!( self.formed == 0, "Context is already formed" );
    self.on_evaluate1 = Some( on_evaluate1 );
    self
  }

  //

  pub fn on_evaluate2( &mut self, on_evaluate2 : fn( &T ) -> T ) -> &mut Self
  {
    assert!( self.on_equalize.is_none(), "Expects no equalizer callback." );
    assert!( self.formed == 0, "Context is already formed" );
    self.on_evaluate2 = Some( on_evaluate2 );
    self
  }

  //

  pub fn form( self ) -> Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    Self { formed : 1, .. self }
  }

  //

  fn to_left_index( &self ) -> left_index<T>
  {
    let mut opts : left_index<T> = left_index::default();
    opts.src( self.dst.clone() );
    if self.on_evaluate1.is_some()
    {
      opts.on_evaluate1( self.on_evaluate1.unwrap() );
    }
    if self.on_evaluate2.is_some()
    {
      opts.on_evaluate2( self.on_evaluate2.unwrap() );
    }
    opts.start_from( self.start_from );
    opts
  }

  //

  pub fn call( &mut self ) -> Vec<T>
  {
    let mut index_finder = self.to_left_index();

    for vec in &self.src
    {
      for el in vec
      {
        index_finder.ins( *el );
        if index_finder.call().is_none()
        {
          self.dst.push( *el );
        }
      }
    }

    self.dst.clone()
  }
}

//

pub struct left_index<T>
{
  src : Vec<T>,
  ins : T,
  start_from : usize,
  on_evaluate1 : Option<fn(&T) -> T>,
  on_evaluate2 : Option<fn(&T) -> T>,
  on_equalize : Option<fn(&T, &T) -> bool>,
  formed : u8,
}

//

impl<T : std::default::Default> Default for left_index<T>
{
  fn default() -> Self
  {
    let opts = Self
    {
      src : vec![],
      ins : Default::default(),
      start_from : 0,
      on_equalize : None,
      on_evaluate1 : None,
      on_evaluate2 : None,
      formed : 0,
    };
    return opts;
  }
}

impl<T : PartialEq> left_index<T>
{
  pub fn src( &mut self, src : Vec<T> ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.src = src;
    self
  }

  //

  pub fn ins( &mut self, ins : T ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.ins = ins;
    self
  }

  //

  pub fn start_from( &mut self, start_from : usize ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.start_from = start_from;
    self
  }

  //

  pub fn on_equalize( &mut self, on_equalize : fn( &T, &T ) -> bool ) -> &mut Self
  {
    assert!( self.on_evaluate1.is_none() && self.on_evaluate2.is_none(), "Expects no evaluation callbacks." );
    assert!( self.formed == 0, "Context is already formed" );
    self.on_equalize = Some( on_equalize );
    self
  }

  //

  pub fn on_evaluate1( &mut self, on_evaluate1 : fn( &T ) -> T ) -> &mut Self
  {
    assert!( self.on_equalize.is_none(), "Expects no equalizer callback." );
    assert!( self.formed == 0, "Context is already formed" );
    self.on_evaluate1 = Some( on_evaluate1 );
    self
  }

  //

  pub fn on_evaluate2( &mut self, on_evaluate2 : fn( &T ) -> T ) -> &mut Self
  {
    assert!( self.on_equalize.is_none(), "Expects no equalizer callback." );
    assert!( self.formed == 0, "Context is already formed" );
    self.on_evaluate2 = Some( on_evaluate2 );
    self
  }

  //

  pub fn form( self ) -> Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    Self { formed : 1, .. self }
  }

  //

  pub fn call( &self ) -> Option<usize>
  {
    if self.on_equalize.is_some()
    {
      let equalizer = self.on_equalize.unwrap();
      for i in 0..self.src.len()
      {
        if equalizer( &self.src[ i ], &self.ins )
        {
          return Some( i );
        }
      }
    }
    else if self.on_evaluate1.is_some()
    {
      let ins;
      let evaluate1 = self.on_evaluate1.unwrap();

      if self.on_evaluate2.is_some()
      {
        let evaluate2 = self.on_evaluate2.unwrap();
        ins = evaluate2( &self.ins );
      }
      else
      {
        ins = evaluate1( &self.ins );
      }

      for i in self.start_from..self.src.len()
      {
        if evaluate1( &self.src[ i ] ) == ins
        {
          return Some( i );
        }
      }
    }
    else
    {
      assert!( self.on_evaluate2.is_none(), "{}", "Expects callback {-on_evaluate1-}." );
      for i in self.start_from..self.src.len()
      {
        if self.src[ i ] == self.ins
        {
          return Some( i );
        }
      }
    }

    None
  }
}
