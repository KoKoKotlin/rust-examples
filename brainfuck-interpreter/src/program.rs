use std::{fs::File, io::{BufReader, Read}};

use crate::vmem::VMem;

#[derive(Debug)]
pub struct Program {
    file_name: String,
    program_counter: usize,
    instructions: Vec<char>,
    name: String,
    mem: VMem
}

fn load_src_file(file_path: &str) -> Vec<char> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => panic!("Can't open src file!"),
    };

    let mut reader = BufReader::new(file);

    let mut result: Vec<u8> = vec![];
    let mut buf = vec![];
    
    match reader.read_to_end(& mut buf) {
        Ok(_)  => result.append(& mut buf),
        Err(_) => panic!("Can't read src file!"),
    };

    let valid_symbols = vec!['<', '>', '+', '-', '[', ']', '.', ','];
    result
        .into_iter()
        .map(|b| b as char)
        .filter(|c| valid_symbols.contains(c))
        .collect()
}

impl Program {

    pub fn new(file_name: String, name: String) -> Self {
        Program {
            file_name: file_name,
            program_counter: 0,
            instructions: vec![],
            name: name,
            mem: VMem::new(),
        }
    }

    pub fn init_program(& mut self) {
        self.instructions = load_src_file(&self.file_name);
    }

    pub fn execute_full(& mut self) {
        println!("Starting to execute program: {}", self.name);
        while self.program_counter < self.instructions.len() {
            self.single_step();
        }
    }

    pub fn single_step(& mut self) {
        let current_inst = self.instructions[self.program_counter];
        // print!("{}: {}", self.program_counter, current_inst);
        match current_inst {
            '<' => self.mem.move_left(),
            '>' => self.mem.move_right(),
            '+' => self.mem.inc(),
            '-' => self.mem.dec(),
            '[' => {
                if self.mem.read() == 0 {
                    let mut bracket_count = 1;
                    loop {
                        if self.program_counter == self.instructions.len() - 1 || bracket_count == 0 {
                            break;
                        }

                        self.program_counter += 1;
                        bracket_count += match self.instructions[self.program_counter] {
                            '[' => 1,
                            ']' => -1,
                            _ => 0,
                        };
                    }
                }
            },
            ']' => {
                let mut bracket_count = 1;
                loop {
                    if self.program_counter == 0 || bracket_count == 0 {
                        break;
                    }

                    self.program_counter -= 1;

                    bracket_count += match self.instructions[self.program_counter] {
                        '[' => -1,
                        ']' => 1,
                        _ => 0,
                    };
                }
                self.program_counter -= 1;
            },
            '.' => print!("{}", self.mem.read().to_be_bytes()[0] as char),
            ',' => {
                let mut input = String::from("");
                println!("Input byte: ");
                while let Err(_) = std::io::stdin().read_line(& mut input) { println!("Error reading input!") }
                let first_byte = input.as_bytes()[0];
                self.mem.write(first_byte as i8);
            },
            _ => {},
        }

        self.program_counter += 1;
        // println!("{}, {} {}", self.program_counter, self.mem.read(), self.mem.pointer);
        // println!(" -> {}: {}, {:?}", self.program_counter, self.instructions[self.program_counter], self.mem);
    }

}