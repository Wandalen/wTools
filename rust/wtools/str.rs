#![ allow( dead_code ) ]
#![ allow( unused_variables ) ]
#![ allow( non_camel_case_types ) ]

use regex::Regex;

pub fn regexp_is( src : &str ) -> bool
{
  let regexp = Regex::new(r"^\\/.*\\/[dgimsuUx]").unwrap();
  regexp.is_match( src )
}

//

pub struct split<'a>
{
  pub src : String,
  pub delimeter : Vec<&'a str>,
  pub preserving_empty : bool,
  pub preserving_delimeters : bool,
  pub preserving_quoting : bool,
  pub inlining_quoting : bool,
  pub stripping : bool,
  pub quoting : bool,
  pub quoting_prefixes : Vec<&'a str>,
  pub quoting_postfixes : Vec<&'a str>,
  pub on_delimeter : Option<fn( &'a str, usize, &'a str ) -> &'a str>,
  pub on_quote : Option<fn( &'a str, usize, &'a str ) -> &'a str>,
  formed : u8,
}

impl<'a> Default for split<'a>
{
  fn default() -> Self
  {
    let opts = Self
    {
      src : String::from( "" ),
      delimeter : vec![ " " ],
      preserving_empty : true,
      preserving_delimeters : true,
      preserving_quoting : true,
      inlining_quoting : true,
      stripping : true,
      quoting : true,
      quoting_prefixes : vec![ "\"" ],
      quoting_postfixes : vec![ "\"" ],
      on_delimeter : None,
      on_quote : None,
      formed : 0,
    };
    return opts;
  }
}

impl<'a> split<'a> /* Dmytro : dubious, need to use traits */
{
  pub fn src( &mut self, src : String ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.src = src;
    self
  }

  //

  pub fn delimeter( &mut self, delimeter : Vec<&'a str> ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.delimeter = delimeter;
    self
  }

  //

  pub fn preserving_empty( &mut self, preserving_empty : bool ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.preserving_empty = preserving_empty;
    self
  }

  //

  pub fn preserving_delimeters( &mut self, preserving_delimeters : bool ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.preserving_delimeters = preserving_delimeters;
    self
  }

  //

  pub fn preserving_quoting( &mut self, preserving_quoting : bool ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.preserving_quoting = preserving_quoting;
    self
  }

  //

  pub fn inlining_quoting( &mut self, inlining_quoting : bool ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.inlining_quoting = inlining_quoting;
    self
  }

  //

  pub fn stripping( &mut self, stripping : bool ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.stripping = stripping;
    self
  }

  //

  pub fn quoting( &mut self, quoting : bool ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.quoting = quoting;
    self
  }

  //

  pub fn quoting_prefixes( &mut self, quoting_prefixes : Vec<&'a str> ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.quoting_prefixes = quoting_prefixes;
    self
  }

  //

  pub fn quoting_postfixes( &mut self, quoting_postfixes : Vec<&'a str> ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.quoting_postfixes = quoting_postfixes;
    self
  }

  //

  pub fn on_delimeter( &mut self, on_delimeter : fn( &'a str, usize, &'a str ) -> &'a str ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.on_delimeter = Some( on_delimeter );
    self
  }

  //

  pub fn on_quote( &mut self, on_quote : fn( &'a str, usize, &'a str ) -> &'a str ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.on_quote = Some( on_quote );
    self
  }

  //

  pub fn form( self ) -> Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    Self { formed : 1, .. self }
  }
}

//

pub fn split( o : &split ) -> Vec<String>
{

  if o.stripping && !o.quoting && o.on_delimeter.is_none()
  {
    let opts = split_fast::from( &o );
    return split_fast( &opts );
  }

  let mut result : Vec<String> = vec![];
  result
}

//

#[derive(Debug, Clone)]
pub struct split_fast<'a>
{
  pub src : String,
  pub delimeter : Vec<&'a str>,
  pub preserving_empty : bool,
  pub preserving_delimeters : bool,
  formed : u8,
}

impl<'a> Default for split_fast<'a>
{
  fn default() -> Self
  {
    let opts = Self
    {
      src : String::from( "" ),
      delimeter : vec![ " " ],
      preserving_empty : true,
      preserving_delimeters : true,
      formed : 0,
    };
    opts
  }
}

impl<'a> split_fast<'a>
{
  pub fn from( o : &'a split ) -> Self
  {
    let opts = Self
    {
      src : o.src.clone(),
      delimeter : o.delimeter.clone(),
      preserving_empty : o.preserving_empty,
      preserving_delimeters : o.preserving_delimeters,
      formed : o.formed,
    };
    opts
  }

  pub fn src( &mut self, src : String ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.src = src;
    self
  }

  //

  pub fn delimeter( &mut self, delimeter : Vec<&'a str> ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.delimeter = delimeter;
    self
  }

  //

  pub fn preserving_empty( &mut self, preserving_empty : bool ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.preserving_empty = preserving_empty;
    self
  }

  //

  pub fn preserving_delimeters( &mut self, preserving_delimeters : bool ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.preserving_delimeters = preserving_delimeters;
    self
  }

  //

  pub fn form( self ) -> Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    Self { formed : 1, .. self }
  }
}

//

pub fn split_fast( o : &split_fast ) -> Vec<String>
{
  let mut result: Vec<String> = vec![];

  let delimeters = &o.delimeter;

  let preserving_empty = o.preserving_empty;
  let preserving_delimeters = o.preserving_delimeters;

  let mut found_delimeters = delimeters.clone();

  if !preserving_delimeters && delimeters.len() == 1
  {
    let splits: Vec<&str> = o.src.split( delimeters[ 0 ] ).collect();
    result = splits.iter().map( | x | x.to_string() ).collect::<Vec<String>>();
    if !preserving_empty
    {
      result = result.iter().filter( | x | x.len() != 0 ).map( | x | x.to_string() ).collect::<Vec<String>>();
    }
  }
  else
  {
    let src_len = o.src.len();
    let delimeter_len = delimeters.len();
    if delimeter_len == 0
    {
      result.push( o.src.to_string() );
      return result;
    }

    let mut closests : Vec<usize> = vec![];
    let mut position : usize = 0;
    let mut closest_position : usize = 0;
    let mut closest_index : i32 = -1;
    let mut has_empty_delimeter : bool = false;

    for d in 0..delimeter_len
    {
      let delimeter = delimeters[ d ];
      if regexp_is( delimeter )
      {
        unimplemented!( "regexp splitting is not implemented" );
      }
      else
      {
        if delimeter.len() == 0
        {
          has_empty_delimeter = true;
        }
      }
      closests.push( delimeter_next( &o.src, delimeter, &mut found_delimeters, position ) );
    }

    let mut delimeter = "";
    while position < src_len || position == 0
    {
      delimeter = closest_which( &o.src, &found_delimeters, &closests, &mut closest_position, &mut closest_index );

      if closest_position == src_len
      {
        break;
      }

      let del_len = delimeter.len();
      if del_len == 0
      {
        closest_position += 1;
      }

      let substring = o.src.get( position..closest_position ).unwrap();
      if preserving_empty || !substring.is_empty()
      {
        result.push( String::from( substring ) );
      }

      if del_len > 0 || position < src_len
      {
        if preserving_delimeters
        {
          if preserving_empty || del_len > 0
          {
            result.push( String::from( delimeter ) );
          }
        }
      }

      position = closests[ closest_index as usize ] + ( if del_len > 0 { del_len } else { 1 } );

      for d in 0..delimeter_len
      {
        if closests[ d ] < position
        {
          closests[ d ] = delimeter_next( &o.src, delimeters[ d ], &mut found_delimeters, position );
        }
      }
    }

    if delimeter.len() > 0 || !has_empty_delimeter
    {
      let substring = o.src.get( position..src_len ).unwrap();
      if preserving_empty || !substring.is_empty()
      {
        result.push( String::from( substring ) );
      }
    }

    if has_empty_delimeter
    {
      if result.len() > 0 && result[ result.len() - 1 ].is_empty()
      {
        result.pop();
      }
    }
  }

  return result.iter().map( | x | x.to_string() ).collect::<Vec<String>>();

  /* */

  fn delimeter_next( src : &String, delimeter : &str, found_delimeters : &mut Vec<&str>, position : usize ) -> usize
  {
    assert!( position <= src.len() );

    if regexp_is( delimeter )
    {
      unimplemented!( "regexp splitting is not implemented" );
    }
    else
    {
      let indexes: Vec<( usize, &str )> = src.match_indices( delimeter ).collect();
      for ( index, _ ) in indexes
      {
        if index >= position
        {
          return index;
        }
      }
    }

    src.len()
  }

  //

  fn closest_which<'a>( src : &String, found_delimeters : &Vec<&'a str>, closests : &Vec<usize>, closest_position : &mut usize, closest_index : &mut i32  ) -> &'a str
  {
    let src_len = src.len();
    *closest_position = src_len;
    *closest_index = 0;
    for d in 0..found_delimeters.len()
    {
      if closests[ d ] < src_len && closests[ d ] < *closest_position
      {
        *closest_position = closests[ d ];
        *closest_index = d as i32;
      }
    }

    found_delimeters[ *closest_index as usize ]
  }
}
