use std::fs;
use std::io::{self, Read, Write};

const MEMORY_SIZE: usize = 30000;

pub fn converter(filename: String, content: String) -> bool {
    let result = fs::write(
        filename,
        content
            .replace("+", "rieru")
            .replace("-", "にゃー子")
            .replace("[", "緑刃")
            .replace("]", "モス")
            .replace(".", "駆ける")
            .replace(",", "れむ")
            .replace(">", "夜桜")
            .replace("<", "紙"),
    );

    result.is_ok()
}

pub fn interpreter(content: String) {
    let mut memory: Vec<i32> = vec![0; MEMORY_SIZE];
    let mut pointer = 0;
    let mut head = 0;

    while head < content.len() {
        // +
        if content[head..].starts_with("rieru") {
            memory[pointer] += 1;
            head += "rieru".len();
            continue;
        }
        // -
        if content[head..].starts_with("にゃー子") {
            memory[pointer] -= 1;
            head += "にゃー子".len();
            continue;
        }
        // [
        if content[head..].starts_with("緑刃") {
            if memory[pointer] == 0 {
                let mut count = 1;
                head += "緑刃".len();
                while count != 0 {
                    if head >= content.len() {
                        eprintln!("'モス' is missing");
                        std::process::exit(1);
                    }
                    if content[head..].starts_with("緑刃") {
                        count += 1;
                        head += "緑刃".len();
                    } else if content[head..].starts_with("モス") {
                        count -= 1;
                        if count != 0 {
                            head += "モス".len();
                        }
                    } else {
                        let ch = content[head..].chars().next().unwrap();
                        head += ch.len_utf8();
                    }
                }
            }
            head += "緑刃".len();
            continue;
        }
        // ]
        if content[head..].starts_with("モス") {
            if memory[pointer] != 0 {
                let mut count = 1;
                head -= 1;
                while head > 0 && !content.is_char_boundary(head) {
                    head -= 1;
                }

                while count != 0 && head > 0 {
                    if content[head..].starts_with("モス") {
                        count += 1;
                    } else if content[head..].starts_with("緑刃") {
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
                    eprintln!("'緑刃' is missing");
                    std::process::exit(1);
                }
                continue;
            }
            head += "モス".len();
            continue;
        }
        // .
        if content[head..].starts_with("駆ける") {
            let byte = (memory[pointer] % 256) as u8;
            io::stdout().write_all(&[byte]).unwrap();
            io::stdout().flush().unwrap();
            head += "駆ける".len();
            continue;
        }
        // ,
        if content[head..].starts_with("れむ") {
            let mut buffer = [0u8; 1];
            io::stdin().read_exact(&mut buffer).unwrap();
            memory[pointer] = buffer[0] as i32;
            head += "れむ".len();
            continue;
        }
        // >
        if content[head..].starts_with("夜桜") {
            pointer += 1;
            if pointer >= MEMORY_SIZE {
                eprintln!("Overflow!");
                std::process::exit(1);
            }
            head += "夜桜".len();
            continue;
        }
        // <
        if content[head..].starts_with("紙") {
            if pointer == 0 {
                eprintln!("Can't decrement anymore");
                std::process::exit(1);
            }
            pointer -= 1;
            head += "紙".len();
            continue;
        }

        let ch = content[head..].chars().next().unwrap();
        head += ch.len_utf8();
    }
}
