fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("Max points is {}", MAX_POINTS);

    let x = 5;
    let x = x + 1;
    let x = x + 2;
    println!("The value of x is {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces: {}", spaces);

    // Won't compile: mismatched types
    //
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // println!("Spaces: {}", spaces);
}
