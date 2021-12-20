// 不安全Rust允许你执行4种在安全Rust中不被允许的操作，而它们也就是所谓的不安全超能力（unsafe superpower）。
// • 解引用裸指针。
// • 调用不安全的函数或方法。
// • 访问或修改可变的静态变量。
// • 实现不安全trait。
// unsafe关键字并不会关闭借用检查器或禁用任何其他Rust安全检查。
// unsafe关键字仅仅让你可以访问这4种不会被编译器进行内存安全检查的特性。

// 裸指针（raw pointer）
pub mod demo_1 {
  pub fn main() {
    let mut num = 5;
    // 你可以在安全代码内合法地创建裸指针，但不能在不安全代码块外解引用裸指针。
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
      println!("r1 = {}", *r1);
      *r2 = 10;
      println!("r2 = {}", *r2);
    }
  }
}
// 不安全函数（unsafe function）
pub mod demo_2 {
  use std::slice;
  // 此处的unsafe关键字意味着我们需要在调用该函数时手动满足并维护一些先决条件
  // 不安全函数的函数体也是unsafe代码块
  unsafe fn dangerous() {}

  fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
      (
        slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
      )
    }
  }
  pub fn main() {
    // 在单独的unsafe代码块中调用dangerous函数。
    unsafe {
      dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
  }
}
// 外部函数接口（Foreign Function Interface，FFI）
pub mod demo_3 {
  // Rust为此提供了extern关键字来简化创建和使用外部函数接口（Foreign Function Interface，FFI）的过程。
  // 这段代码在extern "C"块中列出了我们想要调用的外部函数名称及签名，
  // 其中的"C"指明了外部函数使用的应用二进制接口（Application Binary Interface，ABI）：它被用来定义函数在汇编层面的调用方式。
  extern "C" {
    fn abs(input: i32) -> i32;
  }

  pub fn main() {
    unsafe {
      println!("Absolute value of -3 according to C: {}", abs(-3));
    }
  }
}
// 全局变量（global variable）
pub mod demo_4 {
  // 静态变量的名称会约定俗成地被写作SCREAMING_SNAKE_CASE的形式，并且必须要标注变量的类型
  static HELLO_WORLD: &str = "Hello, world!";
  // 静态变量是可变的。需要注意的是，访问和修改可变的静态变量是不安全的。
  // 使用mut关键字来指定静态变量的可变性
  static mut COUNTER: u32 = 0;
  fn add_to_count(inc: u32) {
    unsafe {
      COUNTER += inc;
    }
  }
  pub fn main() {
    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
      println!("COUNTER = {}", COUNTER);
    }
  }
}
// unsafe trait
// 当某个trait中存在至少一个方法拥有编译器无法校验的不安全因素时，我们就称这个trait是不安全的
// 你可以在trait定义的前面加上unsafe关键字来声明一个不安全trait，同时该trait也只能在unsafe代码块中实现
pub mod demo_5 {
  unsafe trait Foo {
    // 某些方法
  }

  unsafe impl Foo for i32 {
    // 对应的方法实现
  }
}
