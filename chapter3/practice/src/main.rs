const DEGREE_SYMBOL: char = '\u{00B0}';

fn main() {
    temperature_conversions();
    println!("The {}th fibonacci number is: {}", 10, fibonacci(10));
}

// Temperature Conversion Code
fn temperature_conversions() {
    let celsius = 100.00;
    let fahrenheit = to_fahrenheit(celsius);

    println!(
        "From {} to {} and back to {}",
        format_temperature(celsius, 'C'),
        format_temperature(fahrenheit, 'F'),
        format_temperature(to_celsius(fahrenheit), 'C')
    );
}

fn to_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) * 5.0 / 9.0
}

fn to_fahrenheit(temperature: f64) -> f64 {
    temperature * 9.0 / 5.0 + 32.0
}

fn format_temperature(value: f64, unit: char) -> String {
    format!("{}{} {}", value, DEGREE_SYMBOL, unit)
}

// Fibonacci
fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
