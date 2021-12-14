mod _self;
mod references;
mod slice;

pub fn run() {
  println!("=============run demos of ownership=============");
  _self::demo_1();
  _self::demo_2();
  _self::demo_3();

  references::demo_1();
  references::demo_2();

  slice::demo_1();
  slice::demo_2();
  slice::demo_3();
}
