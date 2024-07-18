use rand::thread_rng;

pub const fn pair_to_index(row: usize, column: usize, width: usize) -> usize {
    row * width + column
}

pub struct Game {
    pub board: Board,
    pub menu: Menu,
    pub winstate: Winstate,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(10, 10, 40),
            menu: Menu {
                width_input: String::from("10"),
                height_input: String::from("10"),
                filled_count_input: String::from("65"),
                start_pressed: false,
            },
            winstate: Winstate::InProgress,
        }
    }
    pub fn wincheck(&mut self) {
        if self.board.board_vec.iter().all(|&tile| {
            (tile.empty == false && tile.hidden == false)
                || (tile.empty == true && tile.hidden == true)
        }) {
            self.winstate = Winstate::Won;
            (0..self.board.width * self.board.height)
                .for_each(|id| self.board.board_vec[id].hidden = false)
        } else if self
            .board
            .board_vec
            .iter()
            .any(|&tile| tile.empty == true && tile.hidden == false)
        {
            self.winstate = Winstate::Lost
        } else {
            self.winstate = Winstate::InProgress
        };
    }
}

pub enum Winstate {
    Won,
    Lost,
    InProgress,
}

pub struct Menu {
    pub width_input: String,
    pub height_input: String,
    pub start_pressed: bool,
    pub filled_count_input: String,
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub hidden: bool,
    pub empty: bool,
    pub marked: bool,
}

#[derive(Clone)]
pub struct Board {
    pub board_vec: Vec<Tile>,
    pub width: usize,
    pub height: usize,
    pub filled_count: usize,
    pub vertical_count: Vec<Vec<usize>>,
    pub horizontal_count: Vec<Vec<usize>>,
}

impl Board {
    fn gen_empty(width: usize, height: usize) -> Self {
        let board_vec = (0..width * height)
            .map(|_| Tile {
                hidden: true,
                empty: true,
                marked: false,
            })
            .collect();
        Board {
            board_vec,
            width,
            height,
            filled_count: 0,
            vertical_count: Vec::new(),
            horizontal_count: Vec::new(),
        }
    }
    fn fill_boxes_randomly(&mut self, filled_count: usize) {
        let mut ids: Vec<usize> = (0..self.width * self.height).collect();
        rand::seq::SliceRandom::shuffle(ids.as_mut_slice(), &mut thread_rng());
        ids.iter()
            .take(filled_count)
            .for_each(|&id| self.board_vec[id].empty = false);
    }
    fn count_vertical(&mut self) {
        self.vertical_count = (0..self.width)
            .map(|column| {
                let mut consecutive = 0;
                (0..self.height).fold(Vec::new(), |mut acc, row| {
                    let tile_is_empty =
                        self.board_vec[pair_to_index(row, column, self.width)].empty;
                    if consecutive > 0 && tile_is_empty {
                        acc.push(consecutive);
                        consecutive = 0;
                        acc
                    } else if !tile_is_empty && row == self.height - 1 {
                        consecutive += 1;
                        acc.push(consecutive);
                        acc
                    } else if tile_is_empty {
                        acc
                    } else {
                        consecutive += 1;
                        acc
                    }
                })
            })
            .collect();
    }
    fn count_horizontal(&mut self) {
        self.horizontal_count = (0..self.height)
            .map(|row| {
                let mut consecutive = 0;
                (0..self.width).fold(Vec::new(), |mut acc, column| {
                    let tile_is_empty =
                        self.board_vec[pair_to_index(row, column, self.width)].empty;
                    if consecutive > 0 && tile_is_empty {
                        acc.push(consecutive);
                        consecutive = 0;
                        acc
                    } else if !tile_is_empty && column == self.width - 1 {
                        consecutive += 1;
                        acc.push(consecutive);
                        acc
                    } else if tile_is_empty {
                        acc
                    } else {
                        consecutive += 1;
                        acc
                    }
                })
            })
            .collect();
    }
    pub fn new(width: usize, height: usize, filled_count: usize) -> Self {
        let mut board = Self::gen_empty(width, height);
        board.fill_boxes_randomly(filled_count);
        board.filled_count = filled_count;
        board.count_vertical();
        board.count_horizontal();
        board
    }
}
