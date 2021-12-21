// 函数指针（function pointer）
// 与闭包在函数中传递相似，你可以将普通函数传递至其他函数
// 函数会在传递的过程中被强制转换成fn类型，也就是所谓的函数指针

// 函数指针实现了全部3种闭包trait（ Fn、FnMut以及FnOnce），所以我们总是可以把函数指针用作参数传递给一个接收闭包的函数。
pub mod demo_1 {
  fn add_one(x: i32) -> i32 {
    x + 1
  }

  fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
  }

  pub fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is : {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    // map方法传入闭包当做参数
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("list = {:?}", list_of_strings);
    let list_of_numbers = vec![1, 2, 3];
    // map方法传入函数当做参数
    // ToString::to_string 完全限定语法
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("list = {:?}", list_of_strings);
    #[derive(Debug)]
    enum Status {
      Value(u32),
      Stop,
    }
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("list = {:?}", list_of_statuses);
  }
}
// 返回闭包
pub mod demo_2 {
  // 在大多数希望返回trait的情形下，你可以将一个实现了该trait的具体类型作为函数的返回值。
  // 但你无法对闭包执行同样的操作，因为闭包没有一个可供返回的具体类型。
  // Rust无法推断出自己需要多大的空间来存储此处返回的闭包。但是可以使用trait对象解决这一问题
  fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
  }
}
