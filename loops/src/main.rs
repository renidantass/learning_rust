fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result: {}", result);


    // labeled loops
    let mut second_counter = 0;
    'continuos: loop {
        second_counter += 1;
        println!("Second counter: {}", second_counter);

        if second_counter == 10 {
            break 'continuos;
        }
    }

    // iterating array with 'with'

    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }

    // iterating array with 'for'

    let b = [10, 20, 30, 40, 50];

    for item in b {
        println!("The value arr2 is: {}", item);
    }

    // countdown with 'for'

    for number in (1..4).rev() {
        println!("{number}")
    }


    // from

    let s1 = String::from("hello");
    let s2 = s1.clone();


    println!("{} world", s1);
}
