use std::io;
use std::process;

fn main() {
  println!("Fibonacci Numbers");
  println!("=================\n");

  let n = read_number();
  println!("N: {}", n);

  let nth_number = nth_fibonacci(n);
  println!("Nth Fibonacci number: {}", nth_number);
}

fn read_number() -> u32 {
  println!("Please enter N:");

  let mut n = String::new();
  io::stdin()
    .read_line(&mut n)
    .expect("Failed to read line.");

  match n.trim().parse() {
    Ok(0) | Err(_) => {
      eprintln!("Please enter a positive number!");
      process::exit(1);
    },
    Ok(num) => num,
  }
}

fn nth_fibonacci(n: u32) -> u32 {
  match n {
    1 => 0,
    2 => 1,
    n => {
      let mut n1 = 0;
      let mut n2 = 1;
      for _ in 0..n - 2 {
        n2 = n1 + n2;
        n1 = n2 - n1;
      }
      n2
    },
  }
}
