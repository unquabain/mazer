
/// The direction of an edge. Initially, all edges are undefined until the Automaton
/// walks over the graph, either assigning directions or closing the edges.
#[derive(Clone, Copy)]
pub enum Direction {
    Undefined,
    Directed(usize, usize),
    Closed,
}

/// An Edge represents a connection between two nodes, which are cells on the graph.
/// Closed edges will be represented as lines on the final maze drawing. Initially
/// undirected, the Automaton will assign each edge a direction that points toward
/// the root node, giving every cell a single, unique path to the root and to every
/// other node.
#[derive(Clone, Copy)]
pub struct Edge {
    /// The index to one of the two cells reachable by this edge.
    /// There is no implied order between `a` and `b` until one is
    /// given by the `direction` member.
    pub a: usize,

    /// The index to one of the two cells reachable by this edge.
    /// There is no implied order between `a` and `b` until one is
    /// given by the `direction` member.
    pub b: usize,

    /// The direction of this edge. If this value is `Direction::Directed`,
    /// the values are `Direction::Directed(parent, child)`, where `parent`
    /// is closer to the root, and `child` is further.
    pub direction: Direction,

    /// Whether this edge is on the unique path between the starting node and
    /// ending node (one of which may be the root, but don't have to be).
    pub solution: bool,
}

impl Default for Edge {
    fn default() -> Self {
        Self{
            a: usize::MAX,
            b: usize::MAX,
            direction: Direction::Undefined,
            solution: false,
        }
    }
}

impl Edge {
    /// If the Edge is directed, tells you whether the index you supplied
    /// is the "parent" node.
    pub fn is_head(&self, idx: usize) -> bool {
        if let Direction::Directed(head, _) = self.direction {
            return head == idx;
        }
        false
    }

    /// If you give it one of its indices, it returns the other one.
    pub fn follow_from(&self, idx: usize) -> usize {
        if self.a == idx {
            return self.b;
        }
        self.a
    }
}

