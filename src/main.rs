use std::{
    fs::File,
    io::{Read, Write},
};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 1 {
        panic!("Error! No file target found :(");
    }

    let mut input = String::new();
    let mut file = File::open(&args[1]).expect("Invalid file provided :(");
    file.read_to_string(&mut input)
        .expect("Failed to write to buffer");

    let mut curr_instance = BfInstance::default();
    curr_instance.eval(&input);
}

pub struct BfInstance {
    memory: [u8; 30_000],
    cursor: usize,
}

impl Default for BfInstance {
    fn default() -> Self {
        Self {
            memory: [0; 30_000],
            cursor: 0,
        }
    }
}

impl BfInstance {
    pub fn eval(&mut self, input: &str) {
        let mut instruction_pointer = 0;
        let input_bytes = input.as_bytes();

        while instruction_pointer < input_bytes.len() {
            match input_bytes[instruction_pointer] as char {
                '>' => self.right(),
                '<' => self.left(),
                '+' => self.inc(),
                '-' => self.dec(),
                ',' => self.read_char().expect("Failed to read char from stdin"),
                '.' => self.print_char(),
                '[' => {
                    if self.get() == 0 {
                        let mut loop_depth = 1;
                        while loop_depth > 0 {
                            instruction_pointer += 1;
                            if instruction_pointer >= input_bytes.len() {
                                panic!("Unmatched '[' encountered");
                            }
                            match input_bytes[instruction_pointer] as char {
                                '[' => loop_depth += 1,
                                ']' => loop_depth -= 1,
                                _ => {}
                            }
                        }
                    }
                }
                ']' => {
                    if self.get() != 0 {
                        let mut loop_depth = 1;
                        while loop_depth > 0 {
                            if instruction_pointer == 0 {
                                panic!("Unmatched ']' encountered");
                            }
                            instruction_pointer -= 1;
                            match input_bytes[instruction_pointer] as char {
                                '[' => loop_depth -= 1,
                                ']' => loop_depth += 1,
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }

            instruction_pointer += 1;
        }
    }
    pub fn jump(&mut self, to: usize) {
        self.cursor = to
    }

    pub fn get(&self) -> u8 {
        self.memory[self.cursor]
    }

    pub fn set(&mut self, val: u8) {
        self.memory[self.cursor] = val
    }

    pub fn inc(&mut self) {
        self.memory[self.cursor] += 1
    }

    pub fn dec(&mut self) {
        self.memory[self.cursor] -= 1
    }

    pub fn right(&mut self) {
        self.cursor += 1
    }

    pub fn left(&mut self) {
        self.cursor -= 1
    }

    pub fn read_char(&mut self) -> std::io::Result<()> {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;

        self.set(buffer.chars().collect::<Vec<_>>()[0] as u8);

        Ok(())
    }

    pub fn print_char(&self) {
        print!("{}", self.memory[self.cursor] as char);
        let _ = std::io::stdout().flush();
    }
}
