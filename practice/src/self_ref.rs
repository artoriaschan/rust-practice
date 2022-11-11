// 使用 Option
#[derive(Debug)]
struct WhatAboutThis<'a> {
    name: String,
    nickname: Option<&'a str>,
}

fn run_demo1() {
    let mut tricky = WhatAboutThis {
        name: "Annabelle".to_string(),
        nickname: None,
    };
    tricky.nickname = Some(&tricky.name[..]);

    println!("{:?}", tricky);
}

// unsafe实现，直接使用裸指针
#[derive(Debug)]
struct StringWrapper {
    value: String,
    // 直接使用裸指针
    pointer_to_value: *const String,
    // 还能通过裸指针来修改 String，但是需要将 *const 修改为 *mut
    // pointer_to_value: *mut String,
}

impl StringWrapper {
    fn new(txt: &str) -> Self {
        StringWrapper {
            value: txt.to_string(),
            pointer_to_value: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.value;
        self.pointer_to_value = self_ref;
    }

    fn value(&self) -> &str {
        &self.value
    }

    fn pointer_to_value(&self) -> &String {
        assert!(
            !self.pointer_to_value.is_null(),
            "Test::b called without Test::init being called first"
        );
        // 裸指针解引用在引用
        unsafe { &*(self.pointer_to_value) }
    }
}
fn run_demo2() {
    let mut t = StringWrapper::new("hello");
    t.init();
    println!("{}, {:p}", t.value(), t.pointer_to_value());

    // t.value.push_str(", world");
    // unsafe {
    //     (&mut *t.pointer_to_value).push_str("!");
    // }
    // println!("{}, {:p}", t.value(), t.pointer_to_value());
}

fn main() {}

pub fn run() {
    run_demo1();
    run_demo2();
}
