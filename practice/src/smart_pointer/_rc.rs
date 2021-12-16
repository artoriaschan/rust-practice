use std::rc::Rc;

// Rc<T>通过不可变引用使你可以在程序的不同部分之间共享只读数据。
enum List {
  Cons(i32, Rc<List>),
  Nil,
}
pub mod demo_1 {
  use super::List::{Cons, Nil};
  use super::*;

  pub fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // a.clone() 也可以, 但Rust的惯例是在此场景下使用Rc::clone，因为Rc::clone不会执行数据的深度拷贝操作
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
      let c = Cons(4, Rc::clone(&a));

      println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
  }
}