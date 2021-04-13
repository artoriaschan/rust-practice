mod map;
mod string;
mod vector;

pub fn run() {
  println!("=============run demos of collections=============");
  vector::demo_1();
  vector::demo_2();
  vector::demo_3();

  string::demo_1();
  string::demo_2();
  string::demo_3();

  map::demo_1();
  map::demo_2();
  map::demo_3();
}
