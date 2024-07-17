use rand::thread_rng;

pub const fn pair_to_index(row: usize, column: usize, width: usize) -> usize {
    row * width + column
}
pub const fn index_to_pair(id: usize, width: usize) -> (usize, usize) {
    (id / width, id % width)
}

pub struct Game {
    pub board: Board
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub hidden: bool,
    pub empty: bool,
}

#[derive(Clone)]
pub struct Board {
    pub board_vec: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    fn gen_empty(width: usize, height: usize) -> Self {
        let board_vec = (0..width * height)
            .map(|_| Tile {
                hidden: true,
                empty: true,
            })
            .collect();
        Board {
            board_vec,
            width,
            height,
        }
    }
    fn fill_boxes_randomly(&mut self, filled_count: usize) {
        let mut ids: Vec<usize> = (0..self.width * self.height).collect();
        rand::seq::SliceRandom::shuffle(ids.as_mut_slice(), &mut thread_rng());
        ids.iter()
            .take(filled_count)
            .for_each(|&id| self.board_vec[id].empty = false);
    }
    pub fn new(width: usize, height: usize, filled_count: usize) -> Self {
        let mut board = Self::gen_empty(width, height);
        board.fill_boxes_randomly(filled_count);
        board
    }
}
