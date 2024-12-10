//! BF Interpretter Implementation

use std::io::Write;

/// A BF instance, containing a memory strip of 30,000 elements and a pointer to one of these
/// memory cells
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
    /// Evaluate a string as valid BF
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

    /// Gets the current memory cell
    pub fn get(&self) -> u8 {
        self.memory[self.cursor]
    }

    /// Sets the current memory cell
    pub fn set(&mut self, val: u8) {
        self.memory[self.cursor] = val
    }

    /// Increments the current memory cell
    pub fn inc(&mut self) {
        self.memory[self.cursor] += 1
    }

    /// Decrements the current memory cell
    pub fn dec(&mut self) {
        self.memory[self.cursor] -= 1
    }

    /// Moves the memory pointer right
    pub fn right(&mut self) {
        self.cursor += 1
    }

    /// Moves the memory pointer left
    pub fn left(&mut self) {
        self.cursor -= 1
    }

    /// Reads a character from stdin and sets it's byte value to the current memory cell
    pub fn read_char(&mut self) -> std::io::Result<()> {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;

        self.set(buffer.chars().collect::<Vec<_>>()[0] as u8);

        Ok(())
    }

    /// Prints the current memory cell as a char
    pub fn print_char(&self) {
        print!("{}", self.memory[self.cursor] as char);
        let _ = std::io::stdout().flush();
    }
}
