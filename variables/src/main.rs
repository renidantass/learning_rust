fn main() {
    // variables are immutable by default
    let mut x = 5;
    println!("The value of x is {x}");

    x = 6;
    println!("The value of x is {x}");


    // constants can only be assigned with expressions
    const THREE_HOURS_IN_SECOND: u32 = 60 * 60 * 3;


    // shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // diference between shadowing and mut

    let spaces = "    ";
    let spaces = spaces.len(); // updating the type of variable

    // instead of
    // let mut spaces =  "    ";
    // spaces = spaces.len();

    // datatypes
    let guess: u32 = "42".parse().expect("Not a number!");


    // datatypes_integers

    let signed_integer: u32 = 132;
    let unsigned_integer: i32 = -132;
    let any_integer = 98u32;

    // datatypes_float according to the IEEE-754 standard

    let fp = 2.0; // f64 single precision
    let fp1: f32 = 3.0; // f32 double precision

    // datatypes_boolean

    let t = true;
    let f: bool = false;

    // datatypes_char

    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    // datatypes_tuple

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup; // destructuring tuple into variables

    println!("The value of y is: {y}");

    let five_hundred = tup.0;

    println!("The value of 0 index in tup is {five_hundred}");

    // datatypes_array

    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // outputs: [3, 3, 3, 3, 3]

    let first = a[0];
    let second = a[1];

    println!("the first element is {} and second is {}", first, second);
}
