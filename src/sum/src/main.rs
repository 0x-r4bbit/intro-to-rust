use std::io;
use std::process;

fn main() {
  loop {
    println!("Please enter the first number: ");

    let mut first = String::new();
    io::stdin().read_line(&mut first).unwrap();

    let a: u32;

    match first.trim().parse() {
      Ok(val) => a = val,
      Err(_err) => {
        println!("Not a number!");
        process::exit(1)
      }
    };

    println!("Please enter a second number: ");

    let mut second = String::new();
    io::stdin().read_line(&mut second).unwrap();

    let b: u32;

    match second.trim().parse() {
      Ok(val) => b = val,
      Err(_err) => {
        println!("Not a number!");
        process::exit(1)
      }
    };

    let result = sum(a, b);
    println!("Result: {}", result);
  }
}

fn sum(a: u32, b: u32) -> u32 {
  a + b
}
