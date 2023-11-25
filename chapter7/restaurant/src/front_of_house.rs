// We can declare and define the submodule here like this or declare it here and define it in its own
// file at src/front_of_house/hosting.rs
// pub mod hosting {
//     pub fn add_to_waitlist() {}
//     fn seat_at_table() {}
// }

pub mod hosting;

mod serving {
    fn take_order() {}
    fn server_order() {}
    fn take_payment() {}
}