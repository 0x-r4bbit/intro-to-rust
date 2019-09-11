use std::io;

fn main() {
  println!("Please enter the first number: ");

  let mut first = String::new();
  io::stdin().read_line(&mut first).unwrap();

  let mut a: u32 = 0;

  match first.trim().parse() {
    Ok(val) => a = val,
    Err(_err) => println!("Not a number!")
  };

  println!("Please enter a second number: ");

  let mut second = String::new();
  io::stdin().read_line(&mut second).unwrap();

  let mut b: u32 = 0;

  match second.trim().parse() {
    Ok(val) => b = val,
    Err(_err) => println!("Not a number!")
  };

  let result = sum(a, b);
  println!("Result: {}", result);
}

fn sum(a: u32, b: u32) -> u32 {
  a + b
}
