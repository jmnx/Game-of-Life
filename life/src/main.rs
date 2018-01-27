use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::stdin;
use std::path::Path;
use std::process;
use std::thread;
use std::time;

use rand::Rng;

extern crate rand;

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
                row_vec.push(true);
            } else {
                row_vec.push(false);
            }
        }
        board.push(row_vec);
    }
    board
}
fn load_board_from_file(name: &String) -> Vec<Vec<bool>> {
    let file = File::open(name).unwrap();
    let mut board = Vec::new();
    let mut content = BufReader::new(file);

    for (i,line) in content.lines().enumerate() {
        let l = line.unwrap();
        let mut row_vec = Vec::new();

        for c in l.chars() {
            if c == '#' {
                row_vec.push(true);
            } else {
                row_vec.push(false);
            }
        }
        board.push(row_vec);
    }
    board
}
fn save_board_to_file(text: &String, name: &String) {
    let path = Path::new(name);

    let mut file = match File::create(&path) {
        Err(e) => panic!("No no File!!!"),
        Ok(file) => file,
    };
    match file.write_all(text.as_bytes()) {
        Err(e) => panic!("No no Write!!!"),
        Ok(_) => println!("Wrote File!")
    };
}
fn draw(board: &Vec<Vec<bool>>) -> String {
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

fn print_help() {
    println!("Game of Life\n");
    println!("Implemented in Rust by @svenstaro and @jmnx\n");
    println!("This is the Help\nCommands:");
    println!("  --help -h            this help");
    println!("  --load <filename>    load seed from file and runs it");
    println!("  --save <filename>    save seed to file and exit");
    println!("  --random             generates random seed and runs it");
    println!("  --interactive        startes in interactive mode");
    println!("  --todo               print a list of TODOs :-O");
}

fn run_it(init_board: Vec<Vec<bool>>) {
    let mut i = 0;
    let mut board: Vec<Vec<bool>> = init_board;

    loop {
        i += 1;
        println!("{}[2J", 27 as char);
        print!("{}", draw(&board));
        println!("\n------------------------------------\n Step: {} \n", i);

        let milis = time::Duration::from_millis(100);
        thread::sleep(milis);

        board = update(board);
    }
}

fn interactive () {
    let mut input = String::new();
    let run = true;
    while(run) {
        println!("commands: [load/generate/save]");
        match stdin().read_line(&mut input) {
            Err(e) => {
                println!("Error");
                exit();
            }
            Ok(n) => if input != "load\n" {
                println!("{} <- not y", input);
                exit();
            }
        }
    }
}

fn todos() {
    println!("TODOs:");
    println!("* clear warnings");
    println!("* clean up");
    println!("* interactive");
    println!("* object orientation");
    println!("* by step save");
    println!("* image generator");
    println!("* colors");
    println!("* config. for dead/alive cells");
    println!("* GoL Webserver :-P");
    println!("* GoL on the Blockchain :-P");
}

fn exit () {
    process::exit(0x0f00);
}

fn main() {
    let mut board: Vec<Vec<bool>>;
    let args: Vec<String> = env::args().collect();
    let command: String;
    let file_name: String;
    let extra_arg: String;

    if args.len() >= 2 {

        command = args[1].to_string();
        if args.len() >= 3 {
            file_name = args[2].to_string();
            extra_arg = args[3].to_string();

            println!("Args: {} {} {} \n", command, file_name, extra_arg);

            if command == "--load" {
                board = load_board_from_file( &file_name );
                run_it(board);
            } else if command == "--save" {
                board = init_rd();
                save_board_to_file( &draw(&board), &file_name );
                exit();
            }
        } else {
            println!("Args: {} \n", command);
        }

        if command == "--help" || command == "-h" {
            print_help();
            exit();
        } else if command == "--random" {
            board = init_rd();
            run_it(board);
        } else if command == "--interactive" {
            println!("TODO");
        } else if command == "--todos" {
            todos();
            exit();
        }

        if args.len() >= 3 {
        }
    }
}
