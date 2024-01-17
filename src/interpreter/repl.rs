use std::io::{self, Write};
use crate::interpreter::Stack;
use std::env;
use std::fs;
use super::interpret;

pub fn repl() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 && &args[1] == "-f" {
        let filename = &args[2];
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        //println!("With text:\n{}", contents);

        let mut stack = Stack::new();
        
        let _ = interpret(&contents, &mut stack);
        println!("{:?}", stack);
    } else {
        let _ = interactive_prompt(&read_user_input);
    }
}

pub fn read_user_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{} ", prompt);
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

pub fn interactive_prompt(read_input: &dyn Fn(&str) -> String) -> Result<(), String> {
    let mut stack = Stack::new();
    loop {
        let command = read_input(">");
        match command.as_str().trim() {
            "quit" => break,
            "print" => println!("{:?}", stack),
            cmd => {
                let result = interpret(cmd, &mut stack);
                match result {
                    Ok(_) => (),
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }
    Ok(())
}

