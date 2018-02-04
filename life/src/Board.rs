//
// Game of Life (2018)
//
// Board.rs
//

use rand::Rng;
extern crate rand;

pub struct Board();

impl Board {
    board: Vec<Vec<bool>>;

    static MATRIX: &'static [(i8, i8)] =
    &[
        (-1,-1),(-1, 0),(-1, 1),
        ( 0,-1),( 0, 0),( 0, 1),
        ( 1,-1),( 1, 0),( 1, 1),
    ];

    fn get_num_neighbours(board: &Vec<Vec<bool>>, cord: (usize,usize)) -> u8 {
        let mut num = 0u8;
        let (row, col) = cord;

        for m in MATRIX {
            if m.0 == 0 && m.1 == 0 {
                continue;
            }
            let mut x = row as i8 + m.0;
            let mut y = col as i8 + m.1;

            if y < 0 {
                y += board[0].len() as i8;
            }
            if y >= board[0].len() as i8 {
                y = 0;
            }
            if x < 0 {
                x += board.len() as i8;
            }
            if x >= board.len() as i8 {
                x = 0;
            }
            if board[x as usize][y as usize] {
                num += 1;
            }
        }
        num
    }

    fn is_cell_alive(board: &Vec<Vec<bool>>, cord: (usize,usize)) -> bool {
        let (row, col) = cord;
        let mut num = get_num_neighbours(board, (row, col));
        let is_alive = board[row][col];

        // Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
        // Any live cell with more than three live neighbours dies, as if by overpopulation.
        let rule_1_3 = (is_alive && !(num < 2 || num > 3));
        // Any live cell with two or three live neighbours lives on to the next generation.
        let rule_2 = (is_alive && (num == 2 || num == 3));
        // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
        let rule_4 = (!is_alive && num == 3);

        rule_1_3 || rule_2 || rule_4
    }

    fn update(board: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
        let mut new_board = Vec::new();

        for (r_num, row) in board.iter().enumerate() {
            let mut row_vec = Vec::new();
            for (c_num, col) in row.iter().enumerate() {
                if is_cell_alive(&board, (r_num, c_num)) {
                    row_vec.push(true);
                } else {
                    row_vec.push(false);
                }
            }
            new_board.push(row_vec);
        }
        new_board
    }

    fn init_rd() -> Vec<Vec<bool>> {
        let height = 40;
        let width = 120;
        let mut board = Vec::new();
        let mut rng = rand::thread_rng();

        for row in 0..height {
            let mut row_vec = Vec::new();

            for col in 0..width {
                if rng.gen() {
                    row_vec.push(true);
                } else {
                    row_vec.push(false);
                }
            }
            board.push(row_vec);
        }
        board
    }

    fn to_string(board: &Vec<Vec<bool>>) -> String {
        let mut result = String::new();

        for row in board {
            for col in row {
                if col == &false {
                    result.push(' ');
                } else {
                    result.push('#');
                }
            }
            result.push('\n');
        }
        result
    }
}
