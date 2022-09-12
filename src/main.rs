/*
 * File: main.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use std::ops::Deref;

fn main() {
    dereference_operator_simple();
    dereference_operator_box();
    dereference_operator_my_box();
    call_implicit_deref_conversion();
}

fn call_implicit_deref_conversion() {
    // deref conversion will convert type to correct type; can also be used for mut -> immutable conversion
    let name = MyBox::new(String::from("World"));
    let mut second_name = MyBox::new(String::from("Mutable World"));
    implicit_deref_conversion(&second_name);
    second_name = MyBox::new(String::from("Immutable World"));
    implicit_deref_conversion(&second_name);
    implicit_deref_conversion(&name);
}

// deref conversion will convert to the type to a different type
fn implicit_deref_conversion(name: &str) {
    println!("Hello, {}!", name);
}

// simplified Box<>
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    // If deref would return a value, not a reference, Rust would move ownership
    fn deref(&self) -> &T {
        &self.0
    }
}

fn dereference_operator_my_box() {
    let x = 5;
    // This time y is pointing to a copy of x
    let y = MyBox::new(x);
    assert_eq!(5, x);
    // Follow memory address to x in this case; dereference Box::new(x)
    assert_eq!(5, *y);
    // This is the same
    assert_eq!(5, *y.deref());
}

fn dereference_operator_box() {
    let x = 5;
    // This time y is pointing to a copy of x
    let y = Box::new(x);
    assert_eq!(5, x);
    // Follow memory address to x in this case; dereference Box::new(x)
    assert_eq!(5, *y);
}

fn dereference_operator_simple() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    // Follow memory address to x in this case; dereference &x
    assert_eq!(5, *y);
    //assert_eq!(5, y); // ERROR: can't compare `{integer}` with `&{integer}`
}
