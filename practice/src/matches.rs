// 混合使用if let、else if、else if let和else
mod demo_1 {
  pub fn main() {
    let is_tuesday = false;
    let favorite_color: Option<&str> = Some("Yellow");
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
      println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
      println!("Tuesday is green day!");
    } else if let Ok(age) = age {
      if age > 30 {
        println!("Using purple as the background color");
      } else {
        println!("Using orange as the background color");
      }
    } else {
      println!("Using blue as the background color");
    }
  }
}
// while let
mod demo_2 {
  pub fn main() {
    // let mut stack = Vec::new();
    let mut stack = vec![];

    stack.push(1);
    stack.push(2);
    stack.push(3);
    // 只要stack.pop()返回的值是Some变体，那么while let循环就会不断地进行打印
    while let Some(top) = stack.pop() {
      println!("{}", top);
    }
  }
}
// for
mod demo_3 {
  pub fn main() {
    let v = vec!['a', 'b', 'c'];
    // 使用了enumerate方法来作为迭代器的适配器，它会在每次迭代过程中生成一个包含值本身及值索引的元组。
    for (index, value) in v.iter().enumerate() {
      println!("{} is at index {}", value, index);
    }
  }
}
// 模式语法
mod demo_4 {
  struct Point {
    x: i32,
    y: i32,
  }
  enum Message {
    Hello { id: i32 },
  }
  pub fn main() {
    // 命名变量（named variable）
    let x = Some(5);
    let y = 10;
    match x {
      Some(50) => println!("Got 50"),
      // 会匹配Some变体中携带的任意值。
      // 因为我们处在match表达式创建的新作用域中，所以这里的y是一个新的变量，而不是我们在程序起始处声明的那个存储了10的y
      Some(y) => println!("Matched, y = {:?}", y),
      _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);
    // 多重模式
    let x = 1;
    match x {
      1 | 2 => println!("one or two"),
      3 => println!("three"),
      _ => println!("anything"),
    }
    // 匹配区间
    // 范围模式只被允许使用数值或char值来进行定义
    let x = 5;
    match x {
      1..=5 => println!("one through five"),
      _ => println!("something else"),
    }
    //使用char值区间
    let x = 'c';
    match x {
      'a'..='j' => println!("early ASCII letter"),
      'k'..='z' => println!("late ASCII letter"),
      _ => println!("something else"),
    }
    // 使用模式来分解值
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    // 简写
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
    // 以下画线开始的变量名可以避免触发变量未使用警告
    // 但是_x语法仍然将值绑定到了变量上
    let _y = 11;
    let origin = Point { x: 0, y: 0 };
    match origin {
      // 使用..忽略Point中除x之外的所有字段
      // 使用..必须不能出现任何歧义
      Point { x, .. } => println!("x is {}", x),
    }
    // 匹配守卫（match guard）
    let num = Some(4);
    match num {
      // 匹配守卫（match guard）是附加在match分支模式后的if条件语句
      // 我们无法通过模式表达出类似于if x < 5这样的条件，匹配守卫增强了语句中表达相关逻辑的能力。
      Some(x) if x < 5 => println!("less than five: {}", x),
      Some(x) => println!("{}", x),
      None => (),
    }
    // @绑定
    let msg = Message::Hello { id: 11 };
    match msg {
      // @运算符允许我们在测试一个值是否匹配模式的同时创建存储该值的变量。
      // 匹配模式, id在3-7之间, 并且将id的值赋给变量id_variable
      Message::Hello {
        id: id_variable @ 3..=7,
      } => {
        println!("Found an id in 3-7 range: {}", id_variable)
      }
      // 匹配模式, id在10-12之间
      Message::Hello {
        id: id_variable @ 10..=12,
      } => {
        println!("Found an id in 10-12 range: {}", id_variable)
      }
      // 解构赋值id
      Message::Hello { id } => {
        println!("Found some other id: {}", id)
      }
    }
  }
}
pub fn run() {
  println!("=============run demos of matches=============");
  demo_1::main();
  demo_2::main();
  demo_3::main();
  demo_4::main();
}
