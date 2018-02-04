

use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


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
