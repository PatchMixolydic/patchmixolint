#![allow(unused)]
#![feature(decl_macro)]

macro_rules! foo {
    () => {};
}

struct Foo<'a>(std::marker::PhantomData<&'a ()>);

impl<'a> Foo<'a> {}

fn main() {}
