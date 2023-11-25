// An example library organization to represent a restaurant split between front of house and
// back of house.
// Here is the module structure:
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment
// We can see the parent-child relationship formed between modules and we can also see
// that all modules are organized under an implicit module called "crate"
// https://rust-lang.github.io/api-guidelines/ provides guidelines on how to manage your api
//
// The use keyword brings modules into scope. It is idiomatic to bring a function into scope by using the parent
// while structs and enums are brought into scope directly. The exception here is when bringing two types with the same
// name into scope. The as keyword can be used to provide an alias and we can avoid having to specify the types parent module as well.
//
// Code can be re-exported with pub use which changes the external view of the code you're re-exporting which can make the code organization
// more inuitive to users.

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    // self::hosting::add_to_waitlist(); // not quite sure what this does
    // super::hosting::add_to_waitlist(); // invalid here (no parent module)
    front_of_house::hosting::add_to_waitlist(); // this fails for now because the hosting module is not made public
}
