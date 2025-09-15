// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

// I AM OT DONE

#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

mod macros {
    pub use crate::my_macro;
}

fn main() {
    macros::my_macro!();
}
