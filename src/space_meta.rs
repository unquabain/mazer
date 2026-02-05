use crate::{
    edge::*,
    node::*,
    space::*,
};

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct BorderKey (usize, usize);

impl BorderKey {
    fn new(a: usize, b: usize) -> Self {
        if a < b {
            Self(a, b)
        } else {
            Self(b, a)
        }
    }
    fn into_iter(&self) -> impl Iterator<Item=usize> {
        std::iter::from_fn({
            let mut count = 0;
            move || {
                match count {
                    0 => {
                        count = 1;
                        Some(self.0)
                    }
                    1 => {
                        count = 2;
                        Some(self.1)
                    }
                    _ => None,
                }
            }
        })
    }
}

struct Border {
    key: BorderKey,
    edges: Vec<usize>,
    gateway: Option<usize>,
}

impl Border {
    fn new(key: BorderKey, edge_id: usize) -> Self {
        Self{
            key,
            edges: vec![edge_id],
            gateway: None,
        }
    }
    fn matches_group(&self, node_group: usize) -> bool {
        self.key.0 == node_group || self.key.1 == node_group
    }
    fn matches_key(&self, key: &BorderKey) -> bool {
        self.key == *key
    }
    fn push(&mut self, edge_id: usize) {
        self.edges.push(edge_id)
    }
    pub fn choose_gateway(&mut self, rng: &mut impl rand::Rng) -> usize {
        match self.gateway {
            Some(gw) => gw,
            None => {
                let gw = self.edges[rng.random_range(0..self.edges.len())];
                self.gateway = Some(gw);
                gw
            }
        }
    }
}

pub struct SpaceMeta {
    node_groups: Vec<usize>,
    borders: Vec<Border>,
}

impl SpaceMeta {
    pub fn new(space: &impl Space, edges: &[Edge], nodes: &[Node]) -> Self {
        let mut this = Self{
            node_groups: Default::default(),
            borders: Default::default(),
        };
        for eid in 0..edges.len() {
            let mut node_groups = space.edge_nodes(eid)
                .map(|nid| nodes[nid].group.unwrap())
                .take(2);
            let (a, b) = (node_groups.next().unwrap(), node_groups.next().unwrap());
            if a == b {
                continue
            }
            let bk = BorderKey::new(a, b);
            match this.borders.iter_mut().find(|b| b.matches_key(&bk)) {
                Some(border) => border.push(eid),
                None => this.borders.push(Border::new(bk, eid)),
            }
            if !this.node_groups.contains(&bk.0) {
                this.node_groups.push(bk.0);
            }
            if !this.node_groups.contains(&bk.1) {
                this.node_groups.push(bk.1);
            }
        }

        this
    }
    pub fn open_gateways(&mut self, meta_edges: &[Edge], edges: &mut [Edge], rng: &mut impl rand::Rng) {
        for (meid, _me) in meta_edges.iter().enumerate().filter(|(_, e)| e.direction != EdgeDirection::Closed) {
            let border = &mut self.borders[meid];
            let gw = border.choose_gateway(rng);
            edges[gw].direction = EdgeDirection::Border;
        }
    }
    pub fn gateway(&self, border_id: usize) -> Option<usize> {
        if border_id < self.borders.len() {
            self.borders[border_id].gateway
        } else {
            None
        }
    }
    pub fn zone_index(&self, group_id: usize) -> Option<usize> {
        self.node_groups.iter().position(|z| *z == group_id)
    }
}

impl Space for SpaceMeta {
    fn num_nodes(&self) -> usize {
        self.node_groups.len()
    }
    fn num_edges(&self) -> usize {
        self.borders.len()
    }
    fn node_edges(&self, node_id: usize) -> impl Iterator<Item=usize> {
        let group_id = if node_id < self.node_groups.len() {
            self.node_groups[node_id]
        } else {
            usize::MAX
        };
        self.borders.iter().enumerate().filter_map(move |(bid, border)| {
            if border.matches_group(group_id) {
                Some(bid)
            } else {
                None
            }
        })
    }

    fn edge_nodes(&self, edge_id: usize) -> impl Iterator<Item=usize> {
        self.borders[edge_id].key.into_iter().map(|gid| {
            self.node_groups.iter().position(|z| *z == gid).unwrap()
        })
    }
}
