//! Optimization tools for lonear and non-linear problem solving.
//! 

use deterministic_rand::{ Hrng, Rng };
pub use deterministic_rand::Seed;

pub mod problems;
pub mod hybrid_optimizer;
pub mod simplex;
pub mod optimal_params_search;
#[ cfg( feature="static_plot" ) ]
pub mod plot;
#[ cfg( feature="dynamic_plot" ) ]
pub mod plot_dynamic;
