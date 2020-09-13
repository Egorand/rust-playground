fn main() {
    // Integers

    let dec = 98_222;
    let hex = 0xff;
    let oct = 0o77;
    let bin = 0b1111_0000;
    let byt = b'A';

    println!("dec = {}, hex = {}, oct = {}, bin = {}, byt = {}",
        dec, hex, oct, bin, byt);

    // Floats

    let x = 2.0;
    let y: f32 = 3.0;

    println!("x = {}, y = {}", x, y);

    // Ops

    let sum = 5 + 10;
    let diff = 95.5 - 4.3;
    let prod = 4 * 30;
    let quot = 56.7 / 32.2;
    let rem = 43 % 5;

    println!("sum = {}, diff = {}, prod = {}, quot = {}, rem = {}",
        sum, diff, prod, quot, rem);

    // Booleans

    let t = true;
    let f: bool = false;

    println!("t = {}, f = {}", t, f);

    // Chars

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c = {}, z = {}, cat = {}", c, z, heart_eyed_cat);

    // Tuple

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("tup = {:?}", tup);

    let (x, y, z) = tup;

    println!("x = {}, y = {}, z = {}", x, y, z);

    println!("x = {}, y = {}, z = {}", tup.0, tup.1, tup.2); 
    
    // Array

    let a = [1, 2, 3, 4, 5];

    println!("a = {:?}", a);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("a = {:?}", a);
    println!("first = {}, second = {}", a[0], a[1]);

    let a = [3; 5];

    println!("a = {:?}", a);

    // Will panic at runtime - invalid index
    //
    // let index = 10;
    // println!("{:?}", a[index]);
}
