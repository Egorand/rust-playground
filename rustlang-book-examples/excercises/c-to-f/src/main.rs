use std::io;
use std::process;

fn main() {
  let celsius = read_temperature_in_celsius();
  println!("Celsius: {}º", celsius);

  let fahrenheit = celsius_to_fahrenheit(celsius);
  println!("Fahrenheit: {}º", fahrenheit);
}

fn read_temperature_in_celsius() -> f32 {
  println!("Please enter temperature in degrees Celsius:");

  let mut temperature = String::new();
  io::stdin()
    .read_line(&mut temperature)
    .expect("Failed to read line.");

  match temperature.trim().parse() {
    Ok(num) => num,
    Err(_) => {
      eprintln!("Input invalid, please try again!");
      process::exit(1);
    },
  }
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
  (celsius * (9.0 / 5.0)) + 32.0
}
