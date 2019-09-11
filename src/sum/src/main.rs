use std::io;
use std::process;

fn main() {
  loop {
    println!("Please enter the first number: ");
    let a = read_user_input();

    println!("Please enter a second number: ");
    let b = read_user_input();

    let result = sum(a, b);
    println!("Result: {}", result);
  }
}

fn sum(a: u32, b: u32) -> u32 {
  a + b
}

fn read_user_input() -> u32 {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();

  let digit: u32;

  match input.trim().parse() {
    Ok(val) => digit = val,
    Err(_err) => {
      println!("Not a number!");
      process::exit(1)
    }
  };

  digit
}
