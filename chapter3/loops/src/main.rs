fn main() {
    return_value_from_loop();
    label_loops();
    while_loop();
    iterative_for_loop();
    range_for_loop();
}

fn return_value_from_loop() {
    println!("\nreturn_value_from_loop():");
    let mut counter = 0;

    // Loops can evaluate to a value from the break statement
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn label_loops() {
    println!("\nlabel_loops():");
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
}

fn while_loop() {
    println!("\nwhile_loops():");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn iterative_for_loop() {
    println!("\niterative_for_loop():");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn range_for_loop() {
    println!("\nrange_for_loop():");
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
