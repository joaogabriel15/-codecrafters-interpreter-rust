use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

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
                let file_contents_chars = file_contents.chars();
                let mut index = 1;
                let mut last_char:char = '\0';

                let _ = file_contents_chars.for_each(|char|{
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
                        '/' => println!("SLASH / null"),
                        '=' => {
                            if last_char == '='{
                                println!("EQUAL_EQUAL == null");
                            }else {
                                println!("EQUAL = null");
                            }
                        },
                        '\n'=> index += 1,
                        _ => {
                            eprintln!("[line {}] Error: Unexpected character: {}", index, char);
                            exit_code = 65;
                        }
                    };

                    if last_char == '=' && char == '=' {
                        last_char = '\0'
                    }else{
                        last_char = char;
                    }
                });

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
