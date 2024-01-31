use super::{Graph, GraphError};
use super::Direction;
use tracing::{span, Level};

/// Finite automaton that converts the highly cyclical, undirected graph to a directed-acyclical graph
/// by assigning directions to and closing off edges.
pub struct Automaton<'a> {
    graph: &'a mut Graph,
    visiting: Vec<usize>,
}

impl<'a> Automaton<'a> {
    /// Creates a new Automaton. The Graph it works on must outlive it.
    pub fn new(graph: &'a mut Graph) -> Automaton<'a> {
        Automaton{
            graph,
            visiting: Vec::new(),
        }
    }

    /// Walks the graph, adding direction to undirected edges between
    /// unvisited nodes, and closing off edges between visited nodes.
    pub fn init(&mut self, rng: &mut impl rand::Rng) -> Result<(), GraphError>{
        let span = span!(Level::TRACE, "initializing automaton");
        let _enter = span.enter();
        let coord = self.graph.space.rand_coord(rng);
        let idx = self.graph.space.idx(coord);
        self.graph.visit(idx);
        self.visiting.push(idx);
        while self.visit(rng) {};
        Ok(())
    }

    fn find_parent_edge(&mut self, child: usize, idcs: &Vec<usize>) -> Option<usize> {
        for idx in idcs {
            let edge = &mut self.graph.edges[*idx];
            match edge.direction {
                Direction::Undefined | Direction::Closed => continue,
                Direction::Directed(_, _) => {
                    if edge.is_head(child) {
                        continue;
                    }
                    return Some(*idx);
                }
            }
        }
        None
    }

    /// Sets a starting point. This walks the unique path from the specified point to the
    /// root of the graph, toggling the `solution` field as it goes. Because the `solution`
    /// field is toggled, not set, you can call this twice to set the path between one cell
    /// and another, possibly passing through the root, but not definitely.
    /// 
    /// This method should be called _after_ `init()`.
    pub fn set_start(&mut self, coord: (usize, usize)) -> Result<(), GraphError> {
        let mut idx = self.graph.space.idx(coord);
        loop {
            let idcs = self.graph.edges_for(idx);
            let eidx = match self.find_parent_edge(idx, &idcs) {
                None => return Ok(()),
                Some(eidx) => eidx,
            };
            let edge = &mut self.graph.edges[eidx];
            edge.solution = !edge.solution;
            idx = edge.follow_from(idx);
        }
    }

    fn choose_current(&mut self, rng: &mut impl rand::Rng) -> usize {
        let idx = rng.gen::<usize>() % self.visiting.len();
        self.visiting.swap_remove(idx)
    }
    
    fn visit(&mut self, rng: &mut impl rand::Rng) -> bool {
        if self.visiting.len() == 0 {
            return false;
        }
        let current_idx = self.choose_current(rng);
        let graph = &mut self.graph;
        let idcs = graph.edges_for(current_idx);
        for idx in idcs {
            let edges = &graph.edges;
            let dir = edges[idx].direction;
            match dir {
                Direction::Undefined => (),
                _ => continue,
            }
            let visited_idx = edges[idx].follow_from(current_idx);

            if graph.visited(visited_idx) {
                graph.edges[idx].direction = Direction::Closed;
                continue;
            }

            self.visiting.push(visited_idx);
            graph.edges[idx].direction = Direction::Directed(current_idx, visited_idx);
            graph.visit(visited_idx);
        }

        true
    }

}
