
use deterministic_rand::{ Hrng, Rng };

pub mod sudoku;
pub mod optimization;
#[ cfg( feature="static_plot" ) ]
pub mod plotting;
#[ cfg( feature="dynamic_plot" ) ]
pub mod dynamic_plotting;
