//! Modeling the maze as a directed, acyclic graph and the methods to construct it.
mod space;
mod edge;
mod graph;
mod automation;

pub use space::Space;
pub use edge::{Direction, Edge};
pub use automation::Automaton;
pub use graph::{Graph, GraphError};
