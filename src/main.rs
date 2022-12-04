use std::{env, fs, process};

const INCREMENT: char = '+';
const DECREMENT: char = '-';
const RIGHT: char = '>';
const LEFT: char = '<';
const LOOP_START: char = '[';
const LOOP_END: char = ']';
const OUTPUT: char = '.';
const INPUT: char = ',';

const MEMORY_SIZE: usize = 1024;

fn main() {
    let mut memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];
    let mut ptr: usize = 0;
    let mut code_ptr: usize = 0;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: A Brainfuck code is not passed as a command-line argument.");
        eprintln!("Please pass an argument as the form, $ ./brainfuck [FILENAME].");
        process::exit(1);
    }

    let code: String = fs::read_to_string(&args[1])
        .expect(&format!("Error: The file, {}, cannot be opened.", args[1]));

    let code_len = code.len();

    while code_ptr < code_len {
        match code.chars().nth(code_ptr).unwrap() {
            INCREMENT => memory[ptr] = memory[ptr].wrapping_add(1),

            DECREMENT => memory[ptr] = memory[ptr].wrapping_sub(1),

            RIGHT => ptr = if ptr >= MEMORY_SIZE - 1 { 0 } else { ptr + 1 },

            LEFT => ptr = if ptr <= 0 { MEMORY_SIZE - 1 } else { ptr - 1 },

            LOOP_START => (),

            LOOP_END => (),

            OUTPUT => (),

            INPUT => (),

            _ => (),
        }

        code_ptr += 1;
    }
}
