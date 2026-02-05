use std::iter::Iterator;
use super::space::Space;
use super::render::SpaceRenderer;

macro_rules! rq {
    ( ( $lhs:expr ) / ( $rhs:expr )) => {
        (
            ($lhs as usize / $rhs as usize),
            ($lhs as usize % $rhs as usize)
        )
    }
}

#[derive(Debug)]
pub struct SpaceSquare {
    rows: usize,
    cols: usize,
}

#[derive(Debug)]
pub struct SpaceSquareEdgeIterator<'a> {
    space: &'a SpaceSquare,
    row: usize,
    col: usize,
    step: u8,
}

impl<'a> SpaceSquareEdgeIterator<'a> {
    fn new(space: &'a SpaceSquare, row: usize, col: usize) -> Self {
        Self{
            space,
            row,
            col,
            step: 0,
        }
    }
    fn east(&self, col: usize) -> usize {
        (self.row * self.space.edge_row_len()) + col
    }
    fn south(&self, row: usize) -> usize {
        (row * self.space.edge_row_len()) + self.space.cols - 1 + self.col
    }
}

impl<'a> Iterator for SpaceSquareEdgeIterator<'a> {
    type Item=usize;
    fn next(&mut self) -> Option<usize> {
        loop {
            match self.step {
                0 => {
                    // North
                    self.step = 1;
                    // If this is the first row, there are no North edges
                    if self.row == 0 {
                        continue
                    }


                    // The North edge is the South edge of the previous row.
                    return Some(self.south(self.row-1))
                },
                1 => {
                    // West
                    self.step = 2;
                    // If this is the western-most node, there is no western edge.
                    if self.col == 0 {
                        continue
                    }
                    // The west edge is the east edge of the previous node
                    return Some(
                        self.east(self.col-1)
                    )

                },
                2 => {
                    self.step = 3;
                    // East
                    if self.col >= self.space.cols - 1 {
                        continue
                    }
                    return Some(
                        self.east(self.col)
                    )
                },
                3 => {
                    self.step = 4;
                    // South
                    if self.row >= self.space.rows - 1 {
                        continue
                    }
                    return Some(
                        self.south(self.row)
                    )
                },
                _ => return None,
            }
        }
    }
}


pub struct SpaceSquareNodeIterator<'a> {
    space: &'a SpaceSquare,
    row: usize,
    col: usize,
    step: u8,
}

impl<'a> SpaceSquareNodeIterator<'a> {
    fn new(space: &'a SpaceSquare, row: usize, col: usize) -> Self {
        Self{
            space,
            row,
            col,
            step: 0,
        }
    }
    #[inline]
    fn is_horizontal(&self) -> bool {
        self.space.is_edge_horizontal(self.col)
    }
    fn west(&self, col: usize) -> usize {
        self.row * self.space.cols + col
    }

    fn north(&self, row: usize) -> usize {
        row * self.space.cols + self.col - (self.space.cols - 1)
    }
}
impl<'a> Iterator for SpaceSquareNodeIterator<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        loop {
            match self.step {
                0 => {
                    self.step = 1;
                    if self.is_horizontal() {
                        continue
                    }
                    return Some(
                        self.west(self.col)
                    )
                },
                1 => {
                    self.step = 2;
                    if self.is_horizontal() {
                        continue
                    }
                    if self.col >= self.space.cols - 1 {
                        continue
                    }
                    return Some(
                        self.west(self.col + 1)
                    )

                },
                2 => {
                    self.step = 3;
                    if !self.is_horizontal() {
                        continue
                    }
                    return Some(
                        self.north(self.row)
                    )
                },
                3 => {
                    self.step = 4;
                    if !self.is_horizontal() {
                        continue
                    }
                    if self.row >= self.space.rows {
                        continue
                    }
                    return Some(
                        self.north(self.row + 1)
                    )
                },
                _ => return None,
            }
        }
    }
}

impl SpaceSquare {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self{
            rows,
            cols,
        }
    }
    #[inline]
    fn edge_row_len(&self) -> usize {
        self.cols * 2 - 1
    }

    #[inline]
    fn is_edge_horizontal(&self, col: usize) -> bool {
        col >= self.cols - 1
    }
}

impl Space for SpaceSquare {
    fn num_nodes(&self) -> usize {
        self.rows * self.cols
    }
    fn num_edges(&self) -> usize {
        self.rows * self.edge_row_len() - self.cols
    }
    fn node_edges(&self, node_id: usize) -> impl Iterator<Item=usize> {
        let (row, col) = rq!((node_id) / (self.cols));
        SpaceSquareEdgeIterator::new(self, row, col)
    }
    fn edge_nodes(&self, edge_id: usize) -> impl Iterator<Item=usize> {
        let (row, col) = rq!((edge_id) / (self.edge_row_len()));
        SpaceSquareNodeIterator::new(self, row, col)
    }
}

impl SpaceRenderer<2> for SpaceSquare {
    fn edge_position(&self, edge_id: usize) -> ([f32; 2], [f32; 2]) {
        let mut end_nodes = self.edge_nodes(edge_id);
        let start = end_nodes.next().unwrap();
        let end = end_nodes.next().unwrap();
        let (start_row, start_col) = rq!((start) / (self.cols));
        let (end_row, end_col) = rq!((end) / (self.cols));
        ([ start_row as f32, start_col as f32 ], [ end_row as f32, end_col as f32 ])
    }
    fn node_position(&self, node_id: usize) -> [f32; 2] {
        let (row, col) = rq!((node_id) / (self.cols));
        [ row as f32, col as f32 ]
    }
}


#[cfg(test)]
mod tests {
    use crate::space::Space;
    use super::*;


    fn init_tests() {
        log4rs_test_utils::test_logging::init_logging_once_for(
            None,
            log::LevelFilter::Debug,
            None,
        );
    }

    #[test]
    fn test_num_edges() {
        init_tests();
        let space = SpaceSquare::new(3, 3);
        assert_eq!(space.num_edges(), 12);
    }

    #[test]
    fn test_num_nodes() {
        init_tests();
        let space = SpaceSquare::new(3, 3);
        assert_eq!(space.num_nodes(), 9);
    }

    #[test]
    fn test_node_edges() {
        init_tests();
        let space = SpaceSquare::new(3, 3);
        macro_rules! test {
            ($idx:literal => $edges:expr) => {
                {
                    let mut edges = space.node_edges($idx).collect::<Vec<_>>();
                    edges.sort();
                    assert_eq!(edges, $edges);
                }
            }
        }
        test!(0 => vec![0, 2]);
        test!(1 => vec![0, 1, 3]);
        test!(2 => vec![1, 4]);

        test!(3 => vec![2, 5, 7]);
        test!(4 => vec![3, 5, 6, 8]);
        test!(5 => vec![4, 6, 9]);

        test!(6 => vec![7, 10]);
        test!(7 => vec![8, 10, 11]);
        test!(8 => vec![9, 11]);
    }

    #[test]
    fn test_edge_nodes() {
        init_tests();
        let space = SpaceSquare::new(3, 4);
        macro_rules! test {
            ($idx:literal => $nodes:expr) => {
                {
                    let mut nodes = space.edge_nodes($idx).collect::<Vec<_>>();
                    nodes.sort();
                    assert_eq!(nodes, $nodes);
                }
            }
        }

        test!(0 => vec![0, 1]);
        test!(1 => vec![1, 2]);
        test!(2 => vec![2, 3]);

        test!(3 => vec![0, 4]);
        test!(4 => vec![1, 5]);
        test!(5 => vec![2, 6]);
        test!(6 => vec![3, 7]);

        test!(7 => vec![4, 5]);
        test!(8 => vec![5, 6]);
        test!(9 => vec![6, 7]);

        test!(10 => vec![4, 8]);
        test!(11 => vec![5, 9]);
        test!(12 => vec![6, 10]);
        test!(13 => vec![7, 11]);

        test!(14 => vec![8, 9]);
        test!(15 => vec![9, 10]);
        test!(16 => vec![10, 11]);
    }

    #[test]
    fn test_edge_position() {
        init_tests();
        let space = SpaceSquare::new(3, 4);
        for eid in 0..space.num_edges() {
            let (start, end) = space.edge_position(eid);
            assert!((start[0] == end[0]) || (start[1] == end[1]), "edge {eid} is diagonal: {start:?} {end:?}");
        }
    }
}
