#![crate_name = "unnecessary_mut_params_example"]
#![feature(phase)]

#[phase(plugin)] extern crate unnecessary_mut_params_lint;

struct Adder {
    x: i32,
    y: i32
}

impl Adder {
    fn add(&self)            -> i32 { self.x + self.y }
    fn add_linted(&mut self) -> i32 { self.x + self.y }
    fn add_mut(&mut self)    -> i32 { self.x += self.y; self.x }
}

fn add(a: &mut u32, b: &u32) -> u32 { *a + *b }

fn main() {
    let mut adder = Adder {x: 1, y: 2};

    println!("Result of add is {}", adder.add());
    println!("Result of add_linted is {}", adder.add_linted());
    println!("Result of add_mut is {}", adder.add_mut());

    let (ref mut x, ref mut y) = (3u32, 2u32);
    println!("Adding 3 and 2 = {}", add(x, y));
}