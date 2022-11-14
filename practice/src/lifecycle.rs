// 生命周期标注语法
// 在通过函数签名指定生命周期参数时，我们并没有改变传入引用或者返回引用的真实生命周期，而是告诉编译器当不满足此约束条件时，就拒绝编译通过。
mod demo_0 {
    // 结构体中的生命周期标注
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    // 生命周期约束消除，类型和生命周期之间的约束可以消除
    // // Rust 2015
    // struct Ref<'a, T: 'a> {
    //   field: &'a T
    // }
    // // Rust 2018
    // struct Ref<'a, T> {
    //   field: &'a T
    // }

    // 方法中的生命周期标注
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }
    // impl 块消除
    // impl<'a> Reader for BufReader<'a> {
    //     // methods go here
    //     // impl内部实际上没有用到'a
    // }
    // 在 impl 内部的方法中，根本就没有用到 'a，那就可以写成下面的代码形式。
    // '_ 生命周期表示 BufReader 有一个不使用的生命周期，我们可以忽略它，无需为它创建一个名称。
    // 无法忽略生命周期，因为生命周期参数也是类型的一部分
    // impl Reader for BufReader<'_> {
    //     // methods go here
    // }

    // 函数中的生命周期标注
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    fn main() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }
}
// 生命周期消除
// 1.每一个引用参数都会获得独自的生命周期
// 2.若只有一个输入生命周期(函数参数中只有一个引用类型)，那么该生命周期会被赋给所有的输出生命周期
// 3.若存在多个输入生命周期，且其中一个是 &self 或 &mut self，则 &self 的生命周期被赋给所有的输出生命周期

mod demo_1 {
    #[derive(Debug)]
    struct Foo;

    impl Foo {
        fn mutate_and_share(&mut self) -> &Self {
            &*self
        }
        // fn mutate_and_share<'a>(&'a mut self) -> &'a Self {
        //     &'a *self
        // }
        fn share(&self) {}
    }
    pub fn main() {
        let mut foo = Foo;
        // 'b: {
        //   let mut foo: Foo = Foo;
        //   'c: {
        //       let loan: &'c Foo = Foo::mutate_and_share::<'c>(&'c mut foo);
        //       'd: {
        //           Foo::share::<'d>(&'d foo);
        //       }
        //       println!("{:?}", loan);
        //   }
        // }
        // 还记得生命周期消除规则中的第三条吗？因为该规则，导致了 mutate_and_share 方法中，参数 &mut self 和返回值 &self 的生命周期是相同的
        // 因此，若返回值的生命周期在 main 函数有效，那 &mut self 的借用也是在 main 函数有效
        let loan = foo.mutate_and_share();
        // 违背了可变借用与不可变借用不能同时存在的规则，最终导致了编译错误。
        // foo.share();
        println!("{:?}", loan);
    }
}

mod demo_2 {

    pub fn main() {
        use std::collections::HashMap;
        use std::hash::Hash;
        fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
        where
            K: Clone + Eq + Hash,
            V: Default,
        {
            // 对于map的可变借用生命周期判断失败，导致编译报错
            // match map.get_mut(&key) {
            //     Some(value) => value,
            //     None => {
            //         map.insert(key.clone(), V::default());
            //         map.get_mut(&key).unwrap()
            //     }
            // }
            // 使用裸指针实现
            unsafe {
                let map_ptr: *mut HashMap<K, V> = map;
                match (*map_ptr).get_mut(&key) {
                    Some(value) => value,
                    None => {
                        (*map_ptr).insert(key.clone(), V::default());
                        (*map_ptr).get_mut(&key).unwrap()
                    }
                }
            }
        }
    }
}

mod demo_3 {
    // 'a: 'b => 'a的生命周期不小于'b的生命周期
    struct DoubleRef<'a, 'b: 'a, T> {
        r: &'a T,
        s: &'b T,
    }
    // T: 'a => 类型T的生命周期不小于‘a的生命周期
    // struct Ref<'a, T: 'a> {
    //     r: &'a T,
    // }
    // 1.30版本后能自动推导，不用显式的声明
    struct Ref<'a, T> {
        r: &'a T,
    }

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a: 'b, 'b> ImportantExcerpt<'a> {
        fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
}

// NLL (Non-Lexical Lifetime)
// 引用的生命周期从借用处开始，一直持续到最后一次使用的地方。

// example
// let mut u = 0i32;
// let mut v = 1i32;
// let mut w = 2i32;

// // lifetime of `a` = α ∪ β ∪ γ
// let mut a = &mut u;     // --+ α. lifetime of `&mut u`  --+ lexical "lifetime" of `&mut u`,`&mut u`, `&mut w` and `a`
// use(a);                 //   |                            |
// *a = 3; // <-----------------+                            |
// ...                     //                                |
// a = &mut v;             // --+ β. lifetime of `&mut v`    |
// use(a);                 //   |                            |
// *a = 4; // <-----------------+                            |
// ...                     //                                |
// a = &mut w;             // --+ γ. lifetime of `&mut w`    |
// use(a);                 //   |                            |
// *a = 5; // <-----------------+ <--------------------------+
mod demo_4 {
    fn main() {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        println!("{} and {}", r1, r2);
        // 新编译器中，r1,r2作用域在这里结束

        let r3 = &mut s;
        println!("{}", r3);
    }
}

// Reborrow 再借用
// 在NLL的基础上实现对于变量的再借用
mod demo_5 {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn move_to(&mut self, x: i32, y: i32) {
            self.x = x;
            self.y = y;
        }
    }

    pub fn main() {
        let mut p = Point { x: 0, y: 0 };
        let r = &mut p;
        // 对于再借用而言，rr 再借用时不会破坏借用规则，但是你不能在它的生命周期内再使用原来的借用
        let rr = &*r;
        // 再借用`rr`最后一次使用发生在这里，在它的生命周期中，我们并没有使用原来的借用`r`，因此不会报错
        println!("{:?}", rr);
        // 再借用结束后，才去使用原来的借用`r`
        r.move_to(10, 10);
        println!("{:?}", r);
    }
}

pub fn run() {
    println!("=============run demos of lifecycle=============");
    // demo_1::main();
    demo_2::main();
}
