use std::{
    fs::File,
    io::{Read, Write},
};

use bf_interp::interp::BfInstance;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut curr_instance = BfInstance::default();

    if args.len() <= 1 {
        println!("Entering BF REPL Environment");

        loop {
            let mut buffer = String::new();
            print!("> ");
            let _ = std::io::stdout().flush();

            std::io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read input");

            match buffer.trim() {
                "exit" => {
                    println!("Exiting BF REPL.");
                    break;
                }
                "reset" => {
                    curr_instance = BfInstance::default();
                }
                "" => continue,
                _ => {}
            }

            curr_instance.eval(buffer.trim());
            println!("\t\t\t\t\t\tCurrent Cell: {}", curr_instance.get());
        }
    } else {
        let mut input = String::new();
        let mut file = File::open(&args[1]).expect("Invalid file provided :(");
        file.read_to_string(&mut input)
            .expect("Failed to write to buffer");

        curr_instance.eval(&input);
    }
}
