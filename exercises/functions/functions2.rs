// functions2.rs
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a hint.

extern crate core;

// use core::num::dec2flt::number;

fn main() {
    call_me(3);
}

fn call_me(num:i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
