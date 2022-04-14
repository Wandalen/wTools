#![allow(dead_code)]

use std::collections::HashMap;
use std::rc::Rc;
use std::fmt;
use crate::*;
// use super::*;
// use super::common::*;

//

pub struct OnCommand( Option< Rc< dyn Fn() -> () > > );

impl Default for OnCommand
{
  fn default() -> Self
  {
    Self ( Option::None )
  }
}

impl From< &'static dyn Fn() -> () > for OnCommand
{
  fn from( src : &'static dyn Fn() -> () ) -> Self
  {
    OnCommand( Some( Rc::new( src ) ) )
  }
}

// impl Copy for OnCommand
// {
// }

impl Clone for OnCommand
{
  fn clone( &self ) -> Self
  {
    match self
    {
      OnCommand ( Option::None ) => Self ( None ),
      OnCommand ( Option::Some( boxed ) ) => Self ( Some( boxed.clone() ) ),
    }
  }
}

impl fmt::Debug for OnCommand
{
  fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
  {
    match self
    {
      OnCommand ( Option::None ) => f.write_str( "None" ),
      OnCommand ( Option::Some( _rc ) ) => f.write_str( "OnCommand" ),
    }
  }
}

//

#[derive( Default, Debug, Clone )]
pub struct Command
{
  pub hint : String,
  pub long_hint : String,
  pub phrase : String,
  pub subject_hint : String,
  pub properties_hints : HashMap< String, String >,
  pub properties_aliases : HashMap< String, Vec< String > >,
  pub routine : OnCommand,
}

//

#[derive( Debug, Clone, Default )]
pub struct CommandOptions
{
  ins : Command,
}

//

// ro : null,
// h : null,
// lh : null,

impl CommandOptions
{

  field_str!{ hint }
  field_str!{ hint, h }
  field_str!{ long_hint }
  field_str!{ long_hint, lh }
  field_str!{ phrase }
  field_str!{ subject_hint }
  field_str!{ subject_hint, sh }
  field_map_str_str!{ properties_hints, property_hint }
  field_map_str_vec_str!{ properties_aliases, property_alias }
  field_routine!{ routine }
  field_routine!{ routine, ro }

  // pub fn hint< Str : AsRef< str > >( &mut self, src : Str ) -> &mut Self
  // where
  //   String : From<Str>
  // {
  //   self.ins.hint = src.into();
  //   self
  // }
  // pub fn long_hint< Str : AsRef< str > >( &mut self, src : Str ) -> &mut Self
  // where
  //   String : From<Str>
  // {
  //   self.ins.long_hint = src.into();
  //   self
  // }
  // pub fn phrase< Str : AsRef< str > >( &mut self, src : Str ) -> &mut Self
  // where
  //   String : From<Str>
  // {
  //   self.ins.phrase = src.into();
  //   self
  // }
  // pub fn subject_hint< Str : AsRef< str > >( &mut self, src : Str ) -> &mut Self
  // where
  //   String : From<Str>
  // {
  //   self.ins.subject_hint = src.into();
  //   self
  // }
  // pub fn property_hint< Str : AsRef< str > >( &mut self, property : Str, hint : Str ) -> &mut Self
  // where
  //   String : From<Str>
  // {
  //   self.ins.properties_hints.insert( property.into(), hint.into() );
  //   self
  // }
  // pub fn property_alias< Str : AsRef< str > >( &mut self, property : Str, alias : Str ) -> &mut Self
  // where
  //   String : From<Str>
  // {
  //   let entry = self.ins.properties_aliases.entry( property.into() ).or_insert_with( || -> Vec< String > { vec![] } );
  //   entry.push( alias.into() );
  //   self
  // }
  //
  // pub fn routine( &mut self, routine : &'static dyn Fn() ) -> &mut Self
  // {
  //   self.ins.routine = routine.into();
  //   self
  // }

  pub fn form( &self ) -> Command
  {
    self.ins.clone()
  }

}

// impl Options for CommandOptions
// {
//   type For = Command;
// }
