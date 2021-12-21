// 使用macro_rules!创建声明宏 => 根据模式匹配来替换代码
#[macro_export]
macro_rules! vec {
  // 模式分支
  // $()中的$x:expr可以匹配任意的Rust表达式，并将其命名为$x。
  ($( $x:expr ),*) => {
    let mut temp_vec = Vec::new();
    // 它会为模式中匹配到的每一个$()生成$()*中对应的temp_vec.push()代码；
    // 这一展开过程会重复零次还是多次，取决于匹配成功的表达式数量。
    $(
      temp_vec.push($x);
    )*
    temp_vec
  };
  // vec![1, 2, 3]会生成如下的代码:
  // let mut temp_vec = Vec::new();
  // temp_vec.push(1);
  // temp_vec.push(2);
  // temp_vec.push(3);
  // temp_vec

  // 如果你想要学习更多有关编写宏的知识，请参考在线文档或其他资源，比如The Little Book of Rust Macros等。
}

// 过程宏 => 过程宏会接收并操作输入的Rust代码，并生成另外一些Rust代码作为结果
// 过程宏存在3种不同的类型: 自定义派生宏、属性宏及函数宏
// use proc_macro;
// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {}
mod demo_1 {
  trait HelloMacro {
    fn hello_macro();
  }
  struct Pancakes;

  impl HelloMacro for Pancakes {
    fn hello_macro() {
      println!("Hello, Macro! My name is Pancakes!");
    }
  }

  pub fn main() {
    Pancakes::hello_macro();
  }
}
// derive 宏
mod hello_macro_derive {
  use hello_macro::HelloMacro;
  use hello_macro_derive::HelloMacro;
  #[derive(HelloMacro)]
  struct Pancakes;
  pub fn main() {
    Pancakes::hello_macro();
  }
}

// 属性宏
// 例如可以在编写Web应用框架时为函数添加标记
// #[route(GET, "/")]
// fn index() {
//   // ...
// }
// 这个宏定义的函数签名如下所示：
// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
//   // ...
// }

// 函数宏
// 函数宏可以定义出类似于函数调用的宏，但它们远比普通函数更为灵活。
// 例如，与macro_rules!宏类似，函数宏也能接收未知数量的参数。
// 与macro_rules!宏不同的是，函数宏则可以接收一个TokenStream作为参数，并与另外两种过程宏一样在定义中使用Rust代码来操作TokenStream。
pub fn run() {
  println!("=============run demos of macro=============");
  // demo_1::main();
  hello_macro_derive::main();
}
