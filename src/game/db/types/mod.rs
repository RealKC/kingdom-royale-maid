//! This module contains low-level data types that can be easily converted from Postgres data types,
//! but are also easily converible to application data types.

mod game;

pub use game::{GameState, RunningGame};
