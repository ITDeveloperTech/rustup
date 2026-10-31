fn main() {
    println!("Hello, world");

    //instruction can't return value
    let x = 6;

    // let x = (let y = 6);
    // is error (not return value)

    another_function(x, 'h');

    // This code block is expression
    let y = {
        let x = 3;
        x + 1 // it's expression because x + 1 not have a breakspace => ';'
    };

    println!("The value of y is: {y}");

    let five = five();

    println!("Give me a five: {five}");

    // It's a comment

    // It's a multiline comment?
    // Maybe
    // But not cozy

    //-----------------------

    // conditional code
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition {5} else {6};

    println!("The value of number is: {number}");

    let mut i = 1;
    let cycle_result = loop {
        println!("again!");
        if i == 3 {
            break 15;
        }
        i += 1;
    };

    println!("cycle_result: {cycle_result}");

    // --------------
    //cycle label
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // ------------------
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    // --------------------

    let a = [10, 20, 30, 40 ,50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // Like a foreach better and faster
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }

}

//instruction function
fn another_function(x: i32, unit_label: char) {
    println!("The measurement is: {x}{unit_label}");
}

//expression function
fn five() -> i32 {
    5
}
