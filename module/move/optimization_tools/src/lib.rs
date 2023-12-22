//! Optimization tools for lonear and non-linear problem solving.
//! 

use deterministic_rand::{ Hrng, Rng };
pub use deterministic_rand::Seed;

pub mod sudoku;
pub mod optimization;
pub mod simplex;
#[ cfg( feature="static_plot" ) ]
pub mod plot;
#[ cfg( feature="dynamic_plot" ) ]
pub mod plot_dynamic;
