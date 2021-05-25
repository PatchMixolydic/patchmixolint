#![allow(unused)]
#![allow(patchmixolint::missing_lints)]
#![feature(register_tool)]
#![register_tool(patchmixolint)]

#[derive(PartialEq)]
struct MyFloat {
    x: f32,
    y: f64,
}

impl Eq for MyFloat {}

#[derive(PartialEq)]
enum MyFloatEnum {
    X(f32),
    Y(MyFloat),
}

impl Eq for MyFloatEnum {}

fn main() {}
