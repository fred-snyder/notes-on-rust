// printing and formatting strings
// https://doc.rust-lang.org/std/fmt/index.html

fn main() {
    // define a variable of type unsigned intiger
    // and assign a value to the variable
    let some_number: u8 = 128;
    let some_other_number: u8 = 64;
    // print the variable values and sum to the numbers
    println!("Some number is {}.", some_number);
    println!("Some other number is {}.", some_other_number);
    println!("Summed together: {}.", some_number + some_other_number);

    // new line
    println!("Print a couple of empty lines: \n\n");

    // string formatting with indexes
    println!(
        "The Shebang is born on the {2}th of {1} in the year {0}!",
        2020, "January", 10
    );

    // string formatting with identifiers
    println!(
        "The Shebang is born on the {day}th of {month} in the year {year}!",
        year = 2020,
        month = "January",
        day = 10
    );

    // string formatting with numerics
    println!(
        "Decimal number {}\nin Binary: {:b}\nin Hexadecimal: {:x}\nin Octal: {:o}",
        100, 100, 100, 100
    );

    // raw formatted, so line-breaks (and tabs) are interpreted
    println!(
        "Decimal number {}
        in Binary: {:b}
        in 8-bit Binary: {:08b}
        in Hexadecimal: {:x}
        in Octal: {:o}",
        50, 50, 50, 50, 50
    );

    // the ?-character is a debug formatting option
    // https://doc.rust-lang.org/std/fmt/index.html#formatting-traits
    println!("{:?}", ("Fred", 98, false));
    println!("{:#?}", ("Fred", 98, false));
    println!("My name is {}.", ("Fred", 98, false).0);
}
