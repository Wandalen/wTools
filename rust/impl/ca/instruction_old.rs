// #![allow(dead_code)]

// use crate::*;
// use super::*;
// use super::common::*;

// use std::collections::hash_map::HashMap;
// use std::rc::Rc;
// use std::fmt;

// #[derive( Default, Debug, Clone )]
// pub struct InstructionsParse
// {
//   pub instructions : vec< String >,
//   pub delimeters : vec< String >, /* qqq : implement and cover several delimeters case. write test routine for case of implicit + explicit delimeters */
//   pub properties_map_parsing : bool,
//   pub properties_maps : HashMap< String, String >,
//   pub severalValues : bool, /* qqq : implement */
// }

// impl InstructionsParseOptions
// {

//   // field_str!{ hint }
//   // field_str!{ hint, h }
//   // field_str!{ long_hint }
//   // field_str!{ long_hint, lh }
//   // field_str!{ phrase }
//   // field_str!{ subject_hint }
//   // field_str!{ subject_hint, sh }
//   // field_map_str_str!{ properties_hints, property_hint }
//   // field_map_str_vec_str!{ properties_aliases, property_alias }
//   // field_routine!{ routine }
//   // field_routine!{ routine, ro }

//   // pub fn form( &self ) -> Command
//   // {
//   //   self.ins.clone()
//   // }

// }

// impl Options for InstructionsParseOptions
// {
// }
