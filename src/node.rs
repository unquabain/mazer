pub struct Node {
    pub group: Option<usize>,
    pub root: bool,
}

impl std::default::Default for Node {
    fn default() -> Self {
        Self{
            group: None,
            root: false,
        }
    }
}

impl Node {
    pub fn new() -> Self {
        Default::default()
    }
}

