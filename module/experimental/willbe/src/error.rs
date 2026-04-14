//! Error handling module for willbe.

mod private {}

crate ::mod_interface!
{
  // Be specific about what we import to avoid namespace conflicts
  exposed use ::error_tools :: { typed, untyped, Error, ErrWith, ResultWithReport };
  exposed use ::error_tools ::dependency :: *;
  
  // Re-export standard library Result and Option
  exposed use ::core ::result ::Result;
  exposed use ::core ::option ::Option;
}