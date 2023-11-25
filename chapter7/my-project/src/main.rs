// This src/main.rs project follows rust convention that main is treated as the root of a binary package.
// 'cargo new' also created a Cargo.toml indicating that it created a package. Likewise, a library crate
// will have a file named src/lib.rs. Both of these crates would have the same name as the package. To create a 
// package with multiple binaries, you can place the crate roots in src/bin directory.
//
// Additional source files can be included in src for modules that the crates may depend on. Paths are used 
// to name things and use brings paths into scope. The pub keyword makes the names public. 
//
// Modules
// The mod keyword is used to declare a module within the crate root. The compiler then looks for the module 
// definition in the root file inline via curly brackets following the mod <name> declaration, src/<name>.rs, 
// and src/name/mod.rs, in that order. For example, when declaring a mod garden, the compiler first looks for mod garden {...},
// then src/garden.rs, and finally, /src/garden/mod.rs.
//
// Submodules can be declared in files other than the crate root. The compiler looks for an inline definition and 
// then for files in the src/<mod_name>/<sub_mod>.rs and src/<mod_name>/<sub_mod>/mod.rs, so when declaring mod vegetable in garden.rs
// the compiler looks inline, in src/garden/vegetable.rs, and in src/garden/vegetable/mod.rs.
//
// Once code is written in a submodule, that code can be accessed from anywhere in the crate (assuming the privacy control allows it) with
// a path like crate::garden::vegetables::Asparagus. Code within a module is private from parents by default. To make it 
// visible to parents, declare it with the pub keyword like pub mod vegetable. Then you make members of vegetable public 
// with the pub keyword at declaration.
//
// The use keyword creates shortcuts to reduce the use of long path names, so use crate::garden::vegetables::Asparagus allows us to refer
// to Asparagus as Asparagus. 
//
// Modules allow us to group related code and organize it in such a way that it is more maintainable. It also allows us to control
// the visibility of code to others.

fn main() {
    println!("Hello, world!");
}
