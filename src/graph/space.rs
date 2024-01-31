/// Space represents a geometry. In this case, a 2D, rectangular grid.
/// TODO: Make this a Trait, and implement other geometries.
pub struct Space {
    pub width: usize,
    pub height: usize,
}

impl Space {
    pub fn num_cells(&self) -> usize {
        let Space{width, height} = self;
        width * height
    }
    pub fn num_edges(&self) -> usize {
        let Space{width, height} = self;
        (width * (height + 1)) + ((width + 1) * height)
    }
    /// Turns cell a coordinate into an index.
    pub fn idx(&self, coord: (usize, usize)) -> usize {
        let Space{width, ..} = self;
        let (x, y) = coord;
        (y * width) + x
    }
    /// Turns a cell index into a coordinate.
    pub fn coords(&self, idx: usize) -> (usize, usize) {
        let Space{width, ..} = self;
        (idx % width, idx / width)
    }
    fn span(&self) -> usize {
        let Space{width, ..} = self;
        width * 2 + 1
    }
    
    fn north(&self, coords: (usize, usize)) -> usize {
        let (x, y) = coords;
        self.span()*y + x
    }
    fn west(&self, coords: (usize, usize)) -> usize {
        let (x, y) = coords;
        let Space{width, ..} = self;
        self.north((x, y)) + width
    }
    fn south(&self, coords: (usize, usize)) -> usize {
        let (x, y) = coords;
        self.north((x, y+1))
    }
    fn east(&self, coords: (usize, usize)) -> usize {
        let (x, y) = coords;
        self.west((x+1, y))
    }
    /// Returns all the indices of the edges that lead to or away from the cell at the given
    /// coordinate.
    /// TODO: Other geometries may return different numbers of indices.
    pub fn edges(&self, coords: (usize, usize)) -> [usize;4] {
        [self.north(coords), self.west(coords), self.east(coords), self.south(coords)]
    }
    /// Returns a random coordinate in the defined Space.
    pub fn rand_coord(&self, rng: &mut impl rand::Rng) -> (usize, usize) {
        (rng.gen::<usize>() % self.width, rng.gen::<usize>() % self.height)
    }
}


