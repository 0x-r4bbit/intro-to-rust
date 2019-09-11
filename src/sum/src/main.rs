use std::io;

fn main() {
  println!("Please enter the first number: ");

  let mut first = String::new();
  io::stdin().read_line(&mut first).unwrap();

  let a: u32 = first.trim().parse().expect("Not a number");

  println!("Please enter a second number: ");

  let mut second = String::new();
  io::stdin().read_line(&mut second).unwrap();

  let b: u32 = second.trim().parse().expect("Not a number");

  let result = sum(a, b);
  println!("Result: {}", result);
}

fn sum(a: u32, b: u32) -> u32 {
  a + b
}
