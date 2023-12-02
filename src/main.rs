// #![feature(map_many_mut)]
// #![feature(get_many_mut)]

use std::time::Instant;
// #[macro_use]
// extern crate scan_fmt;

mod day01;
mod day02;

fn main() {
    let s = Instant::now();

    println!("Day 1:");
    day01::main().unwrap();
    println!("Day 2:");
    day02::main().unwrap();

    println!("Total runtime: {:.2?}", s.elapsed());
}
