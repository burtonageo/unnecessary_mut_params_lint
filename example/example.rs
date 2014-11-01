#![crate_name = "unnecessary_mut_params_example"]
#![feature(phase)]

#[phase(plugin)] extern crate unnecessary_mut_params_lint;

struct Adder {
    x: i32,
    y: i32
}

impl Adder {
    fn add(&mut self) -> i32 { self.x + self.y }
}

fn main() {
    let mut adder = Adder {x: 1, y: 2};
    println!("Result is {}", adder.add());
}