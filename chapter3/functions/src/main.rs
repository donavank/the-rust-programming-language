fn main() {
    println!("Hello, world!");
    println!("five(): {}", five());
    println!("also_five(): {}", also_five());
    another_function();
}

fn another_function() {
    println!("Another function.");
}

fn five() -> i32 {
    // Because this function ends with an expression, that expression is implicitly used as
    // the return value
    5
}

fn also_five() -> i32 {
    return 5;
}