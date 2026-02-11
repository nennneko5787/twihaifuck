use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

const MEMORY_SIZE: usize = 30000;

fn interpreter(content: String) {
    let chars: Vec<char> = content.chars().collect();

    let mut memory: Vec<i32> = vec![0; MEMORY_SIZE];
    let mut pointer = 0;
    let mut head = 0;

    while head < chars.len() {
        match chars[head] {
            '+' => memory[pointer] += 1,
            '-' => memory[pointer] -= 1,
            '[' => {
                if memory[pointer] == 0 {
                    let mut count = 1;
                    while count != 0 {
                        head += 1;

                        if head == chars.len() {
                            panic!("']' is missing");
                        }
                        if chars[head] == '[' {
                            count += 1;
                        }
                        if chars[head] == ']' {
                            count -= 1;
                        }
                    }
                }
            }
            ']' => {
                if memory[pointer] != 0 {
                    let mut count = 1;
                    while count != 0 {
                        if head == 0 {
                            panic!("'[' is missing");
                        }
                        head -= 1;

                        if chars[head] == ']' {
                            count += 1;
                        }
                        if chars[head] == '[' {
                            count -= 1;
                        }
                    }
                }
            }
            '.' => {
                let byte = memory[pointer] as u8;
                io::stdout().write_all(&[byte]).unwrap();
            }
            ',' => {
                let mut buffer = [0u8; 1];
                io::stdin().read_exact(&mut buffer).unwrap();
                memory[pointer] = buffer[0] as i32;
            }
            '>' => {
                pointer += 1;
                if pointer > MEMORY_SIZE {
                    panic!("Overflow!")
                }
            }
            '<' => {
                if pointer == 0 {
                    panic!("Can't decrement anymore")
                }
                pointer -= 1;
            }
            _ => {}
        }
        head += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    interpreter(contents);
}
