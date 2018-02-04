//
// Game of Life (2018)
//
// main.rs
//

use std::env;
use std::io::stdin;
use std::process;
use std::thread;
use std::time;

const DEBUG: bool = false;
const VERSION: String = "0.1";

fn print_help() {
    println!("Game of Life\n");
    println!("Implemented in Rust by @svenstaro and @jmnx\n");
    println!("Usage: life [OPTION] <filename> [2nd OPTION] <parameter>");
    println!(" Options:");
    println!("  --help -h            this help");
    println!("  --load <filename>    load seed from file and runs it");
    println!("  --save <filename>    save a random seed to file and exit");
    println!("  --random             generates random seed and runs it");
    println!("  --interactive        startes in interactive mode");
    println!("  --todo               print a list of TODOs");
    if VERSION == "0.3" {
        println!("\n 2nd Options:");
        println!("  --save-steps         save every step to an file in ./steps/step_<n>.life");
        println!("  --png                export every step as an png");
        println!("  --run <n>            just run for n steps");
    }
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
