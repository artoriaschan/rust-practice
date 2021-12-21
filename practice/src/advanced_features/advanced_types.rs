// newtype模式
struct Millimeters(u32);
struct Meters(u32);
// 使用类型别名创建同义类型
type Kilometers = i32;

// Rust有一个名为!的特殊类型，它在类型系统中的术语为空类型（empty type）。
// 因为它没有任何的值，我们倾向于叫它never类型。
fn bar() -> ! {
  // --略
  let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    // continue的返回类型是!
    Err(_) => continue,
  };
  // panic!宏的实现同样使用了never类型。
}
// 动态大小类型（Dynamically Sized Type，DST）
// 尽管&T被视作存储了T所在内存地址的单个值，但&str实际 上是由两个值组成的：str的地址与它的长度。
// 无论&str指向了什么样的字符串，我们总是能够知道&str的大小。这就是 Rust中使用动态大小类型的通用方式：它们会附带一些额外的元数据来存储动态信息的大小。
// 每一个trait都是一 个可以通过其名称来进行引用的动态大小类型。
