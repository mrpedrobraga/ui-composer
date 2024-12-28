#![allow(unused)]

fn main() {}

fn foo<F>(mut f: F)
where
    F: FnMut(),
{
    f();
    f();
}

#[derive(Debug)]
struct NonCopy<T>(T);
