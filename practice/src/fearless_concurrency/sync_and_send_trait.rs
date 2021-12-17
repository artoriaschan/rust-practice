// Send trait
// 只有实现了 Send trait 的类型才可以安全地在线程间转移所有权。
// 除了Rc<T>等极少数的类型，几乎所有的Rust类型都实现了Send trait

// Sync trait
// 只有实现了 Sync trait 的类型才可以安全地被多个线程引用。
// 所有原生类型都满足Sync约束，而完全由满足Sync的类型组成的复合类型也都会被自动识别为满足Sync的类型。
// 智能指针Rc<T>同样不满足Sync约束，其原因与它不满足Send约束类似。
