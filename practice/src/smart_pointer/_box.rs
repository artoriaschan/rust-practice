use std::ops::Deref; // 显示引入Deref

pub fn demo_1() {
  let b = Box::new(5);

  println!("b = {}", b);
}

enum List {
  Cons(i32, Box<List>),
  Nil,
}

pub mod demo_2 {
  use super::List::{Cons, Nil};

  pub fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  }
}

struct MyBox<T>(T); // 元组结构体

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}
// 实现deref
// 当T: Deref<Target=U>时，允许&T转换为&U。
// 当T: DerefMut<Target=U>时，允许&mut T转换为&mut U。
// 当T: Deref<Target=U>时，允许&mut T转换为&U。
impl<T> Deref for MyBox<T> {
  type Target = T; // trait的关联类型

  fn deref(&self) -> &T {
    // 返回引用
    // 在大多数使用解引用运算符的场景下， 我们并不希望获得MyBox<T>内部值的所有权。
    &self.0
  }
}
pub mod demo_3 {
  use super::MyBox;
  fn hello(name: &str) {
    println!("hello, {}", name);
  }
  pub fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // 经历了&MyBox<String> -> &String -> &str的解引用
  }
}

struct CustomSmartPointer {
  data: String,
}
pub mod demo_4 {
  use super::CustomSmartPointer;
  impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
      println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
  }
  pub fn main() {
    let c = CustomSmartPointer {
      data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
      data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
  }
}

pub mod demo_5 {
  use super::CustomSmartPointer;
  pub fn main() {
    let c = CustomSmartPointer {
      data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // std::mem::drop 已包含在预导入模块中
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
  }
}
#[cfg(test)]
mod tests {
  use super::MyBox;
  #[test]
  fn normal_deref() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
  }
  #[test]
  fn box_deref() {
    let x = 5;
    let y = Box::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);
  }
  #[test]
  fn mybox_deref() {
    let x = 5;
    let y = MyBox::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y); // *y 等价于 *(y.deref())
  }
}
