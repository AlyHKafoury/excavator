use std;
use std::io;
use std::io::Write;
use crate::vm::Vm;

pub struct Repl {
  command_buffer: Vec<String>,
  vm: Vm,
}

impl Repl {
  pub fn new() -> Repl {
    Repl {
      vm: Vm::new(),
      command_buffer: vec![],
    }
  }

  pub fn run(&mut self){
    println!("Welcome to Excavator Vm !!");
    loop {
      let mut buffer = String::new();
      let stdin = io::stdin();

      print!(">>> ");
      io::stdout().flush().expect("Unable to write to stdout");

      stdin.read_line(&mut buffer).expect("Unable to get user input");
      let buffer = buffer.trim_end();

      self.command_buffer.push(buffer.to_string());

      match buffer {
        ".quit" => {
          println!("Shutting down exavator vm");
          std::process::exit(0);
        },
        ".history" => {
          for command in &self.command_buffer {
            println!("{}", command);
          }
        }
        _ => {
          println!("Invalid command!");
        }
      }
    }
  }

}