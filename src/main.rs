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
}
