

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EdgeDirection {
    #[default]
    Unknown,
    Closed,
    Forward,
    Backward,
    Border,
}

#[derive(Debug,Default)]
pub struct Edge {
    pub direction: EdgeDirection,
    pub solution: bool,
}

