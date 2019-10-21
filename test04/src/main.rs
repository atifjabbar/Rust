fn main() {
    println!("Hello, world!");
        // addition
    let sum = 5 + 10;
    println!("Hello, world!, {}",sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("Hello, world!, {}",difference);
    // multiplication
    let product = 4 * 30;
    println!("Hello, world!, {}",product);
    // division
    let quotient = 56.7 / 32.2;
    println!("Hello, world!, {}",quotient);
    // remainder
    let remainder = 43 % 5;
    println!("Hello, world!, {}",remainder);

    let tup = (500, 6.4, 1);
    let (_x, _y, _z) = tup;
    println!("The value of y is: {}", _y);

    let _a = [3; 5];
//    println!("The value of y is: {}", _a);
}
