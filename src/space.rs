use std::iter::Iterator;
use crate::{
    node::Node,
    edge::{Edge,EdgeDirection},
    error::*,
};

#[inline]
fn n_unique_indices(rng: &mut impl rand::Rng, len: usize, n: usize) -> impl Iterator<Item=usize> {
    let mut root_idcs = std::collections::HashSet::<usize>::new();
    for _ in 0..n {
        loop {
            let choice = rng.random_range(0..len);
            if !root_idcs.contains(&choice) {
                root_idcs.insert(choice);
                break
            }
        }
    }
    root_idcs.into_iter()
}
#[inline]
fn visit(space: &impl Space, rng: &mut impl rand::Rng, nodes: &mut [Node], edges: &mut [Edge], visiting: &mut Vec<usize>) -> Result<()> {
    let idx = visiting.swap_remove(rng.random_range(0..visiting.len()));
    let node_group = nodes[idx].group;
    node_nodes(space, idx, edges, |edge, nidx| {
        let visit_node = &mut nodes[nidx];
        if visit_node.group.is_some() {
            edge.direction = EdgeDirection::Closed;
            return
        }
        if idx < nidx {
            edge.direction = EdgeDirection::Forward
        } else {
            edge.direction = EdgeDirection::Backward
        }
        visit_node.group = node_group;
        visiting.push(nidx);
    });
    Ok(())
}

#[inline]
fn find_root(space: &impl Space, edges: &mut [Edge], node_id: usize) {
    let mut nid = node_id;
    loop {
        match node_parent(space, nid, edges, |edge| { edge.solution = !edge.solution }) {
            None => return,
            Some(new_nid) => {
                nid = new_nid;
            }
        }
    }
}

#[inline]
fn follow_edge(space: &impl Space, node_id: usize, edge_id: usize) -> Option<usize> {
    space.edge_nodes(edge_id).filter(|nid| *nid != node_id).next()
}
#[inline]
fn node_parent(space: &impl Space, node_id: usize, edges: &mut [Edge], mut on_match: impl FnMut(&mut Edge)) -> Option<usize> {
    match space.node_edges(node_id).filter_map(|eid| {
        if eid < edges.len() {
            match edges[eid].direction {
                EdgeDirection::Forward => {
                    let nid = follow_edge(space, node_id, eid).unwrap();
                    if nid < node_id {
                        Some((eid, nid))
                    } else {
                        None
                    }
                }
                EdgeDirection::Backward => {
                    let nid = follow_edge(space, node_id, eid).unwrap();
                    if nid > node_id {
                        Some((eid, nid))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else { None }
    }).next() {
        None => None,
        Some((eid, node_id)) => {
            on_match(&mut edges[eid]);
            Some(node_id)
        }
    }
}
#[inline]
fn node_nodes(space: &impl Space, node_id: usize, edges: &mut [Edge], mut on_each: impl FnMut(&mut Edge, usize)) { 
    space.node_edges(node_id).for_each(|eid| if edges[eid].direction == EdgeDirection::Unknown{
        on_each(&mut edges[eid], follow_edge(space, node_id, eid).unwrap())
    });
}

pub struct SolutionIterator<'a, 'b, S: Space> {
    space: &'a S,
    edges: &'b [Edge],
    last_node: usize,
    last_edge: Option<usize>,
}

impl<'a, 'b, S: Space> std::iter::Iterator for SolutionIterator<'a, 'b, S> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        match self.space.node_edges(self.last_node)
            .filter(|eid| {
                let edge = &self.edges[*eid];
                edge.solution && match self.last_edge {
                    None => true,
                    Some(leid) => *eid != leid,
                }
            }).next() {
                None => None,
                Some(eid) => {
                    self.last_edge = Some(eid);
                    self.last_node = follow_edge(self.space, self.last_node, eid).unwrap();
                    Some(eid)
                }
        }
    }
}

pub trait Space {
    // Required Methods
    fn num_nodes(&self) -> usize;
    fn num_edges(&self) -> usize;
    fn node_edges(&self, node_id: usize) -> impl Iterator<Item=usize>;
    fn edge_nodes(&self, edge_id: usize) -> impl Iterator<Item=usize>;

    // Provided Methods
    fn layout(&self, roots: usize, rng: &mut impl rand::Rng) -> Result<(Vec<Node>, Vec<Edge>)>
        where Self: Sized
    {
        let num_nodes = self.num_nodes();
        let num_edges = self.num_edges();
        let mut nodes = Vec::<Node>::with_capacity(num_nodes);
        let mut visiting = Vec::<usize>::with_capacity(num_nodes);
        for _ in 0..num_nodes {
            nodes.push(Default::default());
        }
        let mut edges = Vec::<Edge>::with_capacity(num_edges);
        for _ in 0..num_edges {
            edges.push(Default::default())
        }
        for (i, idx) in n_unique_indices(rng, num_nodes, roots).enumerate() {
            nodes[idx].group = Some(i);
            nodes[idx].root = true;
            visiting.push(idx);
        }
        loop {
            if visiting.is_empty() {
                return Ok((nodes, edges))
            }
            visit(self, rng, &mut nodes, &mut edges, &mut visiting)?;
        }
    }

    fn get_endpoints(&self, rng: &mut impl rand::Rng) -> (usize, usize)
        where Self: Sized
    {
        let num_nodes = self.num_nodes();
        let start = rng.random_range(0..num_nodes);
        let mut end = start;
        while end == start {
            end = rng.random_range(0..num_nodes);
        }
        (start, end)
    }

    fn solve(&self, edges: &mut [Edge], start: usize, end: usize) -> impl Iterator<Item=usize>
        where Self: Sized
    {
        find_root(self, edges, start);
        find_root(self, edges, end);
        SolutionIterator{
            space: self,
            edges,
            last_node: start,
            last_edge: None,
        }
    }
}
