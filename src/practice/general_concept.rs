pub mod data_type;
pub mod variables;

pub fn run() {
  println!("run demos of general_concept:");
  variables::demo_1();
  variables::demo_2();
  variables::demo_3();
  variables::demo_4();

  data_type::demo_1();
  data_type::demo_2();
}
