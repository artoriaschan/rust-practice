// 关联类型（associated type）
// 关联类型（associated type）是trait中的类型占位符，它可以被用于trait的方法签名中。
// pub trait Iterator {
//   type Item;
//   fn next(&mut self) -> Option<Self::Item>;
// }
// 语法似乎和泛型语法差不多，那么我们为什么不直接使用泛型来定义Iterator trait呢？

// 如果使用泛型实现, 需要在每次实现该trait的过程中标注类型；
// 当我们在Counter上使用next方法时，也必须提供类型标注来指明想要使用的Iterator实现。

// 默认泛型参数
pub mod demo_1 {
  use std::ops::Add;
  // RHS=Self就是所谓的默认类型参数（default type parameter）。
  // 泛型参数RHS（也就是 “right-handle side”的缩写）定义了add方法中rhs参数的类型。
  // trait Add<RHS = Self> {
  //   type Output;
  //   fn add(self, rhs: RHS) -> Self::Output;
  // }
  #[derive(Debug, PartialEq)]
  struct Point {
    x: i32,
    y: i32,
  }
  #[derive(Debug, PartialEq)]
  struct Millimeters(u32);
  #[derive(Debug, PartialEq)]
  struct Meters(u32);

  impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
      Point {
        x: self.x + other.x,
        y: self.y + other.y,
      }
    }
  }
  // 为了将Millimeters和Meters的值加起来，我们指定impl Add<Meters>来设置RHS类型参数的值，而没有使用默认的Self。
  impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
      Millimeters(self.0 + (other.0 * 1000))
    }
  }

  pub fn main() {
    let a = Point { x: 1, y: 0 };
    let b = Point { x: 2, y: 3 };

    println!("a = {:?}", a);
    println!("b = {:?}", b);

    let result = a + b;
    assert_eq!(result, Point { x: 3, y: 3 });
    println!("a + b = {:?}", result);

    let a = Millimeters(1001000);
    let b = Meters(1);

    let result = a + b;
    println!("a + b = {:?}", result.0);
  }
}
// 完全限定语法
pub mod demo_2 {
  trait Pilot {
    fn fly(&self);
  }

  trait Wizard {
    fn fly(&self);
  }

  struct Human;
  impl Pilot for Human {
    fn fly(&self) {
      println!("This is your captain speaking.");
    }
  }

  impl Wizard for Human {
    fn fly(&self) {
      println!("Up!");
    }
  }

  impl Human {
    fn fly(&self) {
      println!("*waving arms furiously*");
    }
  }
  trait Animal {
    fn baby_name() -> String;
  }

  struct Dog;

  impl Dog {
    fn baby_name() -> String {
      String::from("Spot")
    }
  }
  impl Animal for Dog {
    fn baby_name() -> String {
      String::from("puppy")
    }
  }
  pub fn main() {
    let person = Human;
    // 使用显式的语法来指定具体的fly方法
    // 对于fly等需要接收 self作为参数的方法，Rust可以自动地根据self的类型推导出具体的trait实现。
    Pilot::fly(&person);
    Wizard::fly(&person);
    // Rust默认调用了直接实现在Human类型上的fly方法
    person.fly();
    // Animal trait中有与struct Dog中同名的关联函数baby_name
    println!("A baby dog is called a {}", Dog::baby_name());
    // 因为没有self参数的关联函数，所以Rust无法推断出我们想要调用哪一个Animal::baby_name的实现。
    // 这段代码将会报错
    // println!("A baby dog is called a {}", Animal::baby_name());
    // 使用完全限定语法来调用Dog为Animal trait实现的 baby_name函数
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
  }
}
// 超 trait（supertrait）
pub mod demo_3 {
  use std::fmt;
  // 想要在outline_print的默认实现中使用Display trait的功能
  // 所以OutlinePrint trait必须注明自己只能用于那些提供了Display功能的类型。
  // 在定义trait时指定OutlinePrint: Display来完成该声明
  trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
      let output = self.to_string();
      let len = output.len();
      println!("{}", "*".repeat(len + 4));
      println!("*{}*", " ".repeat(len + 2));
      println!("* {} *", output);
      println!("*{}*", " ".repeat(len + 2));
      println!("{}", "*".repeat(len + 4));
    }
  }
  struct Point {
    x: i32,
    y: i32,
  }
  impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "({}, {})", self.x, self.y)
    }
  }
  impl OutlinePrint for Point {}

  pub fn main() {
    let p = Point { x: 1, y: 2 };

    p.outline_print();
  }
}
// 孤儿规则：只有当类型和对应trait中的任意一个定义在本地包内时，我们才能够为该类型实现这一trait。
// 使用newtype模式来巧妙地绕过这个限制，它会利用元组结构体创建出一个新的类型。

pub mod demo_4 {
  use std::fmt;
  use std::ops::Deref;
  // newtype
  // 因为Wrapper是一个新的类型，所以它没有自己内部值的方法。
  // 为了让Wrapper的行为与Vec<T>完全一致，我们需要在Wrapper中实现所有Vec<T>的方法，并将这些方法委托给self.0。
  struct Wrapper<T = String>(Vec<T>);
  impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "[{}]", self.0.join(", "))
    }
  }
  // 假如我们希望新类型具有内部类型的所有方法，那么我们也可以为Wrapper实现Deref trait来直接返回内部的类型。
  // 假如我们不希望Wrapper类型具有内部类型的所有方法，我们就只能手动实现需要的那部分方法了。
  impl<T> Deref for Wrapper<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
      // 返回引用
      &self.0
    }
  }
  pub fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    //
    for (index, value) in w.iter().enumerate() {
      println!("word{} is {}", index, value);
    }
  }
}
