#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(unused_variables)]

mod _crate;
mod _struct;
mod closure;
mod collections;
mod enumerate;
mod error_handling;
mod fearless_concurrency;
mod general_concept;
mod matches;
mod oop;
mod ownership;
mod smart_pointer;

fn main() {
    // closure::run();
    // _crate::run();
    // smart_pointer::run();
    // fearless_concurrency::run();
    // oop::run();
    matches::run();
}
