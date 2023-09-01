fn main() {
    println!("Hello, world!");

    another_function();
    let x = function_with_parameters(5, 6);
    println!("The value of x is: {}", x);

    let y = if !is_five(x) { 5 } else { 6 };
    println!("The value of y is: {}", y);
}

fn another_function() {
    println!("Another function.")
}

fn function_with_parameters(number_one: i32, number_two: i32) -> i32 {
    return number_one + number_two;
}

fn is_five(x: i32) -> bool {
    return x == 5;
}