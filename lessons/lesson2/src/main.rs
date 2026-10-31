fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("This is guess score: {guess}");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!(
        "sum - {sum} | difference - {difference} | product - {product} | quotient - {quotient} | truncated - {truncated} | remainder - {remainder}"
    );

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //let (five_hundred, six_point_four, one) = tup;

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of x,y,z is: {five_hundred},{six_point_four},{one}");

    // --------------------

    //let months = [...];
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a = [3; 5]; // [3, 3, 3, 3, 3]

    let january = months[0];
    let february = months[1];

    let first = a[0];
    let second = a[1];

    println!("months => {january}, {february} numbers => {first}, {second}");

}
