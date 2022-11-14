// 泛型
// 在 Rust 中泛型是零成本的抽象，意味着你在使用泛型时，完全不用担心性能上的问题。
// 但是Rust 是在编译期为泛型对应的多个类型，生成各自的代码，因此损失了编译速度和增大了最终生成文件的大小。 -- 以空间换取性能
// 具体来说 Rust 通过在编译时进行泛型代码的 单态化(monomorphization)来保证效率。
mod demo_0 {
    // 函数中增加泛型
    // T: std::ops::Add<Output = T>为特征约束
    fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }
    // 枚举中增加泛型
    enum Option<T> {
        Some(T),
        None,
    }
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    // 结构体中增加泛型
    struct Point<T, U> {
        x: T,
        y: U,
    }
    // 方法中增加泛型
    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    // 特征中增加泛型
    // trait Add<RHS = Self> {
    //     type Output;

    //     fn add(self, rhs: RHS) -> Self::Output;
    // }

    // const 泛型
    // const 泛型，也就是针对值的泛型
    fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("{:?}", arr);
    }
    pub fn main() {
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

        let arr: [i32; 3] = [1, 2, 3];
        display_array(arr);

        let arr: [i32; 2] = [1, 2];
        display_array(arr);
    }
}

// 特征 trait
mod demo_1 {
    // 定义trait
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    // 通过 derive 派生特征
    #[derive(Debug)]
    pub struct Post {
        pub title: String,   // 标题
        pub author: String,  // 作者
        pub content: String, // 内容
    }
    // 为结构体实现trait
    // 关于特征实现与定义的位置，有一条非常重要的原则
    // 如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的！
    // 该规则被称为孤儿规则
    // 应对孤儿规则，可以使用`newtype`方式来为外部类型实现外部特征
    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("文章《{}》, 作者是{}", self.title, self.author)
        }
    }

    pub struct Weibo {
        pub username: String,
        pub content: String,
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{}发表了微博: {}", self.username, self.content)
        }
    }

    // 使用特征作为函数参数
    // 语法：impl Trait
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    // 特征约束（trait bound）
    pub fn notify1<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    // 多重约束
    // pub fn notify(item: &(impl Summary + Display)) {}
    // pub fn notify<T: Summary + Display>(item: &T) {}

    // where约束
    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
    // fn some_function<T, U>(t: &T, u: &U) -> i32
    // where
    //     T: Display + Clone,
    //     U: Clone + Debug,
    // {
    // }

    // 函数返回中的 impl Trait
    // 这种返回值方式有一个很大的限制：只能有一个具体的类型
    // 可以使用 `特征对象` 来返回不同的类型，或者 `枚举类型`
    fn returns_summarizable() -> impl Summary {
        Weibo {
            username: String::from("sunface"),
            content: String::from("m1 max太厉害了，电脑再也不会卡"),
        }
    }

    // 调用方法需要引入特征
    // 如果你要使用一个特征的方法，那么你需要将该特征引入当前的作用域中
    use std::convert::TryInto;
    fn try_compare() {
        let a: i32 = 10;
        let b: u16 = 100;

        let b_ = b.try_into().unwrap();

        if a < b_ {
            println!("Ten is less than one hundred.");
        }
    }

    pub fn main() {
        let post = Post {
            title: "Rust语言简介".to_string(),
            author: "Sunface".to_string(),
            content: "Rust棒极了!".to_string(),
        };
        let weibo = Weibo {
            username: "sunface".to_string(),
            content: "好像微博没Tweet好用".to_string(),
        };

        println!("{}", post.summarize());
        println!("{}", weibo.summarize());

        notify(&post);
        notify1(&weibo);

        let weibo1 = returns_summarizable();
        println!("{}", weibo1.summarize());

        try_compare();
    }
}

// 特征对象
// 泛型是在编译期完成处理的：编译器会为每一个泛型参数对应的具体类型生成一份代码，这种方式是 `静态分发(static dispatch)` ，因为是在编译期完成的，对于运行期性能完全没有任何影响。
// 与静态分发相对应的是 `动态分发(dynamic dispatch)` ，在这种情况下，直到运行时，才能确定需要调用什么方法。之前代码中的关键字 dyn 正是在强调这一“动态”的特点。
mod demo_2 {
    pub trait Draw {
        fn draw(&self);
    }

    #[derive(Debug)]
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("drew a Button! describe: {:?}", self);
        }
    }

    #[derive(Debug)]
    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!("drew a SelectBox! describe: {:?}", self);
        }
    }

    pub struct Screen {
        // 如果使用泛型+特征约束的话，只能限制在相同类型的集合
        // 如果只需要同质（相同类型）集合，更倾向于这种写法：使用泛型和 特征约束，因为实现更清晰，且性能更好(特征对象，需要在运行时从 vtable 动态查找需要调用的方法)。

        // 这里对于不同类型的集合，则是需要使用特征对象。
        // 特征对象指向实现了 Draw 特征的类型的实例，也就是指向了 Button 或者 SelectBox 的实例，这种映射关系是存储在一张表中，可以在运行时通过特征对象找到具体调用的类型方法。
        // 可以通过 & 引用或者 Box<T> 智能指针的方式来创建特征对象。

        // 虽然特征对象没有固定大小，但它的引用类型的大小是固定的，它由两个指针组成（ptr 和 vptr），因此占用两个指针大小。
        // ptr 指向实现了特征的具体类型的实例
        // vptr 指向一个虚表 vtable，vtable 中保存了实现该特征的不同类型的实例对于可以调用的实现于特征的方法。

        // ptr所指向的实例只能调用实现于特征的相应方法，而不能调用类型本身实现的方法和类型实现于其他特征的方法。
        pub components: Vec<Box<dyn Draw>>,
        // pub components: Vec<&dyn Draw>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    // 特征对象的限制
    // 不是所有特征都能拥有特征对象，只有对象安全的特征才行。
    // 当一个特征的所有方法都有如下属性时，它的对象才是安全的：
    //  1.方法的返回类型不能是 Self
    //      self：指代当前的实例对象
    //      Self：指代特征或者方法类型的别名
    //  2.方法没有任何泛型参数
    // 对象安全对于特征对象是必须的，因为一旦有了特征对象，就不再需要知道实现该特征的具体类型是什么了。如果特征方法返回了具体的 Self 类型，但是特征对象忘记了其真正的类型，那这个 Self 就非常尴尬，因为没人知道它是谁了。
    // 但是对于泛型类型参数来说，当使用特征时其会放入具体的类型参数：此具体类型变成了实现该特征的类型的一部分。而当使用特征对象时其具体类型被抹去了，故而无从得知放入泛型参数类型到底是什么。

    pub fn main() {
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ],
        };

        screen.run();
    }
}

mod demo_3 {
    // 关联类型 => 增加代码可读性
    // pub trait Iterator {
    //     type Item;   // 单独声明类型

    //     fn next(&mut self) -> Option<Self::Item>;    // 在该处使用定义的类型
    // }
    // 关联类型的具体使用
    // impl Iterator for Counter {
    //     type Item = u32; // 生命具体的类型

    //     fn next(&mut self) -> Option<Self::Item> {
    //         // --snip--
    //     }
    // }

    // 默认泛型类型参数
    // trait Add<RHS=Self> {
    //     type Output;

    //     fn add(self, rhs: RHS) -> Self::Output;
    // }

    use std::ops::Add;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    fn main1() {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
    }
    // 默认类型参数主要用于两个方面：
    //     1.减少实现的样板代码
    //     2.扩展类型但是无需大幅修改现有的代码

    // 调用同名的方法
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

    // 特征定义中的特征约束
    use std::fmt::Display;

    trait OutlinePrint: Display {
        // OutlinePrint: Display 表示如果你想要实现 OutlinePrint 特征，首先你需要实现 Display 特征。
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

    pub fn main() {
        // 1.优先调用类型上的方法
        // 2.方法有self参数时，显式调用具体特征上的方法：Trait::method(&self)
        // 3.方法没有self参数（关联函数）时，使用完全限定语法调用：<Type as Trait>::function(receiver_if_method, next_arg, ...);
        let person = Human;
        Pilot::fly(&person); // 调用Pilot特征上的方法
        Wizard::fly(&person); // 调用Wizard特征上的方法
        person.fly(); // 调用Human类型自身的方法

        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    }
}

pub fn run() {
    println!("=============run demos of generics=============");
    // demo_0::main();
    // demo_1::main();
    // demo_2::main();
    demo_3::main();
}
