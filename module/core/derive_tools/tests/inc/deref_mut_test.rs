use super::*;

// use diagnostics_tools::prelude::*;
// use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq ) ]
pub struct IsTransparent( bool );

impl std::ops::Deref for IsTransparent {
    type Target = bool;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for IsTransparent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

include!( "./only_test/deref_mut.rs" );
