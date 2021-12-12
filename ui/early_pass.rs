#![allow(unused)]
#![feature(decl_macro)]

#[allow(unused_macros)]
macro_rules! foo {
    () => {};
}

struct Foo<'a>(std::marker::PhantomData<&'a ()>);

impl<'a> Foo<'a> {}

fn main() {}
