fn main() {
    let number = 5;

    // Assign a value with an if statement
    let other_number = if true {1} else {0};

    // if else if else
    if number < 5 {
        println!("Number was less than 5");
    } else if number > 10 {
        println!("Number was greater than 10")
    } else {
        println!("Number was between 5 and 10");
    }
}