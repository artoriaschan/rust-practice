mod _box;
mod _rc;
mod _refcell;

pub fn run() {
  println!("=============run demos of smart_pointer=============");
  _box::demo_1();
  _box::demo_2::main();
  _box::demo_3::main();
  _box::demo_4::main();
  _box::demo_5::main();

  _rc::demo_1::main();

  _refcell::demo_1::main();
  _refcell::demo_2::main();
  _refcell::demo_3::main();
}
