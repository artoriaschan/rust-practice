#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(unused_variables)]

mod _crate;
mod _macro;
mod _struct;
mod advanced_features;
mod closure;
mod collections;
mod enumerate;
mod error_handling;
mod fearless_concurrency;
mod general_concept;
mod generics_trait;
mod lifecycle;
mod matches;
mod oop;
mod ownership;
mod self_ref;
mod smart_pointer;

fn main() {
    println!("");
    println!("=============practice=============");
    println!("");
    // closure::run();
    // _crate::run();
    // smart_pointer::run();
    // fearless_concurrency::run();
    // oop::run();
    // matches::run();
    // advanced_features::run();
    // _macro::run();
    // self_ref::run();
    // lifecycle::run();
    generics_trait::run();
}
