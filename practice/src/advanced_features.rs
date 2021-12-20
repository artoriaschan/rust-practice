mod advanced_trait;
mod unsafe_rust;

pub fn run() {
  println!("=============run demos of advanced features=============");
  unsafe_rust::demo_1::main();
  unsafe_rust::demo_2::main();
  unsafe_rust::demo_3::main();
  unsafe_rust::demo_4::main();

  advanced_trait::demo_1::main();
  advanced_trait::demo_2::main();
  advanced_trait::demo_3::main();
  advanced_trait::demo_4::main();
}
