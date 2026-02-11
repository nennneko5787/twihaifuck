use std::fs;
use std::io::{self, Read, Write};

const MEMORY_SIZE: usize = 30000;

pub fn converter(filename: String, content: String) -> bool {
    let result = fs::write(
        filename,
        content
            .replace("+", "SEX")
            .replace("-", "愛液")
            .replace("[", "H")
            .replace("]", "抜ける")
            .replace(".", "射精")
            .replace(",", "腰")
            .replace(">", "お尻")
            .replace("<", "変態"),
    );

    result.is_ok()
}

pub fn interpreter(content: String) {
    let mut memory: Vec<i32> = vec![0; MEMORY_SIZE];
    let mut pointer = 0;
    let mut head = 0;

    while head < content.len() {
        // +
        if content[head..].starts_with("SEX") {
            memory[pointer] += 1;
            head += "SEX".len();
            continue;
        }
        // -
        if content[head..].starts_with("愛液") {
            memory[pointer] -= 1;
            head += "愛液".len();
            continue;
        }
        // [
        if content[head..].starts_with("H") {
            if memory[pointer] == 0 {
                let mut count = 1;
                head += "H".len();
                while count != 0 {
                    if head >= content.len() {
                        eprintln!("'抜ける' is missing");
                        std::process::exit(1);
                    }
                    if content[head..].starts_with("H") {
                        count += 1;
                        head += "H".len();
                    } else if content[head..].starts_with("抜ける") {
                        count -= 1;
                        if count != 0 {
                            head += "抜ける".len();
                        }
                    } else {
                        let ch = content[head..].chars().next().unwrap();
                        head += ch.len_utf8();
                    }
                }
            }
            head += "H".len();
            continue;
        }
        // ]
        if content[head..].starts_with("抜ける") {
            if memory[pointer] != 0 {
                let mut count = 1;
                head -= 1;
                while head > 0 && !content.is_char_boundary(head) {
                    head -= 1;
                }

                while count != 0 && head > 0 {
                    if content[head..].starts_with("抜ける") {
                        count += 1;
                    } else if content[head..].starts_with("H") {
                        count -= 1;
                        if count == 0 {
                            break;
                        }
                    }

                    if head == 0 {
                        break;
                    }
                    head -= 1;
                    while head > 0 && !content.is_char_boundary(head) {
                        head -= 1;
                    }
                }

                if count != 0 {
                    eprintln!("'H' is missing");
                    std::process::exit(1);
                }
                continue;
            }
            head += "抜ける".len();
            continue;
        }
        // .
        if content[head..].starts_with("射精") {
            let byte = (memory[pointer] % 256) as u8;
            io::stdout().write_all(&[byte]).unwrap();
            io::stdout().flush().unwrap();
            head += "射精".len();
            continue;
        }
        // ,
        if content[head..].starts_with("腰") {
            let mut buffer = [0u8; 1];
            io::stdin().read_exact(&mut buffer).unwrap();
            memory[pointer] = buffer[0] as i32;
            head += "腰".len();
            continue;
        }
        // >
        if content[head..].starts_with("お尻") {
            pointer += 1;
            if pointer >= MEMORY_SIZE {
                eprintln!("Overflow!");
                std::process::exit(1);
            }
            head += "お尻".len();
            continue;
        }
        // <
        if content[head..].starts_with("変態") {
            if pointer == 0 {
                eprintln!("Can't decrement anymore");
                std::process::exit(1);
            }
            pointer -= 1;
            head += "変態".len();
            continue;
        }

        let ch = content[head..].chars().next().unwrap();
        head += ch.len_utf8();
    }
}
