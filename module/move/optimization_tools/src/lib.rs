//! Optimization tools for lonear and non-linear problem solving.
//! 

use deterministic_rand::{ Hrng, Rng };
pub use deterministic_rand::Seed;

pub mod nelder_mead;
pub mod sudoku;
pub mod optimization;
pub mod simplex;
pub mod sudoku_opt_params;
#[ cfg( feature="static_plot" ) ]
pub mod plot;
#[ cfg( feature="dynamic_plot" ) ]
pub mod plot_dynamic;
