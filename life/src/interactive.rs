

fn parse_command(command: &String) {
    let success: bool;


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
