mod program;
mod vmem;

use program::Program;

fn main() {
    let mut program = Program::new(String::from("bf_src/tictactoe.bf"), String::from("Hello World"));
    program.init_program();
    program.execute_full();
}
