fn main() {
    moving_and_cloning();
    references_and_borrowing();
}

fn moving_and_cloning() {
    let first = String::from("Ferris");
    let full = add_suffix(first.clone());
    println!("{full}");
    // println!("{first}"); / illegal because first has moved
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn references_and_borrowing() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    // let (m1_again, m2_again) = greet(m1, m2);
    // let s1 = format!("{} {}", m1, m2); // Error: m1 and m2 are moved
    let (m1_again, m2_again) = greet(m1, m2);
    let _s1 = format!("{} {}", m1_again, m2_again); // This works because we move the (g1, g2) back into this function scope with a return value.

    // However, there is a less verbose way to do this via references
    let m3 = String::from("Hello");
    let m4 = String::from("world");
    greet_with_references(&m3, &m4);
    let s2 = format!("{} {}", m3, m4); // Error: m1 and m2 are moved
    println!("{s2}");
    // format! and println! do not move variables. If I had to guess, this is because they are macros which
    // are intentionally designed to prevent the variables from being moved, or pass ownership back to the variables
    // when they are done.
    // My intuition was correct
    println!("{m3} {m4}"); // format! does not move these variables
}

fn greet(g1: String, g2: String) -> (String, String) {
    println!("{} {}!", g1, g2);
    (g1, g2)
}

fn greet_with_references(g1: &String, g2: &String) {
    // g1 and g2 point to m3 and m4 which still own the Strings in heap
    println!("{} {}!", g1, g2);
}

fn dereferencing_pointers() {
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;         // *x reads the heap value, so a = 1
    *x += 1;                 // *x on the left-side modifies the heap value,
                            //     so x points to the value 2

    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    let b: i32 = **r1;       // two dereferences get us to the heap value

    let r2: &i32 = &*x;      // r2 points to the heap value directly
    let c: i32 = *r2;    // so only one dereference is needed to read it

    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference 'static' call with a value
    let x_abs2 = x.abs();      // implicit dereference 'instance' call on the value
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs();       // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len();      // implicit reference
    assert_eq!(s_len1, s_len2);
}
