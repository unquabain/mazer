use super::Space;
use super::{Edge, Direction};
use std::fmt;

/// An enum of the different kinds of errors that a Graph can return.
#[derive(Debug)]
pub enum GraphError {
    ErrOutOfBounds,
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GraphError::ErrOutOfBounds => write!(f, "Index out of bounds"),
        }
    }
}


/// Represents the field of the maze as a graph of edges and nodes. The nodes represent
/// the cells of the maze.
///
/// In this implementation, it is a 2D, rectangular maze, with each
/// cell/node representing a square. There is nothing in this algorithm that prevents it
/// from being other shapes, 3D for example. You would just need to edit the `new()`
/// method to create the required number of cells and connect the edges appropriately.
pub struct Graph {
    /// Information about the geometry of the space in which the Graph exists.
    pub space: Space,

    /// Each cell, or node, in the graph. Currently, they have only one property,
    /// which is whether or not they have been visited by the Automaton.
    pub cells: Vec<bool>,

    /// The edges have rather more properties.
    pub edges: Vec<Edge>,
}


impl Graph {
    /// Creates a new 2D, rectangular graph, and wires up all the edges.
    pub fn new(width: usize, height: usize) -> Self {
        let empty_edge: Edge = Default::default();
        let space = Space{height, width};
        let cells = vec![false; space.num_cells()];
        let edges = vec![empty_edge; space.num_edges()];
        let mut me = Self {
            space,
            cells,
            edges,
        };
        for i in 0..me.space.num_cells() {
            let coord = me.space.coords(i);
            let (x, y) = coord;
            let [up, left, right, down] = me.space.edges(coord);
            me.edges[up].b = i;
            if y == 0 {
                me.edges[up].direction = Direction::Closed;
            }
            me.edges[left].b = i;
            if x == 0 {
                me.edges[left].direction = Direction::Closed;
            }
            me.edges[right].a = i;
            if x == width-1 {
                me.edges[right].direction = Direction::Closed;
            }
            me.edges[down].a = i;
            if y == height-1 {
                me.edges[down].direction = Direction::Closed;
            }
        }
        me
    }

    /// Marks one of the cells visited. It's convenient to make this a method of Graph
    /// for Rusty memory reasons.
    pub fn visit(&mut self, idx: usize) {
        if idx >= self.cells.len() {
            return;
        }

        self.cells[idx] = true;
    }

    /// Whether the cell has been visited already by the Automaton.
    pub fn visited(&self, idx: usize) -> bool {
        if idx >= self.cells.len() {
            return false;
        }
        self.cells[idx]
    }

    /// Given a cell index, it returns all the adjacent edges according to the
    /// Space. For a 2D, rectangular grid, this will be `(north, west, east, south)`,
    /// but the number and order shouldn't matter.
    pub fn edges_for(&self, idx: usize) -> Vec<usize> {
        self.space.edges(self.space.coords(idx)).to_vec()
    }
}

