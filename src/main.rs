use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

fn is_alpha(c: char) -> bool{
    return (c >= 'a' && c <= 'z') ||
           (c >= 'A' && c <= 'Z') ||
            c == '_';
  }


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut exit_code: i32 = 0;
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            // Uncomment this block to pass the first stage
            if !file_contents.is_empty() {
                let mut file_contents_chars = file_contents.chars().peekable();
                let mut index = 1;

                while let Some(char) = file_contents_chars.next() {
                    match char {
                        '(' => println!("LEFT_PAREN ( null"),
                        ')' => println!("RIGHT_PAREN ) null"),
                        '{' => println!("LEFT_BRACE {{ null"),
                        '}' => println!("RIGHT_BRACE }} null"),
                        '*' => println!("STAR * null"),
                        '.' => println!("DOT . null"),
                        ',' => println!("COMMA , null"),
                        '+' => println!("PLUS + null"),
                        '-' => println!("MINUS - null"),
                        ';' => println!("SEMICOLON ; null"),
                        '/' => {
                            if let Some('/') = file_contents_chars.peek() {
                                file_contents_chars.next();
                                while let Some(c) = file_contents_chars.next() {
                                    if c == '\n' {
                                        index += 1;
                                        break;
                                    }
                                }
                            } else {
                                println!("SLASH / null");
                            }
                        }
                        '"' => {
                            let mut word = String::new();
                            let mut is_closed = false;
                            while let Some(c) = file_contents_chars.next() {
                                if c == '"' {
                                    is_closed = true;
                                    break;
                                } else if c == '\n' || file_contents_chars.peek().is_none() {
                                    eprintln!("[line {}] Error: Unterminated string.", index);
                                    exit_code = 65;
                                    break;
                                } else {
                                    word.push(c);
                                }
                            }
                            if is_closed {
                                println!("STRING \"{}\" {}", word, word);
                            }
                        }
                        c if is_alpha(c) =>{
                            let mut word = String::new();
                            word.push(c);

                            while let Some(c) = file_contents_chars.next() {
                                if c != ' ' && (is_alpha(c) || c.is_digit(10)) {
                                    word.push(c);
                                }

                                if c == '\n' || c == ' ' || !(is_alpha(c) || !c.is_digit(10))  || file_contents_chars.peek() == None  {
                                    break;
                                } 
                                
                            }
                            println!("IDENTIFIER {} null", word);
                        },
                        '!' => {
                            if let Some('=') = file_contents_chars.peek() {
                                println!("BANG_EQUAL != null");
                                file_contents_chars.next();
                            } else {
                                println!("BANG ! null");
                            }
                        }
                        '<' => {
                            if let Some('=') = file_contents_chars.peek() {
                                println!("LESS_EQUAL <= null");
                                file_contents_chars.next();
                            } else {
                                println!("LESS < null");
                            }
                        }
                        '>' => {
                            if let Some('=') = file_contents_chars.peek() {
                                println!("GREATER_EQUAL >= null");
                                file_contents_chars.next();
                            } else {
                                println!("GREATER > null");
                            }
                        }
                        '0'..='9' => {
                            let mut number = String::new();
                            number.push(char);
                        
                            while let Some(&next_char) = file_contents_chars.peek() {
                                if next_char.is_digit(10) || next_char == '.' {
                                    number.push(file_contents_chars.next().unwrap());
                                } else {
                                    break;
                                }
                            }
                        

                            match number.parse::<f64>() {
                                Ok(num) => println!("NUMBER {} {:.?}", number, num),
                                Err(_) => {
                                    eprintln!("[line {}] Error: Invalid number format: {}", index, number);
                                    exit_code = 65;
                                }
                            }
                        },
                        '=' => {
                            if let Some('=') = file_contents_chars.peek() {
                                println!("EQUAL_EQUAL == null");
                                file_contents_chars.next();
                            } else {
                                println!("EQUAL = null");
                            }
                        }
                        ' ' | '\r' | '\t' => {}
                        '\n' => index += 1,
                        _ => {
                            eprintln!("[line {}] Error: Unexpected character: {}", index, char);
                            exit_code = 65;
                        }
                    }
                }

                println!("EOF  null");
                if exit_code != 0 {
                    exit(exit_code)
                }

                return;
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
