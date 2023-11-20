#[derive(Debug)] // Adds the debug trait to Rectangle struct
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // dbg! takes ownership of the expression and returns it, so we can use it with assignment too
        height: 50,
    };
    // Debug also prints the file and line number where the macro is called
    dbg!(&rect1);
}
