extern crate rand;

use std::{thread, time, env};
use rand::Rng;

static MATRIX: &'static [(i8, i8)] =
&[
    (-1,-1),(-1, 0),(-1, 1),
    ( 0,-1),( 0, 0),( 0, 1),
    ( 1,-1),( 1, 0),( 1, 1),
];
const DEBUG: bool = false;

fn init_rd() -> Vec<Vec<bool>> {
    let height = 40;
    let width = 120;
    let mut board = Vec::new();
    let mut rng = rand::thread_rng();

    for row in 0..height {
        let mut row_vec = Vec::new();
        for col in 0..width {
            if rng.gen() {
                row_vec.push(false)
            } else {
                row_vec.push(true)
            }
        }
        board.push(row_vec);
    }
    board
}

fn draw(board: &Vec<Vec<bool>>) {
    for row in board {
        for col in row {
            if col == &false {
                print!(" ");
            } else {
                print!("#");
            }
        }
        print!("\n");
    }
}
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
            y = 0 as i8;
        }
        if x < 0 {
            x += board.len() as i8;
        }
        if x >= board.len() as i8 {
            x = 0 as i8;
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

fn main() {
    let mut board = init_rd();

    let mut i = 0;

    loop {
        i += 1;
        println!("{}[2J", 27 as char);
        draw(&board);
        println!("\n------------------------------------\n Step: {} \n", i);

        let ten_milis = time::Duration::from_millis(100);
        thread::sleep(ten_milis);

        board = update(board);
    }
}
